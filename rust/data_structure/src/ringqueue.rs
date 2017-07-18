use std::thread;
use std::sync::{Arc, Mutex};

const SIZE: usize = 1000;

#[derive(Debug)]
struct RingQ{

   head : i32,
   tail:  i32,
   tag: i32,
   size: usize,
   space : Vec<i32>,

}

pub trait Ringqimp {
    fn ringq_free<'a>(&mut self);
    fn ringq_push<'a>(&mut self, ringq: i32);
    fn ringq_poll<'a>(&mut self, ringq: i32);
    fn new() -> RingQ;
    fn ringq_print<'a>(&mut self);
    // add code here

    // add code here
}


impl Ringqimp   for RingQ{

    fn new() -> RingQ{
        RingQ{
        head : 0,
        tail : 0,
        tag : 0,
        size : SIZE,
        space: Vec::new()
       }   
    }

    fn ringq_free<'a>(&mut self){
        RingQ{
        head : 0,
        tail : 0,
        tag : 0,
        size : SIZE,
        space: Vec::new()
       };  
    }

    fn ringq_print<'a>(&mut self){
        println!("print ringq data {:?}",self);
    }


    fn ringq_push(&mut self, data :i32 ){
        if (self.size == self.space.len()){
            println!("queue is full");
            return 
        }
        self.tail += 1;
        self.space.push(data);
        if (self.size == self.space.len()){
            println!("queue is full");
            self.tag = 1;
        }
        println!("{:?}", self);
    }

    fn ringq_poll(&mut self, data :i32){
        if (self.space.len() == 0 ){
            println!("queue is free");
            self.tag = 0;
        }
        self.tail -= 1;
        self.space.pop();
        if (self.space.len() == 0 ){
            println!("queue is free");
            self.tag = 0;
        }
    }

}

pub fn run(){
    let  mut ringq = Arc::new(Mutex::new(RingQ::new()));
    for  row  in 1..10{
        let mut ringq_clone = ringq.clone();
        thread::spawn(move || {
            let mut ringq_push = ringq_clone.lock().unwrap();
            {
                for val in 10..15{
                    ringq_push.ringq_push(row * val);
                    if (ringq_push.space.len() == 45){
                        ringq_push.ringq_print()
                    }
                }
            }
        }).join().expect("thread::spawn failed");
    }
    // for  row  in 1..10{
    //     thread::spawn(move || {
    //         for val in 1..10{
    //             let mut rinq_data = ringq_clone.lock().unwrap();
    //             rinq_data.ringq_push(val);
    //         }

    //     });
    // }
 
}