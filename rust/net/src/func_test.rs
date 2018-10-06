#![allow(unused)]
#![feature(fnbox)]

use std::boxed::FnBox;
use std::collections::HashMap;

pub trait FnBox<A> {
    type Output;

    fn call_box(self: Box<Self>, args: A) -> Self::Output;
}

pub struct FnBoxData{
    list:Vec<i8>
}



impl<i8, FnBoxData> FnBox<i8> for FnBoxData
    where F: FnOnce<i8>
{
    type Output = FnBoxData::Output;

    fn call_box(self: Box<FnBoxData>, args: i8) -> FnBoxData::Output {
        self.call_once(args)
    }
}








fn make_map() -> HashMap<i32, Box<FnBox() -> i32>> {
    let mut map: HashMap<i32, Box<FnBox() -> i32>> = HashMap::new();
    map.insert(1, Box::new(|| 22));
    map.insert(2, Box::new(|| 44));
    map
}

pub fn test_make_map() {
    let mut map = make_map();
    for i in &[1, 2] {
        let f = map.remove(&i).unwrap();
        assert_eq!(f(), i * 22);
    }
}