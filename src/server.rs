use std::sync::{RwLock, Arc};
use axum::Router;
use tokio::signal;

use crate::client::get_proxy;
use crate::config::Setting;


pub async fn server() {
    let setting_data = Setting::load().expect("Cannot load config");
    let port = setting_data.port;
    let setting = Arc::new(RwLock::new(setting_data));

    reload_signal(Arc::clone(&setting));

    let app = Router::new()
        .fallback({
            println!("Req:");
            let refc = Arc::clone(&setting);
            move |req| get_proxy(req,refc)
        });

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await.unwrap();

    println!("Listening {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
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
