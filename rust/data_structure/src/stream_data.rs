use std::io::prelude::*;
use std::net::TcpStream;

use bytebuffer::ByteBuffer;
use packet::{save_data, AddOddLotOrder, Head, IndexDefinition, NominalPrice, NowPrice, PackHead, LevelPrice};

pub fn head() {
    static buff_status:usize = 4096;

    let mut stream = TcpStream::connect("58.87.86.221:6101").unwrap();
    // let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    // ignore the Result
    // let _ = stream.write(&[1])
    let mut body = Vec::new();
    let mut pkt_size = 0;
    let mut buff = [0; 4096];
    loop {
        
        let _ = stream.read(&mut buff); // ignore here too
                                        // println!("{:?}",buff);
        let data = buff.clone();

        // println!("{:?}", data.to_vec());

        let pack = PackHead::new(buff.to_vec());
        // println!("{}", pack);
        // println!("{}", pack.PktSize);
        // println!("{}", pack.SendTime);

        if (body.is_empty()) {
            pkt_size = pack.PktSize
        }
        // if (body.len() < (pack.PktSize as usize)) {
        body.extend(data.to_vec());
        buff = [0; 4096];
        if (body.len() >= pkt_size as usize) {
            let body_clone = body.clone();
            let body_pack = body.split_off(16);
            let body_pack_clone = body_pack.clone();
            let body_head = Head::new(body_pack_clone);
            // println!("MsgType :{}", body_head.MsgType);
            // println!("MsgSize :{}", body_head.MsgSize);

            if (body_head.MsgType == 11){
                 let body_data = NowPrice::new(body_pack);
                // println!("{:?}", &body_pack_clone);
                // println!("{}", body_data.MsgSize);
                // println!("{}", body_data.MsgType);
                // println!("{}", body_data.SecurityCode);
                // println!("{}", body_data.MarketCode);
            // break;
            // if (body_data.MsgType == 11) {
                // let body_pack_clone = body_pack.clone();
                // let body_data = NowPrice::new(body_pack);
                // // println!("{:?}", &body_pack_clone);
                println!("{}", body_data.MsgSize);
                println!("{}", body_data.MsgType);
                println!("{}", body_data.SecurityCode);
                println!("{}", body_data.MarketCode);
                
            // }
            }else if (body_head.MsgType == 53){
                 println!("MsgType :{}", body_head.MsgType);
                 let filler_nums = (pkt_size - 8) / 24;
                 let body_data  = LevelPrice::new(body_pack,filler_nums as u8);
                 println!("{}", body_data);
                 
            }else if (body_head.MsgType == 62){
                println!("MsgType :{}", body_head.MsgType);
                 
            }else if (body_head.MsgType == 40){
                 println!("else MsgType :{}", body_head.MsgType);
            }else{
                println!("error MsgType :{}", body_head.MsgType);
            }
           
            body.clear();
        }
    }
}

// let head = Head::new(buff.to_vec());
// println!( "{}", head.MsgSize);
// println!( "{}", head.MsgType);

//     while true {
//         let mut buff = [0; 4096];
//         let _ = stream.read(&mut buff); // ignore here too
//                                         // println!("{:?}",buff);

//         let head = Head::new(buff.to_vec());

//         while true {
//             let mut len = buff_status;
//             let mut buffer = ByteBuffer::new();
//             save_data(buff.to_vec(), &mut buffer);

//             if (head.MsgSize > buff_status) {
//                 len += buff_status;
//                 let _ = stream.read(&mut buff);
//                 save_data(buff.to_vec(), &mut buffer);
//             } else {
//                 println!("msg size {}", head.MsgSize);
//                 println!("msg type {}", head.MsgType);
//                 let msg_type = head.MsgType;
//                 if (msg_type == 33) {
//                     // let mut body_buff = [0;4092];
//                     // let _ = stream.read(&mut body_buff );
//                     // println!("{:?}",body_buff);
//                     // let mut packet_buff = buff.to_vec().extend(body_buff.iter().cloned());
//                     // println!("{:?}",packet_buff);
//                     let addoddlotorder = AddOddLotOrder::new(buff.to_vec());
//                     println!("{:?}", addoddlotorder);
//                 } else if (msg_type == 70) {
//                     let indexdefinition = IndexDefinition::new(buff.to_vec());
//                     println!("{:?}", indexdefinition);
//                 } else if (msg_type == 40) {
//                     let nominalprice = NominalPrice::new(buff.to_vec());
//                     println!("{:?}", nominalprice.MsgSize);
//                 }
//                 println!("ms type {}", head.MsgType);
//                 break;
//             }
//         }

//         // println!("{:?}",buff);
//     }
// } // the stream is closed here
