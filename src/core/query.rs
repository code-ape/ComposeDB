use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver, SendError};
use lmdb::core::{Environment, MdbResult};
use lmdb::DbFlags;

// pub fn gen_run_query(env: Arc<Environment>) -> Box<Fn(Box<Query>) -> Result<(),()>> {
//     Box::new(move |q: Box<Query>| {
//         run_query(q, env)
//     })
// }

pub fn run_query(q: Box<Query>, env: Arc<Environment>) -> Result<(),()> {
    match *q {
        Query::Get{key: ref key, chan: ref chan} => {
            debug!("Received GetQuery");
            let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
            debug!("A");
            let reader = env.get_reader().unwrap();
            debug!("B");
            let db = reader.bind(&db_handle);
            debug!("C");
            match db.get::<&str>(&*key) {
                Ok(val) => {
                    debug!("D");
                    chan.send(val.to_string()).unwrap();
                    debug!("Finished processing GetQuery");
                    Ok(())
                },
                Err(e) => {
                    error!("Error retrieving key '{}': {}", key, e);
                    chan.send("ERROR".to_string()).unwrap();
                    Err(())
                }
            }

        },
        Query::Set{key: ref key, value: ref value, chan: ref chan} => {
            debug!("Received SetQuery");
            let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
            debug!("A");
            let txn = env.new_transaction().unwrap();
            debug!("B");
            {
                let db = txn.bind(&db_handle);
                debug!("C");
                db.set(&*key, &*value).unwrap();
            }

            debug!("D");
            match txn.commit() {
                Ok(_) => {
                    chan.send("OK".to_string()).unwrap();
                    debug!("Finished processing SetQuery");
                    Ok(())
                }
                Err(_) => Err(())
            }
        }
    }
}

// pub trait Query: Send {
//     fn get_type(&self) -> QueryType;
//     fn send_result(&self) -> Result<(), SendError<u32>>;
// }


pub enum Query {
    Set{key: String, value: String, chan: Sender<String>},
    Get{key: String, chan: Sender<String>}
}

pub fn new_set_query(key: String, val: String) -> (Query, Receiver<String>) {
    let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
    (Query::Set{ key: key, value: val, chan: tx }, rx)
}

pub fn new_get_query(key: String) -> (Query, Receiver<String>) {
    let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
    (Query::Get{ key: key, chan: tx }, rx)
}

// pub struct GetQuery {
//     pub key: String,
//     pub chan: Sender<String>
// }
//
// pub struct SetQuery {
//     pub key: String,
//     pub value: String,
//     pub chan: Sender<String>
// }


// impl GetQuery {
//     pub fn new(key: String) -> (GetQuery, Receiver<String>) {
//         let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
//         (GetQuery{ key: key, chan: tx }, rx)
//     }
// }
//
// impl SetQuery {
//     pub fn new(key: String, val: String) -> (SetQuery, Receiver<String>) {
//         let (tx, rx) : (Sender<String>, Receiver<String>) = channel();
//         (SetQuery{ key: key, value: val, chan: tx }, rx)
//     }
// }
//
//
// impl Query for GetQuery {
//
//     fn get_type(&self) -> QueryType { QueryType::Get }
//
//     fn send_result(&self) -> Result<(), SendError<u32>> {
//         Ok(())
//     }
// }
//
// impl Query for SetQuery {
//
//     fn get_type(&self) -> QueryType { QueryType::Set }
//
//     fn send_result(&self) -> Result<(), SendError<u32>> {
//         Ok(())
//     }
// }
