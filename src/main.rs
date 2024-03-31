// use rouille::Request;
use rouille::Response;

fn main() {
    println!("Hello, world!");

    rouille::start_server("0.0.0.0:7272", move |_request| {
        println!("Hello, world!");
        Response::text("hello world")
    });
}
