#![deny(warnings)]
extern crate hyper_sync;
extern crate env_logger;

use std::io::copy;

use hyper_sync::{Get, Post};
use hyper_sync::server::{Server, Request, Response};
use hyper_sync::uri::RequestUri::AbsolutePath;

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
);

fn echo(mut req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&Get, "/") | (&Get, "/echo") => {
                try_return!(res.send(b"Try POST /echo"));
                return;
            },
            (&Post, "/echo") => (), // fall through, fighting mutable borrows
            _ => {
                *res.status_mut() = hyper_sync::NotFound;
                return;
            }
        },
        _ => {
            return;
        }
    };

    let mut res = try_return!(res.start());
    try_return!(copy(&mut req, &mut res));
}

fn main() {
    env_logger::init();
    let server = Server::http("127.0.0.1:1337").unwrap();
    let _guard = server.handle(echo);
    println!("Listening on http://127.0.0.1:1337");
}
