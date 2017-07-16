extern crate coio;

use std::io::{Read, Write};

use coio::net::TcpListener;
use coio::{spawn, Scheduler};

fn main() {
    // Spawn a coroutine for accepting new connections
    Scheduler::new().with_workers(4).run(move|| {
        let acceptor = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Waiting for connection ...");

        for stream in acceptor.incoming() {
            let (mut stream, addr) = stream.unwrap();

            println!("Got connection from {:?}", addr);

            // Spawn a new coroutine to handle the connection
            spawn(move|| {
                let mut buf = [0; 1024];

                loop {
                    match stream.read(&mut buf) {
                        Ok(0) => {
                            println!("EOF");
                            break;
                        },
                        Ok(len) => {
                            println!("Read {} bytes, echo back", len);
                            stream.write_all(&buf[0..len]).unwrap();
                        },
                        Err(err) => {
                            println!("Error occurs: {:?}", err);
                            break;
                        }
                    }
                }

                println!("Client closed");
            });
        }
    }).unwrap();
}


// fn main() {
//     println!("Hello, world!");
// }
