extern crate net;

use net::query::{Request,Response};
use std::net::{TcpListener, TcpStream};

use net::Pool::ThreadPool;
use net::utils::{rule_data,rule_data_app,static_response};
use net::rule::test_make_map;

pub struct Server{
    host:String,
    port:usize,
    pub static_path:String
}

impl Server {

    
    pub fn new(host: &str, port: usize, static_path:&str ) -> Server{
        Server{
            host:host.to_string(),
            port:port,
            static_path: static_path.to_string()
        }
    }
    
    pub fn process_request(static_path:String,req: net::query::Request,mut stream: TcpStream){
        if req.path.ends_with(".html") || req.path.ends_with(".js"){
               
                // let resp = static_response(static_path_clone,req_path_clone);
                let resp = static_response(&static_path,&req.path);
                println!("static file{}", req.path);
                resp.send(&mut stream);
            } else if req.path == "/"{
                let resp =rule_data("/", req);
                resp.send(&mut stream);
            }else if req.path == "/index"{
                let resp = rule_data_app("/index", req);
                resp.send(&mut stream);       
            }else{
                let content = "404";
                let resp = Response::new(404,"text/html",content.to_string());
                resp.send(&mut stream);
            }

    }
        

    pub fn run(&self){
        // let addr = format!("{}:{}", self.host, self.port).as_str();

        let listener = TcpListener::bind(format!("{}:{}",self.host,self.port).as_str()).unwrap();
        // let listener = TcpListener::bind(&addr).unwrap();
        // let rock: Arc<Rock> = Arc::new(self);
        // accept connections and process them, spawning a new thread for each one
        let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    pool.execute(||{
                         Self::handle_client(stream);
                    }
                    )
                   
                
                }
                Err(e) => { 
                    print!("asdfasdf{:}", e);
                    /* connection failed */ }
            }
        }
        drop(listener);
    }

    
   pub fn handle_client(mut stream: TcpStream) {
    // pares(&mut stream);
    // let mut rule_url = Vec::new(u8);
        // let resp: Response;
        let static_path = String::from("./src/static/");
        if let Some(req) = Request::pares(&mut stream){
            // let mut static_path_clone = self.static_path.clone();
<<<<<<< HEAD
            Self::process_request(static_path ,req, stream);
            
=======
            // let mut req_path_clone = req.path.clone();
            if req.path.ends_with(".html") || req.path.ends_with(".js"){
               
                // let resp = static_response(static_path_clone,req_path_clone);
                let resp = static_response(&static_path,&req.path);
                println!("static file{}", req.path);
                resp.send(&mut stream);
            }
            else if req.path == "/"{
                let resp =rule_data(req);
                resp.send(&mut stream);
            }else if req.path == "/index"{
                let resp = rule_data_app("/index", req);
                resp.send(&mut stream);       
            }else{
                let content = "404";
                let resp = Response::new(404,"text/html",content.to_string());
                resp.send(&mut stream);
            }
>>>>>>> origin/master
            // resp.send(&mut stream);
        }
        else{
            let resp = Response::html_500_body();
            resp.send(&mut stream);
        }
    //  resp.send(&mut stream);
    }
   

    pub fn start(&self){
        println!("net server start  addr  {}:{}", self.host,self.port);
        Self::run(self);
        
    }

}



fn main(){
    // let  server = Server::new("0.0.0.0",5000,"/home/too/work/some_lang/rust/net/src/static/");
    let  server = Server::new("0.0.0.0",5000,"./src/static/");
    server.start();
    // test_make_map()
}