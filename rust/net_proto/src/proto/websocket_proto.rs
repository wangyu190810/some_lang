use std::io;
use std::str;
use bytes::{BytesMut, BufMut, IntoBuf, Buf};
use bytes::buf::FromBuf;
// use bytes::buf::FromBuf::from_buf;
use tokio_io::codec::{Encoder, Decoder};
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use base64;
use sha_1;
use sha1;
use std::collections::HashMap;



pub struct LineCodec;


impl LineCodec {
    // parse websocket proto


    fn parse_headers(msg: String) -> String {
        let mut head_str:String = String::from(""); 
        let mut headers: HashMap<String, String> = HashMap::new();
        if let Some(data) = msg.split("\r\n\r\n").next(){
        let mut it = data.split("\r\n");
        let mut key: String = String::from("");
        while let Some(kv) = it.next() {
            let mut it = kv.split(": ");
            if let Some(k) = it.next() {
                if let Some(v) = it.next() {
                    headers.insert(k.to_string(), v.to_string());
                    if (k == "Sec-WebSocket-Key") {
                        let mut m = sha1::Sha1::new();
                        let screct_key = v.to_owned() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
                        m.update(screct_key.as_bytes());
                        // let  = m.digest().to_string()
                        let key = base64::encode(&m.digest().to_string());
                    }

            }
        }}
let resp = format!("HTTP/1.1 101 Web Socket Protocol Handshake\r\n
Upgrade: webSocket\r\n
Connection: Upgrade\r\n
Sec-WebSocket-Accept: {}\r\n"
                             ,key           );

        let mut handshake = String::from(resp);
        // let head_str = format!(handshake,key);
        head_str = handshake;
        return head_str;
    }else{
            return head_str;
    }

}
}


impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
        
    }
}


impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        // buf.extend(Self.parse_headers(msg).as_bytes());
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
 
        Ok(())
    }
}

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = String;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

pub struct Echo;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}


pub fn line_run() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}