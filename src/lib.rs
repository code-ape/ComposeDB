extern crate lmdb_rs as lmdb;

mod db;
mod state;
mod people;
mod data_interface;

use state::State;
use db::DB;

fn try_me() {
    println!("Starting ComposeDB");

    // let settings = state::Settings::new();
    // let s = State::new(&settings);
    // let db_obj = DB::new(s.db_path);
    // db_obj.set(&"key1", &"val1");
    //
    // let val = db_obj.get(&"key1");
    // println!("key1 => {}", val);

    println!("Exiting ComposeDB");
}

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
