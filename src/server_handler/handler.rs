use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs;
use std::time::Duration;
use std::thread;

use super::response::Response;

pub struct Handler {
    stream : TcpStream
}

impl Handler {
    pub fn new(stream : TcpStream) -> Handler {
        Handler{stream}
    }

    pub fn handle(&mut self) {
        let mut buffer = [0; 512];
        self.stream.read(&mut buffer).unwrap();
        let answ = Handler::answer(buffer);

        self.response(answ.response, answ.page);
    }

    fn response(&mut self, status : &str, page : &str) {
        let contents = fs::read_to_string(page).unwrap();
        let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    fn answer(request: [u8;512]) -> Response<'static, 'static> {
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let ok = "200 OK";
        let hello = "hello.html";
        let not_found = "404.html";

        if request.starts_with(get) {
            Response::new(ok, hello)
        }
        else if request.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            Response::new(ok, hello)
        }
        else {
            Response::new("404 NOT FOUND", not_found)
        }
    }
}