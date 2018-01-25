// extern crate redis_client;
use redis_client;
use redis_client::commands::CommandSender;
use redis_client::errors::RedisError;

pub fn set_and_get() -> Result<String, RedisError> {
    let mut client = try!(redis_client::RedisClient::new("192.168.1.11", "6379"));
    // let set_result: String = try!(client.set("rs_test_1", "{\"length\":5,\"id\":1,\\\"msg_type\\\":1,\\\"msg\\\":\\\"abc\\\"}"));
    
    // let set_result: String = try!(client.set("rs_test_1", "13212"));
    // client.set("rs_test_1", "{\"length\":5,\"id\":1,\"msg_type\":1,\"msg\":\"abc\"}");
    let get_result: String = try!(client.get("rs_test_1"));
    println!("{:?}",get_result.to_string());
    Ok("asdf".to_string())
}
