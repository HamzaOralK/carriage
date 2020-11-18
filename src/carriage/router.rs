use crate::carriage::response::Response;
use crate::carriage::request::Request;
use crate::carriage::route::Route;
use crate::carriage::codes::HttpCodes;

#[derive(Clone)]
pub struct Router
{
    pub routes: Vec<Route>,
    pub name: String
}

impl Router
{
    pub fn new(name: &str) -> Router {
        Router {
            routes: Vec::new(),
            name: name.to_string()
        }
    }

    pub fn add_route (&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn check_routes(&self, req: Request) -> Response {
        let route_index = self.routes.iter().position(|r| r.path == req.url && r.method == *req.method);
        match route_index {
            Some(r) => { 
                let res = self.routes[r].process_events(req);
                return res; 
            },
            None => {
                return Response { code: HttpCodes::NotFound, body: "olmadı" };
            }
        };
    }
}

