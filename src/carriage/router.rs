use crate::carriage::route::Route;
use crate::carriage::method::Method;

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

    pub fn check_routes(&self, method: &Method, path: &str) {
        let route_index = self.routes.iter().position(|r| r.path == path && r.method == *method);
        match route_index {
            Some(r) => { self.routes[r].process_events() },
            None => {println!("Route haven't found!")}
        }
    }
}

