use crate::carriage::response::Response;
use crate::carriage::request::Request;
use crate::carriage::route::Route;
use crate::carriage::method::Method;

#[derive(Clone)]
pub struct Router {
    pub routes: Vec<Route>,
    pub name: String
}

impl Router {
    pub fn new(name: &str) -> Router {
        Router {
            routes: Vec::new(),
            name: name.to_string()
        }
    }

    pub fn add_route<'a> (&'a mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn check_routes<'a>(&'a self, method: &'a Method, path: &'a str, req: Request) -> Response<'a> {
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

