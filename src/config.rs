use std::{process::Command, collections::HashMap};

use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Setting {
    pub servers: HashMap<String,ServerSetting>,
    pub tls: TlsSetting,
    pub port: u16
}

#[derive(Debug,Clone,Deserialize)]
pub struct ServerSetting {
    pub static_serve: Option<StaticSetting>,
    pub proxy: Option<ProxySetting>
}

#[derive(Debug,Clone,Deserialize)]
pub struct StaticSetting {
    pub target: String,
    pub path: Option<String>
}

#[derive(Debug,Clone,Deserialize)]
pub struct ProxySetting {
    pub target: String,
    pub path: Option<String>
}

#[derive(Debug,Clone,Deserialize)]
pub struct TlsSetting {
    pub dir: String,
    pub key: Option<String>,
    pub cert: Option<String>
}

impl TlsSetting {
    fn example() -> Self {
        Self { dir: "/etc/tls".into(), key: Some("key.pem".into()), cert: Some("cert.pem".into()) }
    }
}

impl ServerSetting {
    fn example() -> Self {
        Self { static_serve: None, proxy: Some(ProxySetting::example()) }
    }
}

impl ProxySetting {
    fn example() -> Self {
        Self { target: "localhost:8000".into(), path: None }
    }
}

impl Setting {
    pub fn example() -> Self {
        let mut servers = HashMap::new();
        servers.insert("deuzo.me".into(), ServerSetting::example());
        Self { port: 3000, servers, tls: TlsSetting::example() }
    }

    pub fn load() -> Result<Setting, Box<dyn std::error::Error>> {
        let handle = Command::new("bun")
            .arg("run")
            .arg("./config/config.ts")
            .output()?;

        let stdout = handle.stdout.as_slice();
        Ok(serde_json::from_slice::<Setting>(stdout)?)
    }

    pub fn reload(&mut self, new_config: Self) {
        self.servers = new_config.servers;
        self.tls = new_config.tls;
        self.port = new_config.port;
    }
}

