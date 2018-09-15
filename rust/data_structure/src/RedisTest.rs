// extern crate redis_client;
use redis_client;
use redis_client::commands::CommandSender;
use redis_client::errors::RedisError;
use redis;

pub fn set_and_get() -> Result<String, RedisError> {
    let mut client = try!(redis_client::RedisClient::new("192.168.1.11", "6379"));
    // let set_result: String = try!(client.set("rs_test_1", "{\"length\":5,\"id\":1,\\\"msg_type\\\":1,\\\"msg\\\":\\\"abc\\\"}"));
    
    // let set_result: String = try!(client.set("rs_test_1", "13212"));
    // client.set("rs_test_1", "{\"length\":5,\"id\":1,\"msg_type\":1,\"msg\":\"abc\"}");
    let get_result: String = try!(client.get("rs_test_1"));
    println!("{:?}",get_result.to_string());
    Ok("asdf".to_string())
}


pub fn do_something() -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://192.168.1.11"));
    let con = try!(client.get_connection());
    let _ : () = try!(redis::cmd("SET").arg("my_key").arg("{\"length\":5,\"id\":1,\"msg_type\":1,\"msg\":\"abc\"}").query(&con));
    let _ : () = try!(redis::cmd("HSET").arg("re_test_json").arg(1).arg("{\"length\":5,\"id\":1,\"msg_type\":1,\"msg\":\"abc\"}").query(&con));

    /* do something here */

    Ok(())
}
