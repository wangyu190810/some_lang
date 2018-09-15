fn main() {
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::thread;

    let mut num:i32  = 0;
    while true {
        num = num + 1;
        println!("num {}", num);
        thread::spawn(
            move || {
                {
                    let mut stream = TcpStream::connect("58.87.86.221:6142").unwrap();

                    // ignore the Result
                    let _ = stream.write(b"name\r\n");
                    let _ = stream.write(b"hello\r\n");

                    let _ = stream.read(&mut [0; 128]); // ignore here too
                } // the stream is closed here
            }, // some work here
        );
    }
}
