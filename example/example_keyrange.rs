extern crate lmdb_rs as lmdb;
extern crate rustc_serialize;

use std::path::Path;
use std::str;
use std::mem;
use std::f64::consts::{E,PI};
use rustc_serialize::json;
use lmdb::{EnvBuilder, DbFlags};

macro_rules! path {
    ( $( $x:expr ),* ) => {
        {
            [$($x,)*].join("/")
        }
    };
}

trait DbWorthy {
    fn get_id(&self) -> String;
}

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    first_name: String,
    last_name:  String,
    age:        usize
}

impl Person {
    fn new(first_name: &str, last_name: &str, age: usize) -> Person {
        Person {first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age: age}
    },
    fn get()
}

impl DbWorthy for Person {
    fn get_id(&self) -> String {
        path!(&*self.first_name, &*self.last_name)
    }
}

const PERSON_KEY: &'static str =   "PERSON";
const PERSON_BEGIN: &'static str = "PERSON/";
const PERSON_END: &'static str =   "PERSON0";


fn print_key_value(key: &str, value: &[u8]) {
    println!("key: {}\nraw value: {:?}", key, value);
    let split_key : Vec<&str> = key.split("/").collect();
    match *split_key.get(0).unwrap() {
        "NAME" => println!("value: {}", str::from_utf8(value).unwrap()),
        _ => println!("Not sure how to decode, attempting string.\nvalue: {}",
                        str::from_utf8(value).unwrap())
    }
    println!("");
}

fn main() {
    //write_to_db();
    test_cursor();
    //test_keyrange_from_name();
    //test_keyrange_from_number();
    //test_keyrange_from_to_name();
}

#[allow(dead_code)]
fn write_to_db() {
    let path = Path::new("test-lmdb");
    let env = EnvBuilder::new().open(&path, 0o777).unwrap();

    let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
    let txn = env.new_transaction().unwrap();
    {
        let db = txn.bind(&db_handle); // get a database bound to this transaction

        let people = vec![Person::new("Albert","Einstein", 137),
                          Person::new("Jack", "Daniel", 167)];

        for person in people.iter() {
            let full_key = path!(PERSON_KEY, &*person.get_id());
            db.set(&full_key, &&*json::encode(&person).unwrap()).unwrap();
        }
    }
    match txn.commit() {
        Err(_) => panic!("failed to commit!"),
        Ok(_) => ()
    }
}

#[allow(dead_code)]
fn test_cursor() {
    let path = Path::new("test-lmdb");
    let env = EnvBuilder::new().open(&path, 0o777).unwrap();

    let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
    let reader = env.get_reader().unwrap();
    let db = reader.bind(&db_handle);
    let cursor = db.iter().unwrap();

    for item in cursor {
        let (key,value) = item.get::<&str,&[u8]>();
        print_key_value(key, value);
    }
}


// #[allow(dead_code)]
// fn test_keyrange_from_to_name() {
//     let path = Path::new("test-lmdb");
//     let env = EnvBuilder::new().open(&path, 0o777).unwrap();
//
//     let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
//     let reader = env.get_reader().unwrap();
//     let db = reader.bind(&db_handle);
//     let successor = string_successor(NAME_KEY_TYPE);
//     let str_successor = &*successor;
//     println!("Successor to {} is {}", NAME_KEY_TYPE, successor);
//     let cursor = db.keyrange_from_to(&NAME_KEY_TYPE, &str_successor).unwrap();
//
//     for item in cursor {
//         let (key,value) = item.get::<&str,&[u8]>();
//         print_key_value(key, value);
//     }
// }
