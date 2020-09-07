mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::response::Response;
use carriage::request::*;
use carriage::method::Method;

fn main() {
    let mut router = Router::new("my route");
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", &mut router);
    let test_route: Route = Route::new(Method::GET, "/users", test1);
    let test_route2: Route = Route::new(Method::GET, "/products", test2);
    &cr.router.add_route(&test_route);
    &cr.router.add_route(&test_route2);
    &cr.connect();
}


fn test1(req: Request) -> Response<'static> {
    let body = req.body;
    if body.data["productId"] == "test" {
        println!("güzel product");
    }
    let res = Response { code: "200", body: "asdasd"};
    res
}

fn test2(req: Request) -> Response<'static> {
    let body = req.body;
    if body.data["productId"] == "test" {
        println!("güzel product");
    }
    let res = Response { code: "200", body: "products"};
    res
}