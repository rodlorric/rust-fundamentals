fn main() {

    //let string = String::from("127.0.0.0:8080");
    //let string_slice = &string[10..]; // it's the same as let "string_slice = &string[10..14]; from 10th byte to 14th."
    //let string_borrow: &str = &string;
    //let string_literal = "1234";

    //dbg!(&string);
    //dbg!(string_slice);
    //dbg!(string_borrow);
    //dbg!(string_literal);

    let server = Server::new("127.0.0.0:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self { // Self is an alias of the name of the Struct, so it could alse be "fn new(addr: String) -> Self" instead.
        Self { // Same as above, you could use "Server {"" instead
            addr // It could be "addr: addr" too.
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}