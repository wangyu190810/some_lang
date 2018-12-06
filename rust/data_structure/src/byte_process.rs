use std::io::prelude::*;
use std::net::TcpStream;

use bytebuffer::ByteBuffer;
use packet::{
    save_data, AddOddLotOrder, Head, IndexDefinition, LevelPrice, NominalPrice, NowPrice, PackHead,
};

pub fn full_package() {
    static buff_status: usize = 4096;
    let mut stream = TcpStream::connect("192.168.1.11:6101").unwrap();
    // let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    // ignore the Result
    // let _ = stream.write(&[1])
    let mut content = Vec::new();

    let mut pkt_size = 0;
    let mut buf_len = 0;
    let mut pkg_len = 0;
    let mut SendTime: u64 = 0;
    let mut MsgCount = 0;
    let mut SeqNum = 0;
    let mut Filler = String::new();
    let mut offset = 0;
    let mut MsgSize = 0;
    let mut buff = [0; 1024];
    let mut nums = 0;
    loop {
        let _ = stream.read(&mut buff);
        let data = buff.clone();
        content.extend(data.to_vec());
        buf_len = content.len();
        if (buf_len == 0) {
            stream = TcpStream::connect("192.168.1.11:6101").unwrap();
            content = Vec::new();
            buf_len = 0;
        }
        
        buff = [0; 1024];
        while (content.len() >= pkt_size as usize) {
            println!("end_content {:?}", content.len());
            let mut pack = PackHead::new(buff.to_vec());
            pkt_size = pack.PktSize;
            println!("pkt_size{}", pkt_size);
            if (pkt_size == 0) {
                if (buf_len < 16) {
                    println!("buf_len{}", buf_len);
                    break;
                } else {
                    pack = PackHead::new(buff.to_vec());
                    pkt_size = pack.PktSize;
                    MsgCount = pack.MsgCount;
                    SendTime = pack.SendTime;

                    Filler = pack.Filler;
                    SeqNum = pack.SeqNum;
                    buf_len = buf_len - 16;
                    offset = offset + 16;
                    if pkt_size < 16{
                        break;
                    }
                    pkt_size = pkt_size - 16;
                    println!("pkt_size{}", pkt_size);
                    
                }
            }else{
                break;
                return;
            }

            // SendTime =
            if (MsgCount == 0) {
                if (buf_len < pkt_size as usize) {
                    offset = offset + buf_len;
                    pkt_size = pkt_size - buf_len as u16;
                    buf_len = 0;
                    break;
                } else {
                    offset = offset + pkt_size as usize;
                    // pkt_size = pkt_size  - buf_len as u16;
                    buf_len = buf_len - pkt_size as usize;
                    pkt_size = 0;
                    continue;
                }
            }
            while MsgCount > 0 {
                // 还没有解析过当前消息头,则解析消息头
                if MsgSize == 0 {
                    if buf_len < 4 {
                        break;
                        // 缓存区长度不够一个完整的消息头，去接收消息
                    }

                    let mut body_clone = content.clone();
                    // 去掉16个字符，返回值为剩余的数据。
                    let mut body_pack = body_clone.split_off(16);
                    // 将获取的数据clone
                    let body_pack_clone = body_pack.clone();
                    // 处理获取的数据结构体
                    let body_head = Head::new(body_pack_clone);
                    // 结构体的数据，都要消息大小和消息类型
                    println!("MsgSize{}", body_head.MsgSize);

                    println!("MsgType :{}", body_head.MsgType);
                }
            }

            let mut body_clone = content.clone();
            // 去掉16个字符，返回值为剩余的数据。
            let mut body_pack = body_clone.split_off(16);
            // 将获取的数据clone
            let body_pack_clone = body_pack.clone();
            // 处理获取的数据结构体
            let body_head = Head::new(body_pack_clone);
            // 结构体的数据，都要消息大小和消息类型
            println!("MsgSize{}", body_head.MsgSize);

            println!("MsgType :{}", body_head.MsgType);
            println!("");
            nums += 1;
            if (nums > 10) {
                println!("end_content {:?}", content.len());
                // break;
            }
            // body = body.split_off(pkt_size as usize)
            content = content.split_off(pkt_size as usize);
            let end_content = content.clone();
            // println!("content{:?}",content);

            let mut pack = PackHead::new(end_content);
            pkt_size = pack.PktSize;
            if (pkt_size == 0) {
                content.len() < 16;
                break;
            }
            println!("pkt_size {}", pkt_size);

            if (nums > 10) {
                println!("end_content {:?}", content.len());
                break;
            }
            if (content.is_empty()) {
                let mut pack = PackHead::new(buff.to_vec());
                pkt_size = pack.PktSize;
                SendTime = pack.SendTime;
            } else if (pkt_size == 0) {

            }
            println!("SendTime{}", SendTime);
            println!("pkt_size {}", pkt_size);
        }
    }
}
