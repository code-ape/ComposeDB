extern crate lmdb_rs as lmdb;

mod db;
mod state;

use state::State;
use db::DB;

fn main() {
    println!("Starting ComposeDB");

    let settings = state::Settings::new();

    let s = State::new(&settings);

    let db_obj = DB::new(s.db_path);


    db_obj.set(&"key1", &"val1");

    let val = db_obj.get(&"key1");
    println!("key1 = {}", val);

    println!("Exiting ComposeDB");
}
