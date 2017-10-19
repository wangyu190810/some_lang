
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::collections::LinkedList;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
pub fn run_channel(){


let data = Arc::new(Mutex::new(0));

let (tx, rx) = channel();
for _ in 0..N {
    let (data, tx) = (data.clone(), tx.clone());
    thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let mut data = data.lock().unwrap();
        *data += 1;
        if *data == N {
            tx.send(()).unwrap();
        }
        // the lock is unlocked here when `data` goes out of scope.
    });
}

rx.recv().unwrap();

}

pub fn run_clone_data(){
    
    let mut data: LinkedList<i32> = LinkedList::new();
    let mut clone_data = data.clone();
    let end_data = data.clone();
    {
        let mut clone_first = data;
        clone_first.push_back(123);   
    }
    {

        let mut clone_second = clone_data;
        clone_second.push_back(1);   
    }
    println!("{:?}",end_data);

}