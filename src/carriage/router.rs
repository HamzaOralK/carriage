use crate::carriage::response::Response;
use crate::carriage::request::Request;
use crate::carriage::route::Route;
use crate::carriage::method::Method;
use crate::carriage::request::*;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Router<T>
    where T: SimpleBody + Copy + Debug
{
    pub routes: Vec<Route<T>>,
    pub name: String
}

impl<T> Router<T> 
    where T: SimpleBody + Copy + Debug
{
    pub fn new(name: &str) -> Router<T> {
        Router {
            routes: Vec::new(),
            name: name.to_string()
        }
    }

    pub fn add_route<'a> (&'a mut self, route: Route<T>) {
        self.routes.push(route);
    }

    pub fn check_routes<'a>(&'a self, method: &'a Method, path: &'a str, req: Request<T>) -> Response<'a> {
        let route_index = self.routes.iter().position(|r| r.path == path && r.method == *method);
        match route_index {
            Some(r) => { 
                let res = self.routes[r].process_events(req);
                return res; 
            },
            None => {
                return Response { code: "404", body: "olmadı" };
            }
        };
    }
}

