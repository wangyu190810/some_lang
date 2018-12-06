#![feature(alloc)]
#![allow(unused)]
#![feature(fnbox)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(associated_type_defaults)]
extern crate alloc;

use std::boxed::FnBox;
use std::collections::HashMap;



// pub struct Handler<T> {
//     rule: String,
//     func: T,
// }

type A = String;

trait FnBoxArgs<A>{
    type Output;
    fn call_box(self:Box<Self>, args: A) ->Self::Output;
}

impl<F:FnOnce(A)> FnBoxArgs<A> for F  {
    type Output = String;
    fn call_box(self:Box<F>, args: A)->Self::Output{
        (*self)(args);
        return String::from("/");
    }
    
}



pub struct RuleH<T> {
    rule: String,
    func: T,
}



// impl<T:FnBoxArgs(A),K> FnBoxArgs<A> for RuleH<T>  {
//     type Output =K ;

//     fn call_box(self.Box<Self>, args: A)->Self::Output{
//         (*self.func)(args)
//         // (*self)(args)
//     }

   

    
// }

fn make_rule_args<T>(mut list: Vec<RuleH<Box<FnBoxArgs(A) ->T>>>, rule: RuleH<Box<FnBoxArgs(A) -> T>> )  
-> Vec<RuleH<Box<FnBoxArgs(A) -> T>>>{
    list.push(rule );
    list
}








pub struct Rule<T> {
    rule: String,
    func: T,
}

// impl Rule<> {
//     fn new(){

//     }
// }

// struct F;


fn make_rule(mut list: Vec<Rule<Box<FnBox() -> i32>>>,rule: Rule<Box<FnBox() -> i32>> )  -> Vec<Rule<Box<FnBox() -> i32>>>{

    list.push(rule );
    list
}

fn make_list() -> Vec<Box<FnBox() -> i32>>{
    let mut list: Vec<Box<FnBox() -> i32>> = Vec::new();
    list.push(Box::new(|| 44) );
    list
}


fn make_map() -> HashMap<i32, Box<FnBox() -> i32>> {
    let mut map: HashMap<i32, Box<FnBox() -> i32>> = HashMap::new();
    map.insert(1, Box::new(|| 22));
    map.insert(2, Box::new(|| 44));
    map
}

pub fn test_func(args:String) -> String{
    return args
}

pub fn test_make_rule_args(args: String) {
    type T = (String,);
    let mut list:Vec<RuleH<Box<FnBoxArgs(A) ->T>>> = Vec::new() ;
    // let func : Vec::new(Box::new(|| 44))
    let mut rule_data = RuleH{
        rule: String::from("/"),
        func: Box::new(|args|test_func::<T>) as Box<FnBoxArgs(A) ->(String,) >
    };
    // print!("{:?}",rule_data);
    let handler_list = make_rule_args(list, rule_data);
    for f in handler_list{
        let func:Box<FnBoxArgs(A) ->T> = f.func;
        let nums = func.call_box((args,));
        // println!("num is :{}",nums);
    }

}

pub fn test_make_rule(){
    let mut list:Vec<Rule<Box<FnBox() -> i32>>> = Vec::new() ;
    // let func : Vec::new(Box::new(|| 44))
    let mut rule_data = Rule{
        rule: String::from("/"),
        func: Box::new(|| 44) as Box<FnBox() -> i32>
    };
    // print!("{:?}",rule_data);
    let handler_list = make_rule(list, rule_data);
    for f in handler_list{
        let func = f.func;
        let nums = func();
        println!("num is :{}",nums);
    }
}


pub fn test_make_list(){
    
    let mut list = make_list();
    for f in list{
        f();
    }
    
}



pub fn test_make_map() {
    let mut map = make_map();
    for i in &[1, 2] {
        let f = map.remove(&i).unwrap();
        assert_eq!(f(), i * 22);
    }
}

fn main() {
    println!("Hello, world!");
    test_make_map();
    test_make_list();
    test_make_rule();
}
