
pub trait ToBytes {
    fn to_data<'a>(&'a self) -> Vec<u8>;
}

pub trait IntoBytes {
    fn into_data<'a>(self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_data(byte_vec: &Vec<u8>) -> Self;
}

pub trait TransformBytes {
    fn transform_data(byte_vec: Vec<u8>) -> Self;
}

impl ToBytes for String {
    fn to_data<'a>(&'a self) -> Vec<u8> {
        let t: &'a str = self;
        t.into_data()
    }
}

impl<'a> ToBytes for &'a str {
    fn to_data<'b>(&'b self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBytes for String {
    fn into_data<'a>(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> IntoBytes for &'a str {
    fn into_data<'b>(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}


#[test]
fn to_data_for_string() {
    let a = "Hello".to_string();
    let b = vec![72, 101, 108, 108, 111];
    let c = a.to_data();
    assert_eq!(b, c);
}

#[test]
fn into_data_for_string() {
    let a = "Hello".to_string();
    let b = vec![72, 101, 108, 108, 111];
    let c = a.into_data();
    assert_eq!(b, c);
}

#[test]
fn to_data_for_str() {
    let a : &str = "Hello";
    let b = vec![72, 101, 108, 108, 111];
    let c = a.to_data();
    assert_eq!(b, c);
}

#[test]
fn into_data_for_str() {
    let a : &str = "Hello";
    let b = vec![72, 101, 108, 108, 111];
    let c = a.into_data();
    assert_eq!(b, c);
}
