#[macro_use]
extern crate json;
extern crate redis;
extern crate serde;
extern crate serde_json;

use serde_json::Map;


   
use redis::{Client, Commands, Connection, RedisResult};


fn json_str() -> String{
    let mut map = Map::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);
    serde_json::to_string(&map).unwrap()
}

fn conn(addr: &str) -> Connection{

    let client = Client::open(addr).unwrap();

    let conn = client.get_connection().unwrap();
    return conn
    }

fn hash_set(conn: &Connection, key: &str, filter :&str, values :&str) -> RedisResult<()>{
    try!(conn.hset(key, filter, values));
    //try!(conn.set(key, values));
    Ok(())
}


fn data () -> json::JsonValue{
    let _data = object!{
         "foo" => false,
         "bar" => json::Null,
         "answer" => 42
    };
    return _data
   
   }


fn main(){
    
    let parsed = json::parse(r#"
    {
        "code":200,
        "sucess":true
        
    }
    "#).unwrap();    
    let _data = data();
    println!("{:?}",&_data);
    println!("{:?}",&_data["fo"]);
    let conn_ = conn("redis://127.0.0.1/");
    let value = json_str();    
    hash_set(&conn_,"test_rs_1","123",&value );
    let falg = hash_set(&conn_,"test_rs","123","fasdf" );
    
}
