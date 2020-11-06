#![allow(dead_code)]

use server::Server;
use website_handler::WebSiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;
fn main() {

    //let string = String::from("127.0.0.0:8080");
    //let string_slice = &string[10..]; // it's the same as let "string_slice = &string[10..14]; from 10th byte to 14th."
    //let string_borrow: &str = &string;
    //let string_literal = "1234";

    //dbg!(&string);
    //dbg!(string_slice);
    //dbg!(string_borrow);
    //dbg!(string_literal);

    //let get = Method::GET;
    //let delete = Method::DELETE;
    //let post = Method::POST;
    //let put = Method::PUT;
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebSiteHandler::new(public_path));
}

/* GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/