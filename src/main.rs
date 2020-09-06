mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::response::Response;
use carriage::request::*;
use carriage::method::Method;

fn main() {
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", Router::new("my route"));
    let test_route = Route::new(Method::GET, "/users", test1);
    cr.router.add_route(test_route);
    cr.connect();
}


#[derive(Clone, Copy, Debug)]
pub struct my_body<'a> {
    test: &'a str,
}

impl<'a> SimpleBody for my_body<'a> {}


fn test1(req: Request<my_body>) -> Response<'static> {
    let res = Response { code: "200", body: "{\"test\": 123}"};
    res
}
