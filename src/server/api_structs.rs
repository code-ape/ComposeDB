extern crate rustc_serialize;


#[derive(RustcDecodable, RustcEncodable)]
pub struct GetRequest {
    pub key: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GetResponse {
    pub value: u32
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SetRequest {
    pub key: String,
    pub value: u32
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SetResponse {
    pub status: u32
}
