use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::collections::LinkedList;
use std::sync::mpsc::{Sender, Receiver};

const SIZE: usize = 1000;

#[derive(Debug,Clone)]
struct RingQ{
   head : i32,
   tail:  i32,
   tag: i32,
   size: usize,
   space : LinkedList<i32>,

}

pub trait Ringqimp {
    fn ringq_free<'a>(&mut self);
    fn ringq_push<'a>(&mut self, ringq: i32);
    fn ringq_poll<'a>(&mut self, ringq: i32);
    fn new() -> RingQ;
    fn ringq_print<'a>(&mut self);
    fn ringq_data<'a>(&mut self) -> &mut RingQ;
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
        space: LinkedList::new(),
       }   
    }

    fn ringq_free<'a>(&mut self){
        RingQ{
        head : 0,
        tail : 0,
        tag : 0,
        size : SIZE,
        space: LinkedList::new(),
       };  
    }

    fn ringq_print<'a>(&mut self){
        println!("print ringq data {:?}",self);
    }

    fn ringq_data<'a>(&mut self) -> &mut RingQ{
        return self;
    }


    fn ringq_push(&mut self, data :i32 ){
        if (self.size == self.space.len()){
            println!("queue is full");
            return 
        }
        self.tail += 1;
        self.space.push_back(data);
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
            return 
        }
        
        self.tail -= 1;
        self.space.pop_front();
        if (self.space.len() == 0 ){
            println!("queue is free");
            self.tag = 0;
            return 
        }
    }

}

pub fn run(){
    let  mut ringq = Arc::new(Mutex::new(RingQ::new()));
    // let (tx, rx) = channel();
    // let (tx, rx): (Sender<RingQ>, Receiver<RingQ>) = channel();
    while (true){

        let row = 1;    
   
        // let (ringq_clone, tx) = (ringq.clone(), tx.clone());
        let ringq_clone = ringq.clone();
   
        thread::spawn(move || {

            let mut ringq_push = ringq_clone.lock().unwrap();
            {
                for val in 10..14{
                    ringq_push.ringq_push(row * val);
                    if (ringq_push.space.len() == 45){

                        ringq_push.ringq_print();
                     }
                }
            }
            // ringq_push.ringq_print();
            // tx.send(ringq_push.ringq_data()).unwrap();
        }).join().expect("thread::spawn failed");


        let ringq_clone = ringq.clone();
        thread::spawn(move || {

            let mut ringq_push = ringq_clone.lock().unwrap();
            {
                for val in 10..15{
                    ringq_push.ringq_poll(row * val);
                    // ringq_push.ringq_push(row * val);
                    if (ringq_push.space.len() == 45){
                            ringq_push.ringq_print();
                    }
                }
            }

            // tx.send(ringq_push.ringq_data()).unwrap();
        }).join().expect("thread::spawn failed");
    // tx.send(ringq_clone.ringq_data()).unwrap();
    
    };
    // for  row  in 1..10{
    //     let mut ringq_clone = ringq.clone();
    //     thread::spawn(move || {
    //         let mut ringq_push = ringq_clone.lock().unwrap();
    //         {
    //             for val in 10..15{
    //                 ringq_push.ringq_poll(0);
    //                 if (ringq_push.space.len() == 0){
    //                     ringq_push.ringq_print()
    //                 }
    //             }
    //         }
    //     }).join().expect("thread::spawn failed");
    // }
    // for  row  in 1..10{
    //     thread::spawn(move || {
    //         for val in 1..10{
    //             let mut rinq_data = ringq_clone.lock().unwrap();
    //             rinq_data.ringq_push(val);
    //         }

    //     });
    // }
 
}