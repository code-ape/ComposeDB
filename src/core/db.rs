extern crate lmdb_rs as lmdb;

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::collections::BTreeMap;

use rustc_serialize::json;
use lmdb::{EnvBuilder, DbFlags};
use lmdb::core::{Environment, DbHandle, MdbResult};
use lmdb::traits::FromMdbValue;

use core::action_log::{ActionLogFactory,ActionLogEntry};
use core::query::{run_query, new_getlastlog_query};
use core::data_interface::Serializable;

pub type DbState<'a> = Option<DB<'a>>;

pub struct DB<'a> {
    path: &'a Path,
    pub env:  Environment,
    pub handle: DbHandle,
    pub action_log_factory: ActionLogFactory,
    pub name_to_id_map: BTreeMap<String,u16>,
    pub id_to_trait_map: BTreeMap<u16,Box<Serializable>>
    //pub id_to_trait_map: BTreeMap<u16,Box<Fn<T>(&str)->T>>
}


impl<'a> DB<'a> {
    pub fn new(p: &Path) -> Arc<DB> {
        {
            let env = EnvBuilder::new().open(p, 0o777).unwrap();
            let stale_readers = env.reader_check().unwrap();
            debug!("Removed {} stale readers", stale_readers);
        }

        let env = EnvBuilder::new().open(p, 0o777).unwrap();
        let db_handle  = env.get_default_db(DbFlags::empty()).unwrap();
        let db = Arc::new(DB {
            path: p,
            env: env,
            handle: db_handle,
            action_log_factory: ActionLogFactory::new(0),
            name_to_id_map: BTreeMap::new(),
            id_to_trait_map: BTreeMap::new()
        });


        let (query, chan) = new_getlastlog_query();
        let query_resp = run_query(Box::new(query), db.clone());
        let last_action_number : u64 = match query_resp {
                Ok(_) => {
                    let query_result = chan.recv().unwrap();
                    let s = String::from_utf8(query_result).unwrap();
                    let last_action_log : ActionLogEntry = json::decode(&s).unwrap();
                    debug!("Last action log was: {}", last_action_log.number);
                    last_action_log.number
                },
                Err(_) => {
                    debug!("No prior logs found, starting with 0");
                    0
                }
        };

        db.action_log_factory.number.store(last_action_number as usize, Ordering::Release);
        return db;
    }

    pub fn set(&self, key: &str, val: &str) -> MdbResult<()> {
        let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
        let txn = self.env.new_transaction().unwrap();
        {
            let db = txn.bind(&db_handle); // get a database bound to this transaction
            db.set(&key, &val).unwrap();
        }

        // Note: `commit` is choosen to be explicit as
        // in case of failure it is responsibility of
        // the client to handle the error
        return txn.commit();
    }

    pub fn get<V: FromMdbValue>(&self, key: &str) -> MdbResult<V> {
        let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
        let reader = self.env.get_reader().unwrap();
        let db = reader.bind(&db_handle);
        return db.get(&key);
    }
}
