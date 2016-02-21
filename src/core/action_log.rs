use std::sync::atomic::{AtomicUsize,Ordering};
use time;
use rustc_serialize::json;

pub struct ActionLogFactory {
    pub number: AtomicUsize
}

impl ActionLogFactory {
    pub fn new(start_number: u64) -> ActionLogFactory {
        ActionLogFactory{number: AtomicUsize::new(start_number as usize)}
    }
    pub fn new_entry(&self, key: String, version: u64) -> ActionLogEntry {
        ActionLogEntry::new(self.number.fetch_add(1, Ordering::Acquire) as u64, key, version)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ActionLogEntry {
    pub number: u64,
    pub time: i64,
    pub key: String,
    pub version: u64
}

impl ActionLogEntry {
    fn new(number: u64, key: String, version: u64) -> ActionLogEntry {
        ActionLogEntry{ number: number, time: time::get_time().sec, key: key, version: version }
    }
    pub fn gen_key(&self) -> String {
        format!("log/{}", self.number)
    }
    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}
