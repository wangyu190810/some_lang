#![feature(custom_derive, plugin)]
#![feature(rustc_private)]

extern crate data_structure;
extern crate  bytebuffer;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate redis_client;
extern crate serialize;

#[macro_use]
extern crate serde_derive;

// use data_structure::tree;
mod tree;
mod ringqueue;
mod test_some;
mod process_byte;
mod packet;
mod JsonData;
mod RedisTest;

fn main() {
    // data_structure:: tree::run();
      // tree::run();
      // ringqueue::run();
      // test_some::run();
      // test_some::run_clone_data();
    
    
      // process_byte::test_cast();
      // process_byte::test_pack_unpack();
    
      // let mut data = {"length":5,"id":1,"msg_type":1,"msg":"abc"};
      // let mut data = "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}".to_string();
      // JsonData::str_data(data);
      RedisTest::set_and_get();

}