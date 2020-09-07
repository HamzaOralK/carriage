use std::collections::BTreeMap;
use crate::carriage::method::Method;
use std::fmt::Debug;
use serde::{Deserialize};


pub trait SimpleBody { }
#[derive(Clone, Deserialize, Debug)]
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
pub struct Request<'a>
{
    pub body: SimpleBodyData,
    pub method: &'a Method,
    pub url: &'a str,
}

pub trait Req<'a> {
    fn new(url: &'a str, method: &'a Method, body: SimpleBodyData,) -> Self;
}

impl<'a> Req<'a> for Request<'a>
{
    fn new(url: &'a str, method: &'a Method, body: SimpleBodyData) -> Self {
        Request { body, method, url }
    }
}
