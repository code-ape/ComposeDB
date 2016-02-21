use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver, SendError};
use lmdb::core::{Environment, MdbResult};
use lmdb::DbFlags;
use rustc_serialize::json;

use core::db::DB;
use core::blob::DataBlob;

// pub fn gen_run_query(env: Arc<Environment>) -> Box<Fn(Box<Query>) -> Result<(),()>> {
//     Box::new(move |q: Box<Query>| {
//         run_query(q, env)
//     })
// }

pub fn run_query(q: Box<Query>, db: Arc<DB>) -> Result<(),()> {
    match *q {
        //TODO: make this use blobs
        Query::Get{ref key, ref chan} => {
            debug!("Received GetQuery");
            let db_handle = db.env.get_default_db(DbFlags::empty()).unwrap();
            let reader = db.env.get_reader().unwrap();
            let db_ref = reader.bind(&db_handle);
            match db_ref.get::<&[u8]>(&*key) {
                Ok(val) => {
                    let b : DataBlob = json::decode(str::from_utf8(val).unwrap()).unwrap();
                    chan.send(b.data).unwrap();
                    debug!("Finished processing GetQuery");
                    Ok(())
                },
                Err(e) => {
                    error!("Error retrieving key '{}': {}", key, e);
                    chan.send("ERROR".to_string().into_bytes()).unwrap();
                    Err(())
                }
            }

        },
        //TODO: make this use blobs
        Query::Set{ref key, ref value, ref chan} => {
            debug!("Received SetQuery");
            let db_handle = db.env.get_default_db(DbFlags::empty()).unwrap();
            let txn = db.env.new_transaction().unwrap();
            let action_log = db.action_log_factory.new_entry(key.clone(), 0);
            let b : DataBlob = DataBlob::new_from_vec(0,0,value.clone().into_bytes());
            {
                let db_ref = txn.bind(&db_handle);
                db_ref.set(&*key, &json::encode(&b).unwrap()).unwrap();
                db_ref.set(&action_log.gen_key(), &action_log.to_json()).unwrap();
            }

            match txn.commit() {
                Ok(_) => {
                    chan.send("OK".to_string().into_bytes()).unwrap();
                    debug!("Finished processing SetQuery");
                    Ok(())
                }
                Err(_) => Err(())
            }
        },
        Query::GetLast{ref key, ref chan} => {
            debug!("Received GetLastQuery");
            let db_handle = db.env.get_default_db(DbFlags::empty()).unwrap();
            let reader = db.env.get_reader().unwrap();
            let db_ref = reader.bind(&db_handle);
            let (range_begin, range_end) = (format!("{}/", key), format!("{}0", key));
            let cursor = db_ref.keyrange_from_to(&range_begin, &range_end).unwrap();
            match cursor.last() {
                Some(cursor_val) => {
                    let val = cursor_val.get_value::<&[u8]>();
                    let b : DataBlob = json::decode(str::from_utf8(val).unwrap()).unwrap();
                    chan.send(b.data).unwrap();
                    debug!("Finished processing GetLastQuery");
                    Ok(())
                },
                None  => {
                    error!("No values for key's under '{}'", key);
                    chan.send("ERROR".to_string().into_bytes()).unwrap();
                    Err(())
                }
            }
        },
        Query::GetLastLog{ref chan} => {
            debug!("Received GetLastLog Query");
            let key = "log";
            let db_handle = db.env.get_default_db(DbFlags::empty()).unwrap();
            let reader = db.env.get_reader().unwrap();
            let db_ref = reader.bind(&db_handle);
            // let (range_begin, range_end) = (format!("{}/", key), format!("{}0", key));
            // let cursor = db_ref.keyrange_from_to(&range_begin, &range_end).unwrap();
            // match cursor.last() {
            match db_ref.get::<Vec<u8>>(&"log/0") {
                //    Some(cursor_val) => {
                Ok(val) => {
                    //TODO: figure out why this is segfaulting
                    //let val = cursor_val.get_value::<Vec<u8>>();
                    println!("YAY");
                    chan.send(val).unwrap();
                    debug!("Finished processing GetLastLog Query");
                    Ok(())
                },
                // None  => {
                Err(_)  => {
                    error!("No values for key's under '{}'", key);
                    chan.send("ERROR".to_string().into_bytes()).unwrap();
                    Err(())
                }
            }
        }
    }
}



pub enum Query {
    Set{key: String, value: String, chan: Sender<Vec<u8>>},
    //Update{key: String, value: String, chan: Sender<String>},
    Get{key: String, chan: Sender<Vec<u8>>},
    GetLast{key: String, chan: Sender<Vec<u8>>},
    GetLastLog{chan: Sender<Vec<u8>>}
}

pub fn new_set_query(key: String, val: String) -> (Query, Receiver<Vec<u8>>) {
    let (tx, rx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    (Query::Set{ key: key, value: val, chan: tx }, rx)
}

pub fn new_get_query(key: String) -> (Query, Receiver<Vec<u8>>) {
    let (tx, rx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    (Query::Get{ key: key, chan: tx }, rx)
}

pub fn new_getlast_query(key: String) -> (Query, Receiver<Vec<u8>>) {
    let (tx, rx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    (Query::GetLast{ key: key, chan: tx }, rx)
}

pub fn new_getlastlog_query() -> (Query, Receiver<Vec<u8>>) {
    let (tx, rx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    (Query::GetLastLog{ chan: tx }, rx)
}
