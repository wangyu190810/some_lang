use std::io;
use std::str;
use bytes::{BytesMut, BufMut,IntoBuf,Buf};
use bytes::buf::FromBuf;
// use bytes::buf::FromBuf::from_buf;
use tokio_io::codec::{Encoder, Decoder};
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

use std::collections::HashMap;



pub struct LineCodec;
  

impl LineCodec {
    // parse websocket proto


    fn parse_headers(msg : String) -> String{
        
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut it = msg.split("\r\n");
        while let Some(kv) = it.next(){
            let mut it = kv.split(":");
            if let Some(k) = it.next(){
                 if let Some(v) = it.next() {
                    headers.insert(k.to_string(), v.to_string());
                }
            }
        }
        let mut handshake = String::from("\
HTTP/1.1 101 Web Socket Protocol Handshake\r\n\
Upgrade: webSocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept:{:}\r\n\
Sec-WebSocket-Origin: {:}\r\n\
Sec-WebSocket-Location: {:}\r\n\r\n\
");
        // let head_str = format!(handshake,headers.get("Origin").unwrap(), headers.get("token").unwrap(),headers.get("Location").unwrap());
        // return head_str;
        return handshake;
    }
}



impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut IntoBuf) -> io::Result<Option<String>> {
        if let Some(i) = Vec::from_buf(buf) {
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
         buf.extend(Self.parse_headers(msg).as_bytes());
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