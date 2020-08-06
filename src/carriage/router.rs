use crate::carriage::route::Route;

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
}

