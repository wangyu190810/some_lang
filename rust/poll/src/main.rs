extern crate poll;
use poll::first_thread;


fn print_data(data : i32){
    println!("{}",data );
}


fn print_data_tow(data : i32, string: String){
    println!("{}",data );
    println!("{}",string );
}
fn hello(){
    println!("{}", "hello")
}


fn main() {
    // let mut a = String::new("a");
    let a = String::from("adsf");
    //  first_thread(print_data,123);
    let child = first_thread(print_data_tow,123, a);
    // child.join();
    // first_thread(hello);
}