// extern crate serialize;
// use serialize::json;

// mod process_byte;

pub fn str_data(data:String) -> String{
    //{"length":5,"id":1,"msg_type":1,"msg":"abc"}
    // "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}";

    let end = data.replace("\"","\\\"");
    // println!("{}",end);
    return end;
}
