// #![allow(unused_braces,unused)]

use std::{sync::{Arc, RwLock}, fmt::Debug};

use axum::{
    extract::Request,
    response::{Response, IntoResponse as _}
};
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

use crate::config::Setting;

fn internal_server_error<T: Debug>(err: T) -> StatusCode {
    eprintln!("{:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
}

fn todo_code() -> Result<Response,StatusCode> {
    eprintln!("NOT IMPLEMENTED");
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_proxy(req: Request, setting: Arc<RwLock<Setting>>) -> Result<Response,StatusCode> {

    let host = req
        .headers()
        .get("host")
        .ok_or(StatusCode::NOT_FOUND)?
        .to_str()
        .map_err(|_|StatusCode::NOT_FOUND)?
        .to_string();


    let setting = {
        let lock = setting
            .read()
            .map_err(internal_server_error)?;
        lock
            .servers
            .get(&host)
            .ok_or(StatusCode::NOT_FOUND)?
            .clone()
    };

    let target = if let Some(proxy_setting) = setting.proxy {
        proxy_setting.target
    } else if let Some(_) = setting.static_serve {
        return todo_code()
    } else {
        return todo_code()
    };

    let stream = TcpStream::connect(target).await.map_err(internal_server_error)?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.map_err(internal_server_error)?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    return Ok(sender.send_request(req).await.map_err(internal_server_error).into_response())
}

// pub async fn post_proxy(req: Request, setting: Arc<RwLock<Setting>>) -> Response {
//     let setting = match req.headers().get("host") {
//         Some(host) => {
//             let lock = setting.read().unwrap();
//             lock.find_by_domain(host.to_str().unwrap().to_string()).unwrap().clone()
//         },
//         None => return (StatusCode::NOT_FOUND, "").into_response()
//     };
//
//     let stream = TcpStream::connect(format!("127.0.0.1:{}",setting.port)).await.unwrap();
//     let io = TokioIo::new(stream);
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
//
//     tokio::task::spawn(async move {
//         if let Err(err) = conn.await {
//             println!("Connection failed: {:?}", err);
//         }
//     });
//
//     return sender.send_request(req).await.unwrap().into_response()
//
// }
