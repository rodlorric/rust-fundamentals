use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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
        println!("Listening on: {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to stablish a conection: {}", e),
            }
        }
    }
}
