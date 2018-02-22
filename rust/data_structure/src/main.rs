#![feature(custom_derive, plugin)]
#![feature(rustc_private)]

extern crate data_structure;
extern crate  bytebuffer;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate redis_client;
extern crate serialize;
extern crate redis;
extern crate byteorder;

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
mod stream_data;



fn set_u16_le(a: &mut [u8], v: u16) -> u16 {
    print!("{:?}",a);
    a[0] = v as u8;
    a[1] = (v >> 8) as u8;
    print!("{}",v);
    print!("{:?}",a);
    return v;
}


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
      // RedisTest::set_and_get();
      // RedisTest::do_something();
      stream_data::head();
      // let mut end:u16 = 0;
      //   let mut buff = [2,5];
      //   print!("{:?}",buff);
      //   end = set_u16_le( &mut buff ,end);
      //   println!("{}",end);
    //   let mut price = packet::Head::new([128, 0, 1, 33].to_vec());
    //     let clone = price.clone();
    //     let mut wt_buffer = price.pack();
    //     let head = packet::Head::unpack(wt_buffer);
    //         println!("{}",head.MsgSize);
    // println!("{}",head.MsgType);
        // assert_eq!(clone.MsgSize, unpack.MsgSize)
}