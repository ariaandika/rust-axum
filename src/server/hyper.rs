#![allow(unused_braces,unused_imports)]

use std::{pin::Pin, future::Future};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{service::Service, Request, Response};

use crate::client::client;

pub async fn hyper_server() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let io =  hyper_util::rt::TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                // .serve_connection(io, hyper::service::service_fn(hyper_hello))
                // .serve_connection(io, Svc { })
                .serve_connection(io, move |req| responser(req))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    };
}

// async fn hyper_hello(_: hyper::Request<hyper::body::Incoming>) -> Result<hyper::service::Service::Future, Infallible> {
// Ok(hyper::Response::new(Full::new(Bytes::from("Hello, World!"))))
//     todo!()
// }


type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

async fn responser(req: Request<hyper::body::Incoming>) -> Response {
    fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
        Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
    }

    let res = match req.uri().path() {
        "/" => mk_response(format!("home! counter = {:?}", "ctr")),
        "/posts" => mk_response(format!("posts, of course! counter = {:?}", "Home")),
        "/authors" => mk_response(format!(
            "authors extraordinare! counter = {:?}",
            "Nice"
        )),
        "/test" => return client().await.unwrap().into_response(),
        _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
    };

    res
}


