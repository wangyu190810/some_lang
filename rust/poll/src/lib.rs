
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


use std::thread;
use std::marker::Send;

pub fn first_thread<F:Send + 'static>( func: F, nums: i32, string: String ) 
    where F: Fn(i32,String) {
        let mut nums = nums;
        let mut strings = string;
        let child  =  thread::spawn(move || func(nums, strings));
        child.join();
    }
   

// use poll::first_thread;

