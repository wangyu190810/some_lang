extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate file_copy;

use std::fs::File;
use bincode::{serialize, deserialize, SizeLimit};
use file_copy::sever::httpparse::run;
use file_copy::model::fileupload;

#[derive(Serialize, Deserialize)]
struct A {
    id: i8,
    key: i16,
    name: String,
    values: Vec<String>,
}

#[cfg(windows)]
fn fuck_new_line() -> &'static str {
  "windows\r\n"
}

#[cfg(not(windows))]
fn fuck_new_line() -> &'static str {
  "not windows\n"
}


fn main() {
    let a = A {
        id: 42,
        key: 1337,
        name: "Hello world".to_string(),
        values: vec!["alpha".to_string(), "beta".to_string()],
    };
    println!("{}",fuck_new_line());
    // Encode to something implementing Write
    // let mut f = File::create("/tmp/output.bin").unwrap();
    // bincode::serde::serialize_into(&mut f, &a, bincode::SizeLimit::Infinite).unwrap();

    // // Or just to a buffer
    // let bytes = bincode::serde::serialize(&a, bincode::SizeLimit::Infinite).unwrap();
    // println!("{:?}", bytes);
    //run();
    fileupload::run();
}


// #[macro_use]
// extern crate serde_derive;

// extern crate serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     let serialized = serde_json::to_string(&point).unwrap();
//     println!("serialized = {}", serialized);

//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//     println!("deserialized = {:?}", deserialized);
// }

// #[derive(Serialize, Deserialize, PartialEq)]
// struct Entity {
//     x: f32,
//     y: f32,
// }

// #[derive(Serialize, Deserialize, PartialEq)]
// struct World(Vec<Entity>);

// fn main() {
//     let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

//     let encoded: Vec<u8> = serialize(&world, SizeLimit::Infinite).unwrap();

//     // 8 bytes for the length of the vector, 4 bytes per float.
//     assert_eq!(encoded.len(), 8 + 4 * 4);

//     let decoded: World = deserialize(&encoded[..]).unwrap();

//     assert!(world == decoded);
// }