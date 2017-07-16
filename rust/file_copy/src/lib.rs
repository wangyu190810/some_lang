extern crate hyper;
extern crate futures;
extern crate time;

#[macro_use]
extern crate mysql;

pub mod sever;
pub mod model;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
