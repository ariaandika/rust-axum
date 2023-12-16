# Rust Axum

## TLS

problem with tls

- `axum-server`

it uses external library `axum-server`

it have problem with matching with axum version at installation

- low level `rustls`

the example shows non existing struct in newer version of `tokio-rustls`

## Static Server

todo

## Proxy

using hyper we can immediately return the response

## Basic Validation

using `Request` extractor, we have full control

## Hot Config Reload

using `Arc`, `RwLock`, and tokio signal handling, easily implemented

