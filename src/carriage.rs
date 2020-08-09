use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

pub mod router;
pub mod method;
pub mod route;
pub mod request;
pub mod response;
pub mod threadPool;


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
        
        let url = self.get_url(Some(&request[..]));
        let method = match self.get_method(Some(&request[..])) {
            Ok(m) => {self.decide_method(&m)},
            Err(e) => { method::Method::NONE }
        };
        let body = String::from("test body");

        let res = match &url {
            Ok(url) => {
                let request = request::Request::new(&url, &method, &body);
                self.router.check_routes(&method, &url, request)
            },
            Err(e) => {
                response::Response { code: "404", body: "{\"error\": \"No responser\"}" }
            }
        };

        println!("{:?}", res);

        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            res.code,
            res.body.len(),
            res.body
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

    fn decide_method(&self, method_string: &str) -> method::Method {
        match method_string {
            "GET" => method::Method::GET,
            "POST" => method::Method::POST,
            "PUT" => method::Method::PUT,
            "PATCH" => method::Method::PATCH,
            "DELETE" => method::Method::DELETE,
            "OPTIONS" => method::Method::OPTIONS,
            "HEAD" => method::Method::HEAD,
            _ => method::Method::NONE
        }
    }


    ///URL parser yapılacak url içerisindeki /'ları ayıracak "/users/:id"
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

