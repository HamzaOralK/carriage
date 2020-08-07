mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::response::Response;
use carriage::request::Request;
use carriage::method::Method;

fn main() {
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", Router::new("my route"));
    let test_route = Route::new(Method::GET, "/users", test);
    cr.router.add_route(test_route);

    cr.connect();

}


fn test(req: Request) -> Response<'static> {
    println!("{:?}", req);
    let res = Response { code: "200", body: "{\"test\": 123}"};
    res
}

