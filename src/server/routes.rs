
use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

use server::jobs::Job;
use server::api_structs;

pub fn get_value(_: &mut Request, queue_mut: &Mutex<VecDeque<Job>>, signal: &Condvar) -> IronResult<Response> {
    println!("Request for route '/json'");
    let (tx, rx) : (Sender<u32>, Receiver<u32>)= mpsc::channel();
    let j = Job { number: 0, chan: tx};
    {
        let mut queue = queue_mut.lock().unwrap();
        queue.push_back(j);
    }
    signal.notify_one();

    let result : u32 = rx.recv().unwrap();
    let response = api_structs::GetResponse{ value: result };
    let response_string = json::encode(&response).unwrap();

    println!("Responded with: {}", response_string);
    Ok(Response::with((status::Ok, response_string)))
}

pub fn set_value(req: &mut Request, queue_mut: &Mutex<VecDeque<Job>>, signal: &Condvar) -> IronResult<Response> {
    println!("Request for route '/json/set'");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let set_req : api_structs::SetRequest = json::decode(&payload).unwrap();

    let (tx, rx) : (Sender<u32>, Receiver<u32>)= mpsc::channel();

    let j = Job { number: set_req.value, chan: tx};
    {
        let mut queue = queue_mut.lock().unwrap();
        queue.push_back(j);
    }
    signal.notify_all();

    let result : u32 = rx.recv().unwrap();

    let response = api_structs::SetResponse{ status: result };
    let response_string = json::encode(&response).unwrap();

    println!("Resut for /set/json: {}", response_string);
    Ok(Response::with(status::Ok))
}
