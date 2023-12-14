#![allow(unused_braces)]

pub mod server;
pub mod client;
pub mod config;

#[tokio::main]
async fn main() {
    server::server().await;
}


