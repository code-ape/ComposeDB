#[macro_use] extern crate log;
extern crate env_logger;
extern crate lmdb_rs as lmdb;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

extern crate time;


mod db;
mod core;
pub mod server;

//use db::state::State;
use core::db::DB;


#[test]
fn it_works() {
    let settings = state::Settings::new();
    let s = State::new(&settings);
    let db_obj = DB::new(s.db_path);

    let set_res = db_obj.set(&"key1", &"val1");
    assert!(set_res.is_ok());
    let val_res = db_obj.get::<&str>(&"key1");
    assert!(val_res.is_ok());

    assert_eq!(val_res.unwrap(), "val1");
}
