use bytebuffer::ByteBuffer;
use time;

pub fn test_cast() {
    let mut wt_buffer = ByteBuffer::new();
    wt_buffer.write_u16(1); // buffer contains [0x00, 0x1] if little endian
    println!("{:?}", wt_buffer);
    let mut rd_buffer = ByteBuffer::from_bytes(&vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1]);
    let value = rd_buffer.read_u64(); //Value contains 1
    println!("{:?}", value);
    let mut buffer = ByteBuffer::new();
    // buffer.write_string("Helloasd");

    // buffer.write_string("Hello");
    let sparkle_heart = "Hello123123asdfasdfas".to_string();

    let bytes = sparkle_heart.into_bytes();
    let length = bytes.len();
    println!("read_bytes {:?}", length);
    buffer.write_bytes(&bytes);

    // read_bytes(&mut self, size: usize) -> Vec<u8>
    // println!("read_bytes {:?}",  buffer.read_string());
    println!(
        "read_bytes {:?}",
        String::from_utf8(buffer.read_bytes(9)).unwrap()
    );
    println!(
        "read_bytes {:?}",
        String::from_utf8(buffer.read_bytes(12)).unwrap()
    );
}

//  args = pack('<HB1sLQHHL3sBQlLHBB4s', 52, 1, 'b', 1200,
//      12345678905050, 36, 53, 123988, 'aaa', 1, 150000, 160000, 2, 0, 1, 1, 'xxxx')
//     stime = time.time()
//     for i in range(10000) :
//         get_packet_data(args)
//     args = pack('<HB1sLQHHL3sBQlLHBB4s', 52, 1, 'b', 1200,
//      12345678905050, 36, 53, 123988, 'aaa', 1, 150000, 160000, 2, 0, 1, 1, 'xxxx')

#[derive(Debug, Clone)]
pub struct TestCase {
    length: u64,
    id: u32,
    msg_type: i8,
    msg: String,
}

impl TestCase {
    pub fn new(id: u32, msg_type: i8, msg: String, length: u64) -> TestCase {
        TestCase {
            id: id,
            msg_type: msg_type,
            msg: msg,
            length: length,
        }
    }

    // fn length(self){
    //     let mut msg_length:u64;

    //     self.length = self.msg.len() as u64;
    // }

    pub fn pack(self) -> ByteBuffer {
        let bytes = self.msg.into_bytes();
        let length = bytes.len() as u64;
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u64(self.length + length);
        wt_buffer.write_u32(self.id);
        wt_buffer.write_i8(self.msg_type);
        wt_buffer.write_bytes(&bytes);
        return wt_buffer;
    }

    pub fn unpuck(rd_buffer: ByteBuffer) -> TestCase {
        let mut rd_buffer = rd_buffer;
        TestCase {
            length: rd_buffer.read_u64(),
            id: rd_buffer.read_u32(),
            msg_type: rd_buffer.read_i8(),
            msg: rd_buffer.read_string(),
        }
    }
    // add code here
}




pub fn unpuck(rd_buffer: ByteBuffer) -> TestCase {
    let mut rd_buffer = rd_buffer;
    let pack_length = rd_buffer.read_u64();
    let msg_length = pack_length - 13;
    TestCase {
        length: pack_length,
        id: rd_buffer.read_u32(),
        msg_type: rd_buffer.read_i8(),
        msg: String::from_utf8(rd_buffer.read_bytes(msg_length as usize)).unwrap(),
    }
}
pub fn test_pack_unpack() {
    let Time = time::now();
    println!("{:?}", time::now().rfc822z());
    println!("{:?}", time::strftime("%a, %d %b %Y %T %z", &Time));

    for _ in (0..1000000) {
        let id = 1;
        let msg_type = 1;
        let msg = "abc".to_string();
        let length = (id.to_string().len() + msg_type.to_string().len() + msg.len()) as u64;
        // println!("{}", length);
        let testcast = TestCase::new(id, msg_type, msg, length);
        let clone_data = testcast.clone();
        let mut pack_data = testcast.pack();
        // println("{}")
        // println!("orig {}",clone_data.msg);
        // println!("unpack {:?}",unpuck(pack_data).msg)
    }
    // println!("{:?}",time::now());
    let Time_2 = time::now();
    println!("{:?}", time::strftime("%a, %d %b %Y %T %z", &Time_2));
    // println!("{:?}",Time.tm_nsec - Time_2.tm_nsec);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        // let id = 1;
        // let msg_type = 1;
        // let msg = "abc".to_string();
        // let length = 100;
        // let testcast = TestCase::new(id, msg_type, msg, length);
        // let clone_data = testcast.clone();
        // let mut pack_data = testcast.pack();
        // // println("{}")
        // println!("{}{}", clone_data.msg, unpuck(pack_data).msg)

        // assert_eq!(clone_data.id, unpuck(pack_data).id)
        // assert_eq!(clone_data.msg, unpuck(pack_data).msg)
    }

}
