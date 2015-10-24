extern crate lmdb_rs as lmdb;
use lmdb::core::MdbValue;

use std::string::FromUtf8Error;


pub trait ToBytes {
    fn to_data<'a>(&'a self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_data(byte_vec: &Vec<u8>) -> Self;
}

impl ToBytes for String {
    fn to_data<'a>(&'a self) -> Vec<u8> {
        let t: &'a str = self;
        t.as_bytes().to_vec()
    }
}

// impl FromBytes for String {
//     fn from_data(byte_vec: &Vec<u8>) -> Self {
//         let a = *byte_vec;
//         String::from_utf8(a).unwrap()
//     }
// }

#[test]
fn to_data_for_string() {
    let a = "Hello".to_string();
    let b = a.to_data();
    let c = a.into_bytes();
    assert_eq!(b, c);
}
