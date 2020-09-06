use crate::carriage::method::Method;
use crate::carriage::request::*;
use crate::carriage::response::Response;
use std::fmt::Debug;

pub type Callback<T> = fn(req: Request<T>) -> Response<'static>;

#[derive(Clone)]
pub struct Route<T> 
    where T: SimpleBody + Copy + Clone + Debug
{
    pub method: Method,
    pub path: String,
    pub callback: Callback<T>
}

impl<T> Route<T> 
    where T: SimpleBody + Copy + Clone + Debug 
{
    pub fn new (method: Method, path: &str, callback: Callback<T>) -> Route<T> {
        Route {
            method,
            path: path.to_string(),
            callback: callback
        }
    }

    pub fn process_events(&self, req: Request<T>) -> Response {
        let res = (self.callback)(req);
        return res;
    }
}
