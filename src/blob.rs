use data_interface::ToBytes;

pub struct DataBlob {
    int_type: u32,
    data: Vec<u8>
}

fn to_blob<T: ToBytes>(int_type: u32, thing: T) -> DataBlob {
    DataBlob {
        int_type: int_type,
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
