
use std::sync::Mutex;
use std::sync::mpsc::{channel, SyncSender, Sender, Receiver};

use std::io::Read;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

use core::query::{Query, SetQuery, GetQuery};
use server::api_structs;

pub fn ping(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "pong")))
}

pub fn get_value(req: &mut Request, in_ch_mut: &Mutex<SyncSender<Box<Query>>>)
                -> IronResult<Response> {
    debug!("Request for route '/json'");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let (q,rx) = GetQuery::new(payload); //as (T, Receiver<String>;

    {
        let in_ch = in_ch_mut.lock().unwrap();
        in_ch.send(Box::new(q));
    }

    let result : String = match rx.recv() {
        Ok(x) => x,
        _ => panic!("WTF CHANNEL")
    };
    let response = api_structs::GetResponse{ value: result };
    let response_string = json::encode(&response).unwrap();

    debug!("Responded with: {}", response_string);
    Ok(Response::with((status::Ok, response_string)))
}

pub fn set_value(req: &mut Request, in_ch_mut: &Mutex<SyncSender<Box<Query>>>) -> IronResult<Response> {
    debug!("Request for route '/json/set'");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let set_req : api_structs::SetRequest = json::decode(&payload).unwrap();

    let (tx, rx) : (Sender<String>, Receiver<String>)= channel();

    let q = SetQuery { key: set_req.key, value: "A VALUE".to_string(), chan: tx};//, chan: tx};
    {
        let in_ch = in_ch_mut.lock().unwrap();
        in_ch.send(Box::new(q));
    }

    let result : String = rx.recv().unwrap();

    let response = api_structs::SetResponse{ status: result };
    let response_string = json::encode(&response).unwrap();

    debug!("Resut for /set/json: {}", response_string);
    Ok(Response::with(status::Ok))
}
