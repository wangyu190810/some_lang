use std::io;
use std::str;
use base64;
use sha_1;
use sha1;
use std::collections::HashMap;



pub struct WebSocket;


impl WebSocket {
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
    fn recv(recv_data: str){
        let mut data = recv_data.as_bytes();
        
    }   
    
}

