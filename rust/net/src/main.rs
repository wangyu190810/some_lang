mod query;
use query::{Server,Request,Response};
use std::net::{TcpListener, TcpStream};
use std::path::{PathBuf,Path};
use std::fs::File;
use std::io::*;

fn rule_data(roule: &str, req: Request) -> Response{
    if let Some(req_str) = req.query{
        let name =  req_str.get("name").unwrap();
        let content = name.to_string();
        Response::new(200, "text/html",content)
    }else{

        Response::html_404_body()
    }    
}

fn rule_data_app(roule: &str, req: Request) -> Response{
    if let Some(req_str) = req.query{
        let name =  req_str.get("name").unwrap();
        let content = name.to_string();
        Response::new(200, "text/html",content)
    }else{
        Response::html_500_body()
    }

}

 pub fn static_response(static_path: &String ,path: &String) -> Response{
        // 静态文件解析和返回
        // let mut buf = PathBuf::from(static_path);
        // buf.push(path);
        let mut buf = Path::new(static_path).to_path_buf();
        let p = match path.chars().count() {
            1 => "index.html".to_string(),
            _ => path.chars().skip(1).collect(),
        };
        buf.push(p);
        match buf.as_path().to_str(){
            
            Some(path) => {
                println!("path is {},",path);
                match File::open(path){
                    Ok(mut file ) =>{
                        let mut body = String::new();
                        file.read_to_string(&mut body).unwrap();
                        Response::new(200, "text/html",body)
                    
                    },
                    Err(mut err) => {
                        println!("path is {},{:?}",path,err);
                        Response::html_404_body()
                    }
                
                    
                }
            },
            None =>{
                Response::html_404_body()
            }
        }
    }


impl Server {

    pub fn handle_client(&self, mut stream: TcpStream) {
    // pares(&mut stream);
    // let mut rule_url = Vec::new(u8);
        // let resp: Response;
        
        if let Some(req) = Request::pares(&mut stream){
            // let mut static_path_clone = self.static_path.clone();
            // let mut req_path_clone = req.path.clone();
            if req.path.ends_with(".html") || req.path.ends_with(".js"){
               
                // let resp = static_response(static_path_clone,req_path_clone);
                let resp = static_response(&self.static_path,&req.path);
                println!("static file{}", req.path);
                resp.send(&mut stream);
            }
            else if req.path == "/"{
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
            // resp.send(&mut stream);
        }
        else{
            let resp = Response::html_500_body();
            resp.send(&mut stream);
        }
    //  resp.send(&mut stream);
    }
}

fn main(){
    // let  server = Server::new("0.0.0.0",5000,"/home/too/work/some_lang/rust/net/src/static/");
    let  server = Server::new("0.0.0.0",5000,"./src/static/");
    server.start();
}