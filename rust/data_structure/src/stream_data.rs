use std::io::prelude::*;
use std::net::TcpStream;

use packet::{Head, AddOddLotOrder, IndexDefinition,save_data};
use bytebuffer::ByteBuffer;

pub fn head() {
    let buff_status: u16 = 4096;
    let mut stream = TcpStream::connect("127.0.0.1:6101").unwrap();

    // ignore the Result
    // let _ = stream.write(&[1])
    while true {
        let mut buff = [0; 4096];
        let _ = stream.read(&mut buff); // ignore here too
        // println!("{:?}",buff);

        let head = Head::new(buff.to_vec());
        while true {
            let mut len = buff_status;
            let mut buffer = ByteBuffer::new();
            save_data(buff.to_vec(),&mut buffer);

            if(head.MsgSize > buff_status) {
                len += buff_status;   
                let _ = stream.read(&mut buff);
                save_data(buff.to_vec(),&mut buffer);
            }else{
                println!("msg size {}", head.MsgSize);
        let msg_type = head.MsgType;
        if (msg_type == 33) {
            // let mut body_buff = [0;4092];
            // let _ = stream.read(&mut body_buff );
            // println!("{:?}",body_buff);
            // let mut packet_buff = buff.to_vec().extend(body_buff.iter().cloned());
            // println!("{:?}",packet_buff);
            let addoddlotorder = AddOddLotOrder::new(buff.to_vec());
            println!("{:?}", addoddlotorder);

        } else if (msg_type == 70) {
            let indexdefinition = IndexDefinition::new(buff.to_vec());
            println!("{:?}", indexdefinition);
        }
        println!("ms type {}", head.MsgType);
                break
            }


        }



        // println!("{:?}",buff);

        
    }
} // the stream is closed here
