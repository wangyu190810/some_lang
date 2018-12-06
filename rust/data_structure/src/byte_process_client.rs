use std::io::prelude::*;
use std::net::TcpStream;

use bytebuffer::ByteBuffer;
use chrono::{Datelike, Timelike, Utc};
use packet::{
    save_data, AddOddLotOrder, Head, IndexDefinition, LevelPrice, LevelPriceFiller, NominalPrice,
    NowPrice, PackHead,
};
use utils::{
    Statistics,
    Msg,
    BrokerQueue,
    FillerData,
    IndicativeEquilibriumPrice,
    ReferencePrice,
    CloseingPrice,
    OrderImbalance
};
use std::time::SystemTime;
use std::time::{Duration, Instant};
use time::PreciseTime;

use std::collections::HashMap;

fn get_request(stream: &mut TcpStream, r: &mut Vec<u8>) {
    const CHUNCK_SIZE: usize = 4096;
    let mut buf = [0; CHUNCK_SIZE];
    while let Ok(len) = stream.read(&mut buf) {
        r.extend_from_slice(&buf[..len]);
        if len != CHUNCK_SIZE {
            return;
        }
    }
}

pub fn full_package() {
    static buff_status: usize = 4096;
    println!("start");
    // let mut stream = TcpStream::connect("192.168.1.11:6101").unwrap();
    let mut stream = TcpStream::connect("218.253.82.29:6105").unwrap();
    // let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    // ignore the Result
    // let _ = stream.write(&[1])
    let mut content = Vec::new();
    let mut buf_len = 0;
    let mut buff = [0; 1024];
    let mut pkg_len = 0;

    let mut PktSize = 0;
    let mut SendTime: u64 = 0;
    let mut MsgCount = 0;
    let mut SeqNum = 0;
    let mut Filler = String::new();
    let mut offset = 0;
    let mut MsgSize = 0;
    let mut data = buff.clone();
    let mut nums = 0;
    let mut request_str = Vec::new();
    let start = PreciseTime::now();
    let mut now_time = Instant::now();
    let mut msg_count = HashMap::new();
    loop {
        if (buf_len == 0) {
            // stream = TcpStream::connect("192.168.1.11:6101").unwrap();
            content = Vec::new();
            buf_len = 0;
            // stream.read(&mut buff);

            data = buff.clone();
            request_str = Vec::new();
            get_request(&mut stream, &mut request_str);
        } else {
            request_str = Vec::new();
            get_request(&mut stream, &mut request_str);
            // content.extend(data.to_vec());
        }
        let data = request_str.clone();
        content.extend(data);
        buf_len = content.len();
        // println!("content len start:{}", buf_len);
        // // let sys_time = SystemTime::now();
        // println!("{:?}", nums);
        // let start_time = sys_time.tv_sec;
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // let now = Utc::now();
        // let start_time = now.second();

        // whatever you want to do

        if (1 == nums / 10000) {
            // let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            // let now = Utc::now();
            // let end_time = now.second();
            // let end = PreciseTime::now();
            // let cost = start.to(end);

            println!("cost time :{}", now_time.elapsed().as_secs());
            println!("msg_count {:?}",msg_count);
            now_time = Instant::now();
            // msg_count = HashMap::new();
            nums = 0;

            // println!(
            //     "process {} msg  end_time {} - start_time {} times {}",
            //     nums,
            //     end_time,
            //     start_time,
            //     cost.num_seconds()
            // );
            // break;
            // let sys_time = SystemTime::now();
            // println!("{:?}",sys_time);
        }
        loop {
            // println!("near loop");
            buf_len = content.len();
            if buf_len < 16 {
                // println!("buf_len < 16");
                break;
            }
            // header = get_packet_header(buf[:16])
            let packtitle = content.to_vec().split_off(16);
            let mut pack = PackHead::new(content.to_vec());
            //    pack = PackHead::new(buff.to_vec());
            PktSize = pack.PktSize;
            MsgCount = pack.MsgCount;
            SendTime = pack.SendTime;

            Filler = pack.Filler;
            SeqNum = pack.SeqNum;
            if buf_len < PktSize as usize {
                // println!("buf_len {} < PktSize {}", buf_len, PktSize);
                break;
            }
            if MsgCount == 0 {
                // buf = buf[header['PktSize']:]
                content = content.split_off(PktSize as usize);
                // println!("MsgCount == 0 PktSize {}", PktSize);
                break;
            }

            // data = Head::new(data.to_vec().split_off(PktSize as usize));
            let mut content_clone = content.clone();
            let mut content_body = content_clone.split_off(16);
            // println!("content len {}",content.len());
            let mut content_head = content_body.split_off(4);
            let body_head = Head::new(content_body);
            // println!("content len start {} PktSize {} ",content.len(),PktSize);
            //  println!("content len {}",content.len() - PktSize as usize);
            let x = body_head.MsgType;
            let counter = msg_count.entry(x).or_insert(0);
            *counter += 1;
            let _ = match x {
                53 => {
                    LevelPrice::new(content_head, 1);

                    // return;
                }
                40 => {
                    NominalPrice::new(content_head);
                    // return;
                }
                41 =>{
                    IndicativeEquilibriumPrice::new(content_head);
                }
                43 =>{
                    ReferencePrice::new(content_head);
                }
                
                54 =>{
                    BrokerQueue::new(content_head);
                }
                56 =>{
                    OrderImbalance::new(content_head);
                }
                60 =>{
                    Statistics::new(content_head);
                }
                62 =>{
                    CloseingPrice::new(content_head);
                }
                // 40 => "something else",
                _ => {
                    // "";
                    // return;
                }
            };

            content = content.split_off(PktSize as usize);

            // println!("content len end {}",content.len());
            // // 结构体的数据，都要消息大小和消息类型
            // println!("MsgSize :{}", body_head.MsgSize);

            // println!("MsgType :{}", body_head.MsgType);
            //  println!("content len :{}", content.len());
            nums += 1;
            // if data :
            //     total += 1
            //     data['SeqNum'] = header['SeqNum']
            //     data['SendTime'] = header['SendTime']/1000000
            //     self.handle(data)
            // content =  content.to_vec().split_off(PktSize as usize);
        }
    }
    // let end = PreciseTime::now();
    // let cost = start.to(end);
    println!("cost time :{}", now_time.elapsed().as_secs())
}
