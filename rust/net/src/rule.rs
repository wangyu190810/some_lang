#![allow(unused)]
#![feature(fnbox)]

use std::boxed::FnBox;
use std::collections::HashMap;
use query::{Request,Response};

trait FnBoxArgs{
    fn call_box(self:Box<Self>, request:Request) ->Response;
}


impl<F:FnOnce(Request) -> Response > FnBoxArgs for F  {
    fn call_box(self:Box<F>,request:Request) ->Response{
        let resp = (*self)(request);
        let content = String::from("hello");
        Response::new(200, "text/html",content)
        
    }
    
}



fn call_back(args:i32)->i32{
    args
}


fn make_map() -> HashMap<i32, Box<FnBox() -> i32>> {
    let mut map: HashMap<i32, Box<FnBox() -> i32>> = HashMap::new();
    map.insert(1, Box::new(move || 44));
    map.insert(2, Box::new(move || 44));
    map
}

fn make_map_data() ->HashMap<String, Box<FnBox() -> i32>> {}



pub fn test_make_map() {
    let mut map = make_map();
    for i in &[1, 2] {
        let f = map.remove(&i).unwrap();
        assert_eq!(f(), i * 22);
    }
}