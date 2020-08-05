mod carriage;
use carriage::router;
use carriage::method;

fn main() {
    let cr = carriage::Carriage::new("127.0.0.1", "7878");
    let mut x = router::Route::new(method::Method::GET, "/users".to_string(), test);
    x.process_events();

    cr.connect();

}


fn test() {
    println!("test");
}

