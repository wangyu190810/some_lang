use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
// use std::fmt::Debug::fmt;

macro_rules! get {
    ( $expr : expr ) => {
        match $expr {
            Some(v) => v,
            None => return None,
        }
    }
}

#[derive(Debug)]
struct Method{
    GET: String,
    POST: String
    
}

impl Method {

    fn new() -> Method{
        Method{
            GET:"GET".to_string(),
            POST:"POST".to_string()
        }
    }
}

struct Request{
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    query: Option<HashMap<String, String>>,

}


impl Request {
    
    // fn new(stream: &mut TcpStream){
    //     Self.pares(stream)
    // }

    fn pares(stream: &mut TcpStream) -> Option<Request>{
        let  mut request_str = Vec::new();
        Self::get_request(stream, &mut request_str);
        
        match String::from_utf8(request_str){
            Ok(request_str) =>{
                let mut base_req = request_str.clone();
                println!("request_str {:}", request_str);
                let mut lines = request_str.split("\t\n");
                println!("linesdata {:?}" ,lines);
                let values: Vec<_> = get!(lines.next()).split(' ').collect();
                println!("values {:?}", values);
                if values.len() >= 3{
                    let methods:Method = Method::new();
                    // let GET = methods.GET;
                    // let POST = methods.POST;
                    // println!("methods GET {:?} POST {:?}",GET,POST);
                    if  values[0] == methods.GET{
                        println!("methods adfad {}",values[0]);
                            let (path, query) = Self::parse_get_resource(values[1]);
                            println!("path {} query {:?}" , path, query);
                            let headers: HashMap<_, _> = lines.flat_map(Self::parse_header).collect();
                            println!("headsds {:?}", headers);
                            Some( Request {
                                    method: values[0].to_string(),
                                    path: path.to_string(),
                                    version: values[2].to_string(),
                                    headers: headers,
                                    query: query,
                                })
                            }
                    else if values[0] == methods.POST {
                            // let pares_post  = move || request_str : pares_post ;
                            println!("POST Method");                          
                            let (path, query) = Self::parse_post_resource(base_req.as_str());  
                            println!("path {} query {:?}" , path, query);
                            let headers: HashMap<_, _> = lines.flat_map(Self::parse_header).collect();
                            println!("headsds {:?}", headers);
                            Some( Request {
                                    method: values[0].to_string(),
                                    path: path.to_string(),
                                    version: values[2].to_string(),
                                    headers: headers,
                                    query: query,
                                })
                            
                        }
                    else {
                            println!("other {}",values[1].to_string());
                            None
                        }
                
                   
                }
                else{
                    None
                }
                
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
    fn parse_get_resource(resource: &str) ->(String, Option<HashMap<String, String>>){
        let parts: Vec<_> = resource.splitn(2, '?').collect();
        if parts.len() == 1 || parts[1].trim().chars().count() == 0 {
            (parts[0].to_string(), None)
        }else {
            (parts[0].to_string(), Self::parse_query(parts[1]))
        }
    }

    fn parse_query(q: &str) -> Option<HashMap<String, String>> {
        let mut query: HashMap<String, String> = HashMap::new();
        let mut it = q.split('&');
        while let Some(kv) = it.next(){
            let mut it = kv.split("=");
            if let Some(k) = it.next(){
                 if let Some(v) = it.next() {
                    query.insert(k.to_string(), v.to_string());
                }
            }
        }
        if query.is_empty() {
            None
        }else{
            Some(query)
        }
    }

    fn parse_post_resource(resource: &str) ->(String, Option<HashMap<String, String>>){
        let parts: Vec<_> = resource.splitn(2, '?').collect();
        // if parts.len() == 1 || parts[1].trim().chars().count() == 0 {
        //     (parts[0].to_string(), None)
        // }else {
            (parts[0].to_string(), Self::parse_post_query(resource))
        // }
    }

 fn parse_post_query(q: &str) -> Option<HashMap<String, String>> {
        let mut query: HashMap<String, String> = HashMap::new();
        // if "Content-Disposition".match(q){

        // }else{

        
        let mut it = q.split("\r\n\r\n");
        println!("parse_post_query{:?}", it);
        it.next();
        while let Some(kv) = it.next(){
            let mut kvs = kv.split("&");
            while let Some(kv) = kvs.next(){
                let mut kv = kv.split("=");
                if let Some(k) = kv.next(){
                    if let Some(v) = kv.next(){
                         query.insert(k.to_string(), v.to_string());
                    }
                }
            }
            // if let Some(k) = it.next(){
            //      if let Some(v) = it.next() {
            //         query.insert(k.to_string(), v.to_string());
            //     }
            // }
        }
        if query.is_empty() {
            None
        }else{
            Some(query)
        }
    }

    fn parse_header(line: &str) -> Option<(String, String)> {
        // let mut it  = line.splitn(2,": ");
        // let header = get!(it.next());
        // let values = get!(it.next());
        // Some((header.to_string()), (values.to_string()))

        let mut it = line.splitn(2, ": ");
        let header = get!(it.next());
        let value = get!(it.next());
        Some((header.to_string(), value.to_string()))

    }   
}

//  fn pares(stream: &mut TcpStream) -> Option<Request>{
//         let  mut request_str = Vec::new();
//         get_request(stream, &mut request_str);
//         match String::from_utf8(request_str){
//             Ok(request_str) =>{
//                 println!("request_str {:}", request_str);
//                 stream::write(header().into_bytes())
//                 //header();
//             }
//             Err(_) =>{
//                 None
//             }
//         }
//     }
    
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
        Self::with_head_body(Self::header(code, mime, content.chars().count()), content)
    }

    fn with_head_body(head: String, body: String) -> Response{
        Response{
            head: head,
            body: body
        }
    }
   
        
    fn send(self, stream: &mut TcpStream) {
        match write!(stream, "{}\r\n{}", self.head, self.body) {
            Err(e) => println!("Response error: {}", e),
            _ => {
                drop(stream);
            } 
        }
    }
        
    fn header(code: u16, mime: &str, length: usize) -> String {
        let m = match code{
            200 => "Ok",
            404 => "Not Found",
            _ => "Not Implemented"
        };
        format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
                code, m, mime, length)
    }

    fn body(s: &str) -> String{
        format!("\r\n{}\r\n",s)
    }
}

// 

fn handle_client(mut stream: TcpStream) {
    // pares(&mut stream);
    if let Some(req) = Request::pares(&mut stream){
        process(req)
    }
    let content = "hello worldasdfasdf";
    
    Response::new(200, "text/html",content.to_string()).send(&mut stream);
   
}



// fn process(stream: TcpStream, req: Request){
fn process(req: Request){
    let content = "hello worldasdfasdf";
    // let method = req.method.as_str();
    // println!("{}",method);
    let get = "GET".to_string();
    match req.method {
            get => println!("{}",get)
    // match method{
    //     "GET" =>  Response::new(200, "text/html",content.to_string()).send(&mut stream),
    // }
    }
}


fn server(){
    let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
    // let rock: Arc<Rock> = Arc::new(self);
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
                
                // stream.close();
            }
            Err(e) => { 
                print!("asdfasdf{:}", e);
                /* connection failed */ }
        }
    }
    drop(listener);
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