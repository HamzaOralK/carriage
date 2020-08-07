mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::response::Response;
use carriage::request::Request;
use carriage::method::Method;

fn main() {
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", Router::new("my route"));
    let test_route = Route::new(Method::GET, "/users", test);
    let second_test_route = Route::new(Method::GET, "/products", test2);
    cr.router.add_route(test_route);
    cr.router.add_route(second_test_route);

    cr.connect();

}


fn test(req: Request) -> Response<'static> {
    println!("{:?}", req);
    let res = Response { code: "200", body: "{\"test\": 123}"};
    res
}

fn test2(req: Request) -> Response<'static> {
    println!("{:?}", req);
    let res = Response { code: "200", body: "{\"test\": 123444}"};
    res
}
