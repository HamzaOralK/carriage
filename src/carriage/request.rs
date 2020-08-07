// use std::collections::BTreeMap;
use crate::carriage::method::Method;

#[derive(Debug)]
pub struct Request<'a> {
    method: &'a Method,
    body: &'a str,
    // parameters: BTreeMap<&'a str, &'a str>,
    // query: BTreeMap<&'a str, &'a str>,
    url: &'a str
}


impl<'a> Request<'a> {
    pub fn new(url: &'a str, method: &'a Method, body: &'a str) -> Request<'a> {
        Request {
            method,
            url,
            body
        }
    }
}
