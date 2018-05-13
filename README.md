# hyper-sync

[![Travis Build Status](https://travis-ci.org/khvzak/hyper-sync.svg?branch=master)](https://travis-ci.org/khvzak/hyper-sync)
[![Coverage Status](https://coveralls.io/repos/github/khvzak/hyper-sync/badge.svg?branch=master)](https://coveralls.io/github/khvzak/hyper-sync?branch=master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/hyper-sync)](https://crates.io/crates/hyper-sync)
[![Released API docs](https://docs.rs/hyper-sync/badge.svg)](https://docs.rs/hyper-sync)

A Modern HTTP library for Rust.
Fork of the original hyper library (v0.10.x)

### Documentation

- [Stable](http://docs.rs/hyper-sync)

## Overview

Hyper is a fast, modern HTTP implementation written in and for Rust. It
is a low-level typesafe abstraction over raw HTTP, providing an elegant
layer over "stringly-typed" HTTP.

Hyper offers both an HTTP client and server which can be used to drive
complex web applications written entirely in Rust.

The documentation is located at [http://docs.rs/hyper-sync](http://docs.rs/hyper-sync).

## Example

### Hello World Server:

```rust
extern crate hyper_sync;

use hyper_sync::Server;
use hyper_sync::server::{Request, Response};

fn hello(_: Request, res: Response) {
    res.send(b"Hello World!").unwrap();
}

fn main() {
    Server::http("127.0.0.1:3000").unwrap()
        .handle(hello).unwrap();
}
```

### Client:

```rust
extern crate hyper_sync;

use std::io::Read;

use hyper_sync::Client;
use hyper_sync::header::Connection;

fn main() {
    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://rust-lang.org/")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
```
