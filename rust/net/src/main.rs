use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

struct Request{
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    query: Option<HashMap<String, String>>,

}

impl Request {
    
    fn pares(stream: &mut TcpStream) -> Option<Request>{
        let  mut request_str = Vec::new();
        Request.get_request(stream, &mut request_str);
        match String::from_utf8(request_str){
            Ok(request_str) =>{
                println!("request_str {:}", request_str);
                None
            }
            Err(_) =>{
                None
            }
        }
    }
    
    fn get_request(stream: &mut TcpStream, r: &mut Vec<u8>){
        const CHUNCK_SIZE: usize = 4096;
        let mut buf = [0;CHUNCK_SIZE];
        while let Ok(len) = stream.read(&mut buf) {
            r.extend_from_slice(&buf[..len]);
            if len != CHUNCK_SIZE{
                return ;
            }
        }

    }
}

 fn pares(stream: &mut TcpStream) -> Option<Request>{
        let  mut request_str = Vec::new();
        get_request(stream, &mut request_str);
        match String::from_utf8(request_str){
            Ok(request_str) =>{
                println!("request_str {:}", request_str);
                stream::write(header().into_bytes())
                //header();
            }
            Err(_) =>{
                None
            }
        }
    }
    
fn get_request(stream: &mut TcpStream, r: &mut Vec<u8>){
        const CHUNCK_SIZE: usize = 4096;
        let mut buf = [0;CHUNCK_SIZE];
        while let Ok(len) = stream.read(&mut buf) {
            r.extend_from_slice(&buf[..len]);
            if len != CHUNCK_SIZE{
                return ;
            }
        }

    }


struct Response {
    head: String,
    body: String
}

impl Response{
    fn new(code: u16, mime: &str, content: String) -> Response{

    }

    fn with_head_body(head: String, body: String) -> Response{
        Response{
            head: head,
            body: body
        }
    }
    
    fn header(code: u16, mime: &str, length: usize) -> String {
        let m = match code{
            200 => "Ok",
            404 => "Not Found",
            - => "Not Implemented"
        }
        format!("HTTP/1.1 200 \r\nContent-Type: \r\nContent-Length: 10\r\n")
    }
}

// 

fn handle_client(mut stream: TcpStream) {
    pares(&mut stream);
   
}


fn server(){
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { 
                print!("asdfasdf{:}", e);
                /* connection failed */ }
        }
    }
}

fn client(){
   let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

    // ignore the Result
    let _ = stream.write(&[1]);
    let _ = stream.read(&mut [0; 128]); // ignore here too
} 



fn main(){
    server();
}