#![feature(fnbox)]
use std::boxed::FnBox;
#![feature(unboxed_closures)]

use std::cmp::PartialOrd;
use std::collections::HashMap;

trait HandlerInit {
    fn first() -> String {
        return String::from("index");
    }
}

trait FnBox {
    fn call_box(self: Box<Self>, args: A);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

fn test() {
    println!("test{}", 1);
}

type hander_func = Box<FnBox + Send + 'static>;

struct Handler {
    roule:HashMap<String, Box<FnBox() -> String>>
    // roule: String,
    // func: hander_func,
}



struct HandlerList{
    hander: Box<Handler>,
}



impl Handler {
    pub fn test(func: Box<FnBox() -> String>) ->Handler {
        println!("init handler");
        let mut map: HashMap<i32, Box<FnBox() -> i32>> = HashMap::new();
        let new_str: String = String::from("/");
        map.insert(new_str,func)
        Handler {
            roule: map
        }
    }
}



// impl HandlerList {
//     pub fn test(func: hander_func) {
//         println!("init handler");
//         let new_str: String = String::from("/");
//         let hander = Handler {
//             roule: new_str,
//             func: func
//         };
//         HandlerList{
//             hander:
//         }

//     }
// }
