// #![allow(unused_imports,unused_braces)]

use std::sync::{RwLock, Arc};
use axum::Router;
use tokio::signal;

use crate::client::get_proxy;
use crate::config::reload;

pub struct Setting {
    pub servers: Vec<ServerSetting>
}

impl Setting {
    pub fn find_by_port(&self, port: u16) -> Option<&ServerSetting> {
        self.servers.iter().find(|e|e.port == port)
    }
    pub fn find_by_domain<T: PartialEq<&'static str>>(&self, domain: T) -> Option<&ServerSetting> {
        self.servers.iter().find(|e|domain == e.domain)
    }
    pub fn example() -> Setting {
        Setting {
            servers: vec![
                ServerSetting {
                    port: 3000,
                    domain: "deuzo.me"
                }
            ]
        }
    }
}

#[derive(Clone)]
pub struct ServerSetting {
    pub port: u16,
    pub domain: &'static str
}

pub async fn server() {
    let setting = Arc::new(RwLock::new(Setting::example()));

    reload_signal(Arc::clone(&setting));

    let app = Router::new()
        .fallback({
            let refc = Arc::clone(&setting);
            move |req| get_proxy(req,refc)
        });
        

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",std::env::args().nth(1).unwrap())).await.unwrap();

    println!("Listening");

    axum::serve(listener, app).await.unwrap();

}

fn reload_signal(r: Arc<RwLock<Setting>>) {
    tokio::spawn(async move {
        let mut signal = signal::unix::signal(signal::unix::SignalKind::user_defined1()).unwrap();
        signal.recv().await;

        {
            let mut lock = r.write().unwrap();
            lock.servers = vec![ServerSetting { port: 443, domain: "app.js" }];
            reload();
        }

        reload_signal(r);
    });
}
