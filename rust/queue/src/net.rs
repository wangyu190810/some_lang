use std::net::{TcpListener, TcpStream};


fn handle_client(stream: TcpStream) {
    
    
}

// accept connections and process them serially


pub fn start(){
    // net start
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
     
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }

}