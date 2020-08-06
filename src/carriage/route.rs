use crate::carriage::method::Method;
pub type Callback = fn();

pub struct Route {
    pub method: Method,
    pub path: String,
    pub callback: Callback
}

impl Route {
    pub fn new<'a> (method: Method, path: String, callback: Callback) -> Route {
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
