use crate::carriage::method::Method;
use crate::carriage::request::Request;
use crate::carriage::response::Response;

pub type Callback = fn(req: Request) -> Response<'static>;

pub struct Route {
    pub method: Method,
    pub path: String,
    pub callback: Callback
}

impl Route {
    pub fn new (method: Method, path: &str, callback: Callback) -> Route {
        Route {
            method,
            path: path.to_string(),
            callback: callback
        }
    }

    pub fn process_events(&self, req: Request) -> Response {
        println!("process events");
        let res = (self.callback)(req);
        return res;
    }
}
