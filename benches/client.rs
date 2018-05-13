#![deny(warnings)]
#![feature(test)]
extern crate hyper_sync;

extern crate test;

use std::fmt;
use std::io::{self, Read, Write, Cursor};
use std::net::SocketAddr;
use std::time::Duration;

use hyper_sync::{net, header};

static README: &'static [u8] = include_bytes!("../README.md");

struct MockStream {
    read: Cursor<Vec<u8>>
}

impl MockStream {
    fn new() -> MockStream {
        let head = b"HTTP/1.1 200 OK\r\nServer: Mock\r\n\r\n";
        let mut res = head.to_vec();
        res.extend_from_slice(README);
        MockStream {
            read: Cursor::new(res)
        }
    }
}

impl Clone for MockStream {
    fn clone(&self) -> MockStream {
        MockStream {
            read: Cursor::new(self.read.get_ref().clone())
        }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl Write for MockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        // we're mocking, what do we care.
        Ok(msg.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Foo;

impl header::Header for Foo {
    fn header_name() -> &'static str {
        "x-foo"
    }

    fn parse_header(_: &header::Raw) -> hyper_sync::Result<Foo> {
        Err(hyper_sync::Error::Header)
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
        f.fmt_line(&"Bar")
    }
}

impl net::NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // can't time out
        Ok(())
    }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // can't time out
        Ok(())
    }
}

struct MockConnector;

impl net::NetworkConnector for MockConnector {
    type Stream = MockStream;
    fn connect(&self, _: &str, _: u16, _: &str) -> hyper_sync::Result<MockStream> {
        Ok(MockStream::new())
    }
}

#[bench]
fn bench_mock_hyper(b: &mut test::Bencher) {
    let url = "http://127.0.0.1:1337/";
    b.iter(|| {
        let mut req = hyper_sync::client::Request::with_connector(
            hyper_sync::Get, hyper_sync::Url::parse(url).unwrap(), &MockConnector
        ).unwrap();
        req.headers_mut().set(Foo);

        let mut s = String::new();
        req
            .start().unwrap()
            .send().unwrap()
            .read_to_string(&mut s).unwrap()
    });
}
