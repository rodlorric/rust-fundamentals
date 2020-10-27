use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {    
    pub fn new(addr: String) -> Self { // Self is an alias of the name of the Struct, so it could alse be "fn new(addr: String) -> Self" instead.
        Self { // Same as above, you could use "Server {"" instead
            addr // It could be "addr: addr" too.
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
    }
}
