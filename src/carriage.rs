use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

pub mod router;
pub mod method;

pub struct Carriage {
    // router: Router,
    listener: TcpListener,
    routers: Vec<router::Router>
}

impl Carriage {

    pub fn new(address: &str, port: &str) -> Carriage {
        let mut ip: String = address.to_owned();
        let port: String = port.to_owned();
        ip.push_str(":");
        ip.push_str(&port);
        Carriage {
            listener:  TcpListener::bind(ip).unwrap_or_else(|err| {
                println!("Problem parsing arguments: {}", err);
                std::process::exit(1);
            }),
            routers: Vec::new()
        }
    }

    pub fn connect(&self) {

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.handle_request(stream);
        }
    }

    fn handle_request(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let method = self.get_method(Some(&request[..]));

        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn get_method<'a>(&self, request: Option<&'a str>) -> Option<&'a str> {
        match request {
            Some(request) => {
                let x = &request.find(" ");
                let slice: &str;
                slice = &request[0..x.unwrap()];
                return Some(slice);
            },
            None => {
                None
            }
        }
        
    }
}

