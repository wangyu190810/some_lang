#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate tokio_core;
extern crate futures;

use std::net::TcpListener;

pub mod basequeue;
pub mod base;
pub mod test;
pub mod net;
pub mod rpcbase;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}



pub struct Queues{
    items:String
}


#[derive(Debug)]
pub struct Header {
  version: u8,      // 2 bits, RTP version field must equal 2. 
  padding: u8,      // 1 bit
  marker : u8,      // 1 bit
  pt     : u8,      // 7 bits, TODO: use enum.
  seq    : u16,     // 16 bits
  ts     : u32,     // 32 bits
  ssrc   : u32,     // 32 bits, SHOULD be random
  // CC( CSRC Count ): u8,   // 4 bits, The CSRC count contains the number of CSRC identifiers that follow the fixed header.
  csrc      : Vec<u32>,      // 32 bits, 0 to 15 items ( The number of identifiers is given by the CC field),
  // extension : Option<Extension> // Extension Header
}




#[derive(Debug)]
pub struct StrHeader {
  version: u8,      // 2 bits, RTP version field must equal 2. 
  timestamp: i32,      // 1 bit
  msg : u8,      // 1 bit
  pt     : u8,      // 7 bits, TODO: use enum.
  seq    : u16,     // 16 bits
  ts     : u32,     // 32 bits
  ssrc   : u32,     // 32 bits, SHOULD be random
  // CC( CSRC Count ): u8,   // 4 bits, The CSRC count contains the number of CSRC identifiers that follow the fixed header.
  csrc      : Vec<u32>,      // 32 bits, 0 to 15 items ( The number of identifiers is given by the CC field),
  // extension : Option<Extension> // Extension Header
}



// impl Queues {
     
//      fn new(T: Vec<T>){
//          Queues{
//              items: T
//          }
//      }

//      fn insert(&self,T: Vec<T>){
//          self.items.push()
//      }
// }

