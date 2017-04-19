use std::io;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};


fn stdinput(){

let stdin = io::stdin();
for line in stdin.lock().lines(){
            match line{
                Ok(info) => match info.as_str(){
                    "save" =>{
                    } 
                    _ => {
                        println!{"info is {}", info}
                    }

                },
                Err(e) => {
                    println!("{:?}",e);
                }
             }
            //  queue.save();
            // println!("{}", line.unwrap());
        }
}

pub fn fibonacci(i: i32) -> i32{
    if(i<2) {
        return i
    };
    return fibonacci(i-2) + fibonacci(i-1);
}

pub  fn fib_test(){
    
    let start = SystemTime::now();
    println!("{}",fibonacci(32));
    let end = SystemTime::now();
    println!("{:?}",start );
    println!("{:?}",end );

} 