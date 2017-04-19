extern crate queue;

use std::net::TcpListener;
use std::time::{Duration, SystemTime};
use std::thread;

use queue::Queues;
use queue::basequeue;
use queue::test;
use queue::net;
use queue::rpcbase;

static NTHREADS: i32 = 10;

// This is the `main` thread

fn data_process(i: i32) {
    println!("data_process thread {}", i)
}


// fn main() {

//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     listener.set_nonblocking(true).expect("can`t set non block");
//     match listener.accept() {
//         Ok((_socket, addr)) => println!("new client: {:?}", addr),
//         Err(e) => println!("couldn't get client: {:?}", e),
//     }

// }

fn main() {
    // basequeue::thread_test();
    // net::start();
    rpcbase::run();
}


// fn main() {
//     // Make a vector to hold the children which are spawned.
//     let mut children = vec![];

//     for i in 0..NTHREADS {
//         // Spin up another thread
//         children.push(thread::spawn(move ||
//         data_process(i)
//         ));
//     }

//     for child in children {
//         // Wait for the thread to finish. Returns a result.
//         let _ = child.join();
//     }
// }