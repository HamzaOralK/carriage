#![feature(async_closure)]
mod carriage;
use carriage::router::Router;
use carriage::route::Route;
use carriage::response::Response;
use carriage::request::*;
use carriage::method::Method;
use carriage::codes::HttpCodes;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new("my router");
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", &mut router);
    let test_route: Route = Route::new(Method::POST, "/users", test1);
    let test_route2: Route = Route::new(Method::GET, "/products", test2);
    cr.router.add_route(test_route);
    cr.router.add_route(test_route2);
    let _ = cr.connect().await;
    Ok(())
}


fn test1(req: Request) -> Response<'static> {
    println!("{:?}", req.body.data["productId"]);
    let body = req.body;
    if body.data["productId"] == "test" {
        println!("güzel product");
    }
    let res = Response { code: HttpCodes::Unauthorized, body: "asdasd"};
    res
}

fn test2(req: Request) -> Response<'static> {
    let body = req.body;
    let mut res = Response { code: HttpCodes::OK, body: "products"};
    if body.data["productId"] == "test" {
        res.body = "güzel product";
    }
    res
}