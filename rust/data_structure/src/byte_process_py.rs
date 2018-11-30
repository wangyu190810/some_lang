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
    loop {
        if (buf_len == 0) {
            // stream = TcpStream::connect("192.168.1.11:6101").unwrap();
            content = Vec::new();
            buf_len = 0;
            stream.read(&mut buff);
            data = buff.clone();
        } else {
            stream.read(&mut buff);
            data = buff.clone();
            // content.extend(data.to_vec());
        }
        if buf_len == 0 {
            let mut pack = PackHead::new(buff.to_vec());
            //    pack = PackHead::new(buff.to_vec());
            PktSize = pack.PktSize;
            MsgCount = pack.MsgCount;
            SendTime = pack.SendTime;

            Filler = pack.Filler;
            SeqNum = pack.SeqNum;
        }

        if PktSize > 0 {
            println!("PktSize{}", PktSize);
            println!("content{}", content.len());
            if PktSize <= 1024 {
                data.to_vec().split_off(PktSize as usize);
            }
            content.extend(data.to_vec());
            if content.len() <= PktSize as usize {
                continue;
            } else {

            }
        }

        buf_len = content.len();
        
        buff = [0; 1024];
        if buf_len >= 0 {
            while true {
                let mut content_clone = content.clone();
                
                if content_clone.len() < 16{
                    break;
                }
                let mut pack = PackHead::new(content_clone);
                //    pack = PackHead::new(buff.to_vec());
                PktSize = pack.PktSize;
                if (PktSize == 0){
                    break
                }
                if(buf_len > PktSize as usize ){
                    // println!("{}{}",PktSize,PktSize);
                    break
                }
                MsgCount = pack.MsgCount;
                SendTime = pack.SendTime;

                Filler = pack.Filler;
                SeqNum = pack.SeqNum;

                content = content.split_off(PktSize as usize);
                buf_len = content.len();
                println!("buf_len {:?}", buf_len);
            }
        }
    }
}

//         while (content.len() > PktSize as usize) {

//             // 没有未解析完的包
//             let mut content_clone = content.clone();
//             let body_content_clone = content.clone();
//             let mut body_content_clone = content.clone();
//             if PktSize == 0{
//                 // 剩余消息中没有一个完整的包头（包头长度16），去接收消息
//                 if buf_len < 16{
//                     break
//                 }
//                 // 解析包头
//                  println!("end_content {:?}", content.len());
//                  content = body_content_clone.split_off(16);
//                 let mut pack = PackHead::new(body_content_clone);
//                 //    pack = PackHead::new(buff.to_vec());
//                     PktSize = pack.PktSize;
//                     MsgCount = pack.MsgCount;
//                     SendTime = pack.SendTime;

//                     Filler = pack.Filler;
//                     SeqNum = pack.SeqNum;

//                 SendTime = SendTime / 1000000;
//                  content = content_clone.split_off(PktSize as usize);
//                   println!("PktSize{}", PktSize);
//                  break;
//             }else{
//                 body_content_clone = content.clone();
//                 content = body_content_clone.split_off(16);
//                 let mut pack = PackHead::new(body_content_clone);
//                 //    pack = PackHead::new(buff.to_vec());
//                     PktSize = pack.PktSize;
//                     MsgCount = pack.MsgCount;
//                     SendTime = pack.SendTime;

//                     Filler = pack.Filler;
//                     SeqNum = pack.SeqNum;

//                  println!("PktSize{}", PktSize);
//                 SendTime = SendTime / 1000000;
//                  content = content_clone.split_off(PktSize as usize);
//                  break;
//             }
//         }
//     }
// }

// 移动offset到消息头，并减少缓存区的长度
// buf_len, offset, PktSize = buf_len-16, offset+16, PktSize-16
//                 buf_len = buf_len - 16;
//                     offset = offset + 16;
//                     if PktSize < 16{
//                         break;
//                     }
//                     PktSize = PktSize - 16;
//             }
//             // # 包中的所有消息都已经解析完了，需要释放包中剩余的部分
//             // #（可能会出现包中有多余部分的情况）
//             if MsgCount == 0{
//                 // # 缓存区长度不够一个完整的包
//                 // if buf_len < PktSize:
//                 //     // # 剔除剩余的内容
//                 //     buf_len, offset, PktSize = 0, offset+buf_len, PktSize-buf_len
//                 //     break
//                 // else:
//                 //     buf_len, offset, PktSize = buf_len-PktSize, offset+PktSize, 0
//                 //     continue

//                 if (buf_len < PktSize as usize) {
//                     offset = offset + buf_len;
//                     PktSize = PktSize - buf_len as u16;
//                     buf_len = 0;
//                     break;
//                 } else {
//                     offset = offset + PktSize as usize;
//                     // pkt_size = pkt_size  - buf_len as u16;
//                     buf_len = buf_len - PktSize as usize;
//                     PktSize = 0;
//                     continue;
//                 }
//             }
//             // ## 解析当前包中的消息
//             while MsgCount > 0{

//                 // ## 还没有解析过当前消息头,则解析消息头
//                 if MsgSize == 0{
//                      // # 缓存区长度不够一个完整的消息头，去接收消息
//                     if buf_len < 4{
//                         break
//                     }
//                     // msg_func, MsgSize, MsgType = MSG_FUNCS.get(buffer[offset:offset+4], (None, 0, 0))
//                     body_content_clone = content.clone();
//                     content = body_content_clone.split_off(4);
//                     let body_head = Head::new(body_content_clone);
//                     // 结构体的数据，都要消息大小和消息类型
//                     println!("MsgSize{}", body_head.MsgSize);

//                     println!("MsgType :{}", body_head.MsgType);
//                     break;
//                     // if msg_func is None:
//                     //     // # 没有对应的处理方法，需要解析出消息头
//                     //     MsgSize, MsgType = unpack_from('<HH', buffer, offset)
//                 // ## 剩余缓存区中是否还包含完成的消息
//                 }

//                 if buf_len >= MsgSize{
//                     // if msg_func is not None:
//                     //     // ## 解析消息
//                     //     try:
//                     //         total += 1
//                     //         msg_data = msg_func( buffer, offset )
//                     //         msg_data['SeqNum'] = SeqNum
//                     //         msg_data['SendTime'] = SendTime
//                     //         data_handler( msg_data )
//                     //     except KeyboardInterrupt:
//                     //         sys.exit(0)
//                     //     except:
//                     //         pass
//                     // # 剔除已经解析完的消息

//                     // offset,buf_len,MsgCount,MsgSize,PktSize = offset+MsgSize,buf_len-MsgSize,MsgCount-1,0,PktSize-MsgSize
//                     offset = offset+MsgSize;
//                     buf_len = buf_len-MsgSize;
//                     MsgCount = MsgCount - 1;
//                     PktSize = PktSize-MsgSize as u16;
//                     MsgSize = 0;
//                     }
//                 else{

//                     break
//                 }
//             }
//             // # 消息还没有解析完表示缓存区中不足以满足一个消息，需要接收更多消息内容
//             if MsgCount > 0{
//                 break
//             }
//         if offset > 0{
//             content = content_clone.split_off(offset as usize);
//             offset = 0
//         }

//         }
//     }
// }
