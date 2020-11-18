use std::error::Error;

use tokio::net::{ TcpListener, TcpStream };
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub mod router;
pub mod method;
pub mod route;
pub mod request;
pub mod response;
pub mod codes;

use request::*;

pub struct Carriage<'a>
{
    address: String,
    pub router: &'a mut router::Router
}

impl<'a> Carriage <'a> {
    pub fn new(address: &str, port: &str, router: &'a mut router::Router) -> Carriage<'a> {
        let mut address: String = address.to_owned();
        let port: String = port.to_owned();
        address.push_str(":");
        address.push_str(&port);
        Carriage {
            address,
            router
        }
    }

    pub async fn connect(&'a mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let mut listener = TcpListener::bind(&self.address).await.unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let r = self.router.clone();
            tokio::spawn(async move {
                let _ = handle_request(&r, socket).await;
            });
        }
    }
}


pub async fn handle_request (router: &router::Router, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let url = get_url(Some(&request[..]));
    let method = match get_method(Some(&request[..])) {
        Ok(m) => {decide_method(&m)},
        Err(_e) => { method::Method::NONE }
    };

    let res = match url {
        Ok(url) => {
            let body = get_body(&request).unwrap();
            let request = Request::new(&url, &method, body);
            let response = router.check_routes(request);
            response
        },
        Err(e) => {
            println!("{}", e);
            response::Response { code: codes::HttpCodes::NotFound, body: "{\"error\": \"No responser\"}" }
        }
    };

    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        res.code,
        res.body.len(),
        res.body
    );
    let _ = stream.write(&response.as_bytes()).await;
    Ok(())
}

fn get_method(request: Option<&str>) -> Result<String, String> {
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

fn decide_method(method_string: &str) -> method::Method {
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

fn get_url(request: Option<&str>) -> Result<String, String> {
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


fn get_body(request: &str) -> Result<SimpleBodyData, String> {
    let double_space_index = request.find("\r\n\r\n").unwrap_or(0);
    let sliced_first = &request[double_space_index + 4..];
    let second_result = sliced_first.find("\u{0}").unwrap_or(0);
    let final_slice = &sliced_first[..second_result];
    let deserialized: SimpleBodyData = SimpleBodyData::new(serde_json::from_str(final_slice).unwrap());
    Ok(deserialized)
}
