use std::io::{self};
use std::sync::{RwLock, Arc};
use axum::Router;
use axum::extract::Request;
use hyper::body::Incoming;
use hyper_util::rt::{TokioIo, TokioExecutor};
use tokio::signal;
use tower::Service;

use crate::client::get_proxy;
use crate::config::Setting;

pub mod tls;

pub async fn server() -> io::Result<()> {
    let setting = Setting::load().expect("Cannot load config");
    let port = setting.port;
    let setting = Arc::new(RwLock::new(setting));

    reload_signal(Arc::clone(&setting));

    let app = Router::new()
        .fallback({
            println!("Req:");
            let refc = Arc::clone(&setting);
            move |req| get_proxy(req,refc)
        });

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    let acceptor = {
        let setting = setting.read().unwrap();
        let key = setting.tls.key.as_ref().unwrap();
        let cert = setting.tls.cert.as_ref().unwrap();
        tls::load_tls(key.to_string(), cert.to_string())?
    };

    println!("Listening {}",listener.local_addr().unwrap());

    // axum::serve(listener, app).await.unwrap();
    loop {
        let tower_service = app.clone();
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();


        let fut = async move {
            let stream = acceptor.accept(stream).await?;

            let stream = TokioIo::new(stream);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().call(request)
            });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                eprintln!("error serving connection: {}", err);
            }

            Ok(()) as io::Result<()>
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{:?}", err);
            }
        });
    }
}

fn reload_signal(r: Arc<RwLock<Setting>>) {
    tokio::spawn(async move {
        match signal::unix::signal(signal::unix::SignalKind::user_defined1()) {
            Ok(mut s) => s.recv().await,
            Err(er) => return eprintln!("Err: {er}"),
        };

        let setting = Setting::load().unwrap();

        {
            let mut lock = r.write().unwrap();
            lock.reload(setting);
        }

        reload_signal(r)
    });
}

