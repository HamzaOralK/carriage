# Carriage
Carriage is a Rust framework for web applications and API.
## Usage
```rust
    let mut router = Router::new("my route");
    let mut cr = carriage::Carriage::new("127.0.0.1", "7878", &mut router);
    let test_route: Route = Route::new(Method::GET, "/users", test1);
    let test_route2: Route = Route::new(Method::GET, "/products", test2);
    &cr.router.add_route(&test_route);
    &cr.router.add_route(&test_route2);
    &cr.connect();
```
## Usage

To run `cargo run`, to build `cargo build --release`.

## Upcoming features

Parameter parsing and async running on a single thread.


## License
[MIT](https://choosealicense.com/licenses/mit/)