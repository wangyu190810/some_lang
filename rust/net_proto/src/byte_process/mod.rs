use bytes::Bytes;

pub fn start_byte() {

    let mut mem = Bytes::from(&b"Hello world"[..]);
    let a = mem.slice(0, 5);

    assert_eq!(&a[..], b"Hello");

    let b = mem.split_to(6);

    assert_eq!(&mem[..], b"world");
    assert_eq!(&b[..], b"Hello ");

    assert_eq!(&mem[..], b"world");
}
pub fn parse_byte(){

}


#[test]
fn test_case(){
    start_byte();
}

