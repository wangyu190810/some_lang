use std::sync::mpsc;
use std::thread;

use futures::Future;
use tarpc::future::{client, server};
use tarpc::future::client::ClientExt;
use tarpc::util::{FirstSocketAddr, Never};


use tarpc;
use tokio_core::reactor;

// service! {
//     rpc hello(name: String) -> String;
//     rpc start(name: String) -> String;
// }

// #[derive(Clone)]
// struct HelloServer;

// impl SyncService for HelloServer {
    
//     fn hello(&self, name: String) -> Result<String, Never> {
//         Ok(format!("Hello, {}!", name))
//     }
//     fn start(&self, name: String) -> Result<String, Never> {
//         Ok(format!("start, {}!", name))
//     }
// }

// pub fn run() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let mut handle = HelloServer.listen("localhost:0", server::Options::default())
//             .unwrap();
//         tx.send(handle.addr()).unwrap();
//         handle.run();
//     });
//     let client = SyncClient::connect(rx.recv().unwrap(), client::Options::default()).unwrap();
//     println!("{}", client.hello("Mom".to_string()).unwrap());
//     println!("{}", client.start("Mom".to_string()).unwrap());

    
// }

// #[derive(Clone)]
// struct HelloServerFuture;

// impl FutureService for HelloServerFuture {
//     type HelloFut = Result<String, Never>;

//     fn hello(&self, name: String) -> Self::HelloFut {
//         Ok(format!("Hello, {}!", name))
//     }
//     fn start(&self, name: String) -> Self::HelloFut {
//         Ok(format!("Start, {}!", name))
//     }
// }

// fn run_data(){
//     let mut reactor = reactor::Core::new().unwrap();
//     let (handle, server) = HelloServerFuture.listen("localhost:10000".first_socket_addr(),
//                                   &reactor.handle(),
//                                   server::Options::default())
//                           .unwrap();
//     reactor.handle().spawn(server);
//     let options = client::Options::default().handle(reactor.handle());
//     reactor.run(FutureClient::connect(handle.addr(), options)
//             .map_err(tarpc::Error::from)
//             .and_then(|client| client.hello("Mom".to_string()))
//             .map(|resp| println!("{}", resp)))
//         .unwrap();

// }

// service! {
//     rpc hello(name: String) -> String;
// }

// service! {
//     rpc hello(name: String) -> String;
//     rpc hello1(name: String) -> String;
// }



// // service! {
// //     rpc hello(name: String) -> String;
// // }

// #[derive(Clone)]
// struct HelloServer;

// impl SyncService for HelloServer {
//     fn hello(&self, name: String) -> Result<String, Never> {
//         Ok(format!("Hello, {}!", name))
//     }
//     fn hello1(&self, name: String) -> Result<String, Never> {
//         Ok(format!("Hello, {}!", name))
//     }
// }

// fn get_acceptor() -> TlsAcceptor {
//      let buf = include_bytes!("test/identity.p12");
//      let pkcs12 = Pkcs12::from_der(buf, "password").unwrap();
//      TlsAcceptor::builder(pkcs12).unwrap().build().unwrap()
// }

// fn main() {
//     let addr = "localhost:10000";
//     let acceptor = get_acceptor();
//     let _server = HelloServer.listen(addr, server::Options::default().tls(acceptor));
//     let client = SyncClient::connect(addr,
//                                      client::Options::default()
//                                          .tls(tls::client::Context::new("foobar.com").unwrap()))
//                                          .unwrap();
//     println!("{}", client.hello("Mom".to_string()).unwrap());
// }




service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
struct HelloServer;

impl FutureService for HelloServer {
    type HelloFut = Result<String, Never>;

    fn hello(&self, name: String) -> Self::HelloFut {
        Ok(format!("Hello, {}!", name))
    }
}

pub fn run() {
    let mut reactor = reactor::Core::new().unwrap();
    let (handle, server) = HelloServer.listen("localhost:10000".first_socket_addr(),
                                  &reactor.handle(),
                                  server::Options::default())
                          .unwrap();
    reactor.handle().spawn(server);
    let options = client::Options::default().handle(reactor.handle());
    reactor.run(FutureClient::connect(handle.addr(), options)
            .map_err(tarpc::Error::from)
            .and_then(|client| client.hello("Mom".to_string()))
            .map(|resp| println!("{}", resp)))
        .unwrap();
}