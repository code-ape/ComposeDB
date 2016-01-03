use std::sync::mpsc::{channel, Sender, Receiver, SendError};

pub trait Query: Send {
    fn get_type(&self) -> QueryType;
    fn send_result(&self) -> Result<(), SendError<u32>>;
}

// impl<F: ?Sized> Query for Box<F> where F: Query {
//     fn get_type(&self) -> QueryType {
//         (**self).get_type()
//     }
//
//     fn send_result(&self) -> Result<(), SendError<u32>> {
//         (**self).send_result()
//     }
// }

pub enum QueryType {
    Set,
    Get
}

pub struct GetQuery {
    pub key: String,
    pub chan: Sender<String>
}

pub struct SetQuery {
    pub key: String,
    pub value: String,
    pub chan: Sender<String>
}

impl GetQuery {
    pub fn new(key: String) -> (GetQuery, Receiver<String>) {
        let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
        (GetQuery{ key: key, chan: tx }, rx)
    }
}

impl SetQuery {
    pub fn new(key: String, val: String) -> (SetQuery, Receiver<String>) {
        let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
        (SetQuery{ key: key, value: val, chan: tx }, rx)
    }
}


impl Query for GetQuery {

    fn get_type(&self) -> QueryType { QueryType::Get }

    fn send_result(&self) -> Result<(), SendError<u32>> {
        Ok(())
    }
}

impl Query for SetQuery {

    fn get_type(&self) -> QueryType { QueryType::Set }

    fn send_result(&self) -> Result<(), SendError<u32>> {
        Ok(())
    }
}
