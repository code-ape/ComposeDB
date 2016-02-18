extern crate lmdb_rs as lmdb;

use std::path::Path;
use lmdb::{EnvBuilder, DbFlags};
use lmdb::core::{Environment, MdbResult};
use lmdb::traits::FromMdbValue;

pub type DbState<'a> = Option<DB<'a>>;

pub struct DB<'a> {
    path: &'a Path,
    env:  Environment
}


impl<'a> DB<'a> {
    pub fn new(p: &Path) -> DB {
        DB {
            path: p,
            env: EnvBuilder::new().open(p, 0o777).unwrap(),
        }
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
