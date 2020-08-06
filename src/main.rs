mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::method::Method;

fn main() {
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", Router::new("my route"));
    let test_route = Route::new(Method::POST, "/users".to_string(), test);
    cr.router.add_route(test_route);

    cr.connect();

}


fn test() {
    println!("test");
}

