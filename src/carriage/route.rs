use crate::carriage::method::Method;
pub type Callback = fn();

pub struct Route {
    pub method: Method,
    pub path: String,
    pub callback: Callback
}

impl Route {
    pub fn new<'a> (method: Method, path: &str, callback: Callback) -> Route {
        Route {
            method,
            path: path.to_string(),
            callback
        }
    }

    pub fn process_events(&self) {
        println!("process events");
        (self.callback)();
    }
}
