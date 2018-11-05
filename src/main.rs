extern crate futures;
extern crate hyper;

use futures::{future, Stream};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::str;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn route(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POST to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/echo/upper") => {
            *response.body_mut() = Body::wrap_stream(req.into_body().map(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<_>>()
            }))
        }
        (&Method::POST, "/echo/reverse") => {
            *response.body_mut() = Body::wrap_stream(req.into_body().map(|chunk| {
                str::from_utf8(&chunk)
                    .unwrap_or("")
                    .to_string()
                    .chars()
                    .rev()
                    .collect::<String>()
            }))
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(response))
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(route))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Now listening on {}", addr);
    hyper::rt::run(server);
}
