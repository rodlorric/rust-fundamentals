use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;
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

    let server = Server::new("127.0.0.0:8080".to_string());
    server.run();
}

/* GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/