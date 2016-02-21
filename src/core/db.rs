extern crate lmdb_rs as lmdb;

use std::str;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use rustc_serialize::json;
use lmdb::{EnvBuilder, DbFlags};
use lmdb::core::{Environment, MdbResult};
use lmdb::traits::FromMdbValue;

use core::action_log::{ActionLogFactory,ActionLogEntry};
use core::query::{run_query, new_getlastlog_query};

pub type DbState<'a> = Option<DB<'a>>;

pub struct DB<'a> {
    path: &'a Path,
    pub env:  Environment,
    pub action_log_factory: ActionLogFactory
}


impl<'a> DB<'a> {
    pub fn new(p: &Path) -> Arc<DB> {
        let env = EnvBuilder::new().open(p, 0o777).unwrap();
        let stale_readers = env.reader_check().unwrap();
        debug!("Removed {} stale readers", stale_readers);

        let mut db = DB {
            path: p,
            env: env,
            action_log_factory: ActionLogFactory::new(0)
        };

        let db_arc = Arc::new(db);

        let (query, chan) = new_getlastlog_query();
        debug!("A");
        let query_resp = run_query(Box::new(query), db_arc.clone());
        drop(db_arc);
        let last_action_number : u64 = match query_resp {
                Ok(_) => {
                    debug!("0");
                    let query_result = chan.recv().unwrap();
                    debug!("A");
                    let s = String::from_utf8(query_result).unwrap();
                    debug!("s = {}", s);
                    let last_action_log : ActionLogEntry = json::decode(&s).unwrap();
                    debug!("Last action log was: {}", last_action_log.number);
                    last_action_log.number
                    //10
                },
                Err(_) => {
                    debug!("No prior logs found, starting with 0");
                    0
                }
        };


        debug!("B");
        let env = EnvBuilder::new().open(p, 0o777).unwrap();
        let stale_readers = env.reader_check().unwrap();
        debug!("Removed {} stale readers", stale_readers);
        let mut db = DB {
            path: p,
            env: env,
            action_log_factory: ActionLogFactory::new(0)
        };

        let db_arc = Arc::new(db);
        debug!("C");
        db_arc.action_log_factory.number.store(last_action_number as usize, Ordering::Release);
        debug!("D");
        return db_arc;
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
