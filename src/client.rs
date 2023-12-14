// #![allow(unused_braces,unused)]


use std::sync::{Arc, RwLock};

use axum::{
    extract::Request,
    response::{Response, IntoResponse}
};
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

use crate::server::Setting;


pub async fn get_proxy(req: Request, setting: Arc<RwLock<Setting>>) -> Response {
    let setting = match req.headers().get("host") {
        Some(host) => {
            let lock = setting.read().unwrap();
            lock.find_by_domain(host).unwrap().clone()
        },
        None => return (StatusCode::NOT_FOUND, "").into_response()
    };

    let stream = TcpStream::connect(format!("127.0.0.1:{}",setting.port)).await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    return sender.send_request(req).await.unwrap().into_response()
}

pub async fn post_proxy(req: Request, setting: Arc<RwLock<Setting>>) -> Response {
    let setting = match req.headers().get("host") {
        Some(host) => {
            let lock = setting.read().unwrap();
            lock.find_by_domain(host).unwrap().clone()
        },
        None => return (StatusCode::NOT_FOUND, "").into_response()
    };

    let stream = TcpStream::connect(format!("127.0.0.1:{}",setting.port)).await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    return sender.send_request(req).await.unwrap().into_response()

}
