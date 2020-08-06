use std::collections::BTreeMap;

pub struct Request {
    method: Method,
    body: String,
    parameters: BTreeMap,
    query: BTreeMap,
    url: String
}

