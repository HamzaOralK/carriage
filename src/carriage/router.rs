use crate::carriage::method;
pub type Callback = fn();

pub struct Router { }

pub struct Route {
    method: method::Method,
    path: String,
    callback: Callback
}

impl Route {
    pub fn new<'a> (method: method::Method, path: String, callback: Callback) -> Route {
        Route {
            method,
            path,
            callback
        }
    }

    // pub fn set_callback(&mut self, c: Callback) {
    //     self.callback = c;
    // }

    pub fn process_events(&self) {
        println!("process events");
        (self.callback)();
    }
}



