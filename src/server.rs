extern crate iron;
extern crate router;
extern crate rustc_serialize;

use std::collections::VecDeque;
use std::io::Read;
use std::sync::{Mutex, Arc, Condvar};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct GetRequest {
    key: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct GetResponse {
    value: u32
}

#[derive(RustcDecodable, RustcEncodable)]
struct SetRequest {
    key: String,
    value: u32
}

#[derive(RustcDecodable, RustcEncodable)]
struct SetResponse {
    status: u32
}

struct Job {
    number: u32,
    chan: Sender<u32>
}

fn get_value(_: &mut Request, queue_mut: &Mutex<VecDeque<Job>>, signal: &Condvar) -> IronResult<Response> {
    println!("Request for route '/json'");
    let (tx, rx) : (Sender<u32>, Receiver<u32>)= mpsc::channel();
    let j = Job { number: 0, chan: tx};
    {
        let mut queue = queue_mut.lock().unwrap();
        queue.push_back(j);
    }
    signal.notify_one();

    let result : u32 = rx.recv().unwrap();
    let response = GetResponse{ value: result };
    let response_string = json::encode(&response).unwrap();

    println!("Responded with: {}", response_string);
    Ok(Response::with((status::Ok, response_string)))
}

fn set_value(req: &mut Request, queue_mut: &Mutex<VecDeque<Job>>, signal: &Condvar) -> IronResult<Response> {
    println!("Request for route '/json/set'");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let set_req : SetRequest = json::decode(&payload).unwrap();

    let (tx, rx) : (Sender<u32>, Receiver<u32>)= mpsc::channel();

    let j = Job { number: set_req.value, chan: tx};
    {
        let mut queue = queue_mut.lock().unwrap();
        queue.push_back(j);
    }
    signal.notify_all();

    let result : u32 = rx.recv().unwrap();

    let response = SetResponse{ status: result };
    let response_string = json::encode(&response).unwrap();

    println!("Resut for /set/json: {}", response_string);
    Ok(Response::with(status::Ok))
}

fn main() {

    let queue : Arc<Mutex<VecDeque<Job>>> = Arc::new(Mutex::new(VecDeque::new()));
    let signal : Arc<Condvar> = Arc::new(Condvar::new());


    let num_workers = 3;

    for i in 0..num_workers {
        let queue_clone = queue.clone();
        let signal_clone = signal.clone();

        thread::spawn(move || {
            println!("Starting worker {}", i);
            let mut counter = 0;
            let mut j: Job;
            loop {
                loop {
                    let mut q = queue_clone.lock().unwrap();
                    q = signal_clone.wait(q).unwrap();
                    j = match q.pop_front() {
                        Some(x) => x,
                        None => continue
                    };
                    break;
                }
                println!("Worker {}: received job {}", i, counter);
                let new_value = j.number + 100;
                thread::sleep_ms(10000);
                j.chan.send(new_value).unwrap();
                counter += 1;
            }
            println!("Ending worker {}", i);
        });
    }

    let mut router = Router::new();

    let queue_2 = queue.clone();
    let queue_3 = queue.clone();
    let signal_2 = signal.clone();
    let signal_3 = signal.clone();

    router.get("/json", move |r: &mut Request| get_value(r, &queue_2, &signal_2));
    router.post("/json/set", move |r: &mut Request| set_value(r, &queue_3, &signal_3));

    println!("Starting ComposeDB.");
    Iron::new(router).http("localhost:3000").unwrap();

}
