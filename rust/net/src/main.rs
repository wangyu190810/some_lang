mod query;
use query::{Server,Request,Response};
use std::net::{TcpListener, TcpStream};


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

impl Server {

    pub fn handle_client(mut stream: TcpStream) {
    // pares(&mut stream);
    // let mut rule_url = Vec::new(u8);
        // let resp: Response;
        if let Some(req) = Request::pares(&mut stream){
            if req.path == "/"{
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
    let  server = Server::new("0.0.0.0",5000,".");
    server.start();
}