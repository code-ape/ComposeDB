use core::data_interface::ToBytes;
use rustc_serialize::{json,Encodable};

#[derive(RustcDecodable, RustcEncodable)]
pub struct DataBlob {
    pub int_type: u32,
    pub version: u64,
    pub data: Vec<u8>
}

impl DataBlob {
    pub fn new_from_struct<T: Encodable>(int_type: u32, version: u64, data: T) -> DataBlob {
        DataBlob {
            int_type: int_type,
            version: version,
            data: json::encode(&data).unwrap().into_bytes()
        }
    }
    pub fn new_from_vec(int_type: u32, version: u64, data: Vec<u8>) -> DataBlob {
        DataBlob {
            int_type: int_type,
            version: version,
            data: data
        }
    }
}

fn to_blob<T: ToBytes>(int_type: u32, version: u64, thing: T) -> DataBlob {
    DataBlob {
        int_type: int_type,
        version: version,
        data: thing.to_data()
    }
}

#[test]
fn to_blob_for_string() {
    let int_type = 42;
    let a = "Hello".to_string();
    let b = vec![72, 101, 108, 108, 111];
    let c = to_blob(int_type, a);
    assert_eq!(c.int_type, int_type);
    assert_eq!(c.data, b);
}
