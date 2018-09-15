use std::path::{PathBuf,Path};
use std::fs::File;
use std::io::*;

use query::{Request,Response};

pub fn rule_data(roule: &str, req: Request) -> Response{
    if let Some(req_str) = req.query{
        let name =  req_str.get("name").unwrap();
        let content = name.to_string();
        Response::new(200, "text/html",content)
    }else{

        Response::html_404_body()
    }    
}

pub fn rule_data_app(roule: &str, req: Request) -> Response{
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
