use std::collections::BTreeMap;
use crate::carriage::method::Method;
use std::fmt::Debug;
use std::marker::Copy;
use serde::{Serialize, Deserialize};


pub trait SimpleBody { }
#[derive(Deserialize, Debug)]
pub struct SimpleBodyData {
    pub data: BTreeMap<String, String>
}

impl SimpleBody for SimpleBodyData {}

impl SimpleBodyData {
    pub fn new(data: BTreeMap<String, String>) -> SimpleBodyData {
        SimpleBodyData {
            data
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request<'a, T>
where
    T: SimpleBody + Copy + Clone + Debug,
{
    body: Option<T>,
    method: &'a Method,
    url: &'a str,
}

pub trait Req<'a> {
    type inner_type: SimpleBody + Copy + Clone + Debug;
    // fn new(url: &'a str, method: Method, body: Self::inner_type,) -> Self;
    fn new(url: &'a str, method: &'a Method) -> Self;
}

impl<'a, T> Req<'a> for Request<'a, T>
where
    T: SimpleBody + Copy + Clone + Debug,
{
    type inner_type = T;

    // fn new(url: &'a str, method: Method, body: T) -> Self {
    fn new(url: &'a str, method: &'a Method) -> Self {
        Request { body: None, method, url }
    }
}
