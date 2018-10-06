#![allow(unused)]
#![feature(fnbox)]

pub mod query;
pub mod utils;
pub mod Pool;
// pub mod handler;
pub mod func_test;

#[cfg(test)]


mod tests {
    #[test]
    fn it_works() {
    }

    #[test]
    fn func_test_case(){
        func_test::test_make_map()
    }
}


