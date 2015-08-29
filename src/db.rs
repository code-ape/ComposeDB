extern crate lmdb_rs as lmdb;


use std::path::Path;
use lmdb::{EnvBuilder, DbFlags};
use lmdb::core::Environment;

pub type DbState = Option<DB>;

pub struct DB {
    env: Environment
}

impl DB {
    pub fn new(p: &Path) -> DB {
        DB {
            env: EnvBuilder::new().open(p, 0o777).unwrap(),
        }
    }

    pub fn set(&self, key: &str, val: &str) {
        let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
        let txn = self.env.new_transaction().unwrap();
        {
            let db = txn.bind(&db_handle); // get a database bound to this transaction
            db.set(&key, &val).unwrap();
        }

        // Note: `commit` is choosen to be explicit as
        // in case of failure it is responsibility of
        // the client to handle the error
        match txn.commit() {
            Err(_) => panic!("Failed to commit!"),
            Ok(_) => ()
        }
    }

    pub fn get(&self, key: &str) -> &str {
        let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
        let reader = self.env.get_reader().unwrap();
        let db = reader.bind(&db_handle);
        return db.get::<&str>(&key).unwrap();
    }
}
