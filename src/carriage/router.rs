use crate::carriage::response::Response;
use crate::carriage::request::Request;
use crate::carriage::route::Route;
use crate::carriage::method::Method;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Router<'a>
{
    pub routes: Arc<Mutex<Vec<&'a Route>>>,
    pub name: &'a str
}

impl<'a> Router<'a> 
{
    pub fn new(name: &str) -> Router {
        Router {
            routes: Arc::new(Mutex::new(Vec::new())),
            name: name
        }
    }

    pub fn add_route (&mut self, route: &'a Route) {
        self.routes.lock().unwrap().push(route);
    }

    pub fn check_routes(&self, req: Request) -> Response {
        let route_index = self.routes.lock().unwrap().iter().position(|r| r.path == req.url && r.method == *req.method);
        match route_index {
            Some(r) => { 
                let res = self.routes.lock().unwrap()[r].process_events(req);
                return res; 
            },
            None => {
                return Response { code: "404", body: "olmadı" };
            }
        };
    }
}

