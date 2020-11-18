#![allow(dead_code)]
#![allow(unused_variables)]

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
use std::fmt::{ Display, Formatter, Result};

#[derive(Debug)]
pub enum HttpCodes {
  // 2xx Success
  OK,
  Accepted,
  // 3xx Redirections
  MovedPermanently = 301,
  // 4xx Client Error
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
  RequestTimeout = 408,
  // 5xx ServerError
  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502,
  ServiceUncallable = 503
}

impl Display for HttpCodes {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      HttpCodes::OK => write!(f, "200"),
      HttpCodes::Accepted => write!(f, "202"),
      HttpCodes::MovedPermanently => write!(f, "301"),
      HttpCodes::BadRequest => write!(f, "400"),
      HttpCodes::Unauthorized => write!(f, "401"),
      HttpCodes::Forbidden => write!(f, "403"),
      HttpCodes::NotFound => write!(f, "404"),
      HttpCodes::RequestTimeout => write!(f, "408"),
      HttpCodes::InternalServerError => write!(f, "500"),
      HttpCodes::NotImplemented => write!(f, "501"),
      HttpCodes::BadGateway => write!(f, "502"),
      HttpCodes::ServiceUncallable => write!(f, "503"),
    }
  }
}