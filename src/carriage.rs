use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::ptr;


pub mod router;
pub mod method;
pub mod route;

pub struct Carriage {
    // router: Router,
    listener: TcpListener,
    pub router: router::Router
}

impl Carriage {

    pub fn new(address: &str, port: &str, router: router::Router) -> Carriage {
        let mut ip: String = address.to_owned();
        let port: String = port.to_owned();
        ip.push_str(":");
        ip.push_str(&port);
        let listener = match TcpListener::bind(ip) {
            Ok(listener) => listener,
            Err(e) => {
                println!("Problem parsing arguments: {}", e);
                std::process::exit(1);
            }
        };
        Carriage {
            listener: listener,
            router: router
        }
    }

    pub fn connect(&self) {
        for r in &self.router.routes {
            println!("{}, {:?}", r.path, r.method);
        }
        
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
        let url = self.get_url(Some(&request[..]));
        println!("{:?}, {:?}", method.unwrap(), url.unwrap());

        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn get_method<'a>(&self, request: Option<&'a str>) -> Result<String, String> {
        match request {
            Some(request) => {
                let x = &request.find(" ");
                let slice = match x {
                    Some(x) => &request[0..*x],
                    None => { "" }
                };
                return Ok(slice.to_string());
            },
            None => {
                Err("No request".to_string())
            }
        }
    }

    fn get_url<'a>(&self, request: Option<&'a str>) -> Result<String, String> {
        match request {
            Some(request) => {
                let first_result = request.find(" ").unwrap_or(0);
                let sliced_first = &request[first_result+1..];
                let second_result = sliced_first.find(" ").unwrap_or(0);
                let final_slice = &sliced_first[..second_result];
                return Ok(final_slice.to_string());
            },
            None => {
                Err("No request".to_string())
            }
        }
    }
}

