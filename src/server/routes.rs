
use std::sync::Mutex;
use std::sync::mpsc::{channel, SyncSender, Sender, Receiver};

use std::io::Read;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use rustc_serialize::json::Json;

use core::query::{Query, new_set_query, new_get_query};
use server::api_structs;

pub fn ping(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "pong")))
}

macro_rules! try_json_conv {
    ( $j:ident, $s:path ) => ({
        match $s(&$j) {
            Some(x) => x,
            None => {
                error!("Failed casting json to map.");
                return Ok(Response::with((status::BadRequest, "Incorrectly formatted JSON")));
            }
        }
    })
}

macro_rules! try_json_from_req {
    ( $req:ident ) => ({
        let mut payload = String::new();
        $req.body.read_to_string(&mut payload).unwrap();

        match Json::from_str(&*payload) {
            Ok(x) => {
                debug!("received:\n{}", x.pretty());
                x
            },
            Err(e) => {
                error!("Error decoding json: {}", e);
                return Ok(Response::with((status::BadRequest, "Invalid JSON")));
            }
        }
    })
}

macro_rules! try_json_get {
    ( $j:ident, $s:expr ) => ({
        match $j.get($s) {
            Some(x) => x,
            None => {
                error!("Failed, no key '{}' for map.", $s);
                return Ok(Response::with((status::BadRequest, "Incorrectly formatted JSON")));
            }
        }
    });
    ( $j:ident, $s:expr, $w:path ) => ({
        match $j.get($s) {
            Some(x) => try_json_conv!(x, $w),
            None => {
                error!("Failed, no key '{}' for map.", $s);
                return Ok(Response::with((status::BadRequest, "Incorrectly formatted JSON")));
            }
        }
    });
}

pub fn get_handle(req: &mut Request, in_ch_mut: &Mutex<SyncSender<Box<Query>>>)
                -> IronResult<Response> {
    debug!("Get request for route '/json'");

    let json_req = try_json_from_req!(req);
    let j = try_json_conv!(json_req, Json::as_object);
    let command = try_json_get!(j,"command", Json::as_string);
    let key = try_json_get!(j,"key", Json::as_string);

    match command {
        "get" => {
            let (q,rx) = new_get_query(key.to_string());
            {
                let in_ch = in_ch_mut.lock().unwrap();
                in_ch.send(Box::new(q)).unwrap();
            }
            let result : Vec<u8> = match rx.recv() {
                Ok(x) => x,
                _ => {
                    error!("Failed to retrieve value over channel.");
                    return Ok(Response::with(status::InternalServerError));
                }
            };
            let text = String::from_utf8(result).unwrap();
            debug!("Responded with: {}", text);
            return Ok(Response::with((status::Ok, text)));
        },
        _ => {
            return Ok(Response::with((status::BadRequest,"Invalid command")));
        }
    }
}


pub fn post_handle(req: &mut Request, in_ch_mut: &Mutex<SyncSender<Box<Query>>>)
                -> IronResult<Response> {
    debug!("Post request for route '/json'");

    let json_req = try_json_from_req!(req);
    let j = try_json_conv!(json_req, Json::as_object);
    let command = try_json_get!(j,"command", Json::as_string);
    let key = try_json_get!(j,"key", Json::as_string);

    match command {
        "set" => {
            let data = try_json_get!(j, "data");
            let string_data = json::encode(&data).unwrap();
            let (q,rx) = new_set_query(key.to_string(), string_data);

            {
                let in_ch = in_ch_mut.lock().unwrap();
                in_ch.send(Box::new(q));
            }


            let result : Vec<u8> = match rx.recv() {
                Ok(x) => x,
                _ => {
                    error!("Failed to retrieve value over channel.");
                    return Ok(Response::with(status::InternalServerError));
                }
            };

            let text = String::from_utf8(result).unwrap();
            debug!("Result for post to /json: {}", text);
            return Ok(Response::with((status::Ok,text)));
        },
        "update" => {
            let data = try_json_get!(j, "data");
            return Ok(Response::with((status::Ok,"Feature not implemented")));
        },
        _ => {
            return Ok(Response::with((status::BadRequest,"Invalid command")));
        }
    }
}



pub fn set_value(req: &mut Request, in_ch_mut: &Mutex<SyncSender<Box<Query>>>) -> IronResult<Response> {
    debug!("Request for route '/json/set'");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let set_req : api_structs::SetRequest = json::decode(&payload).unwrap();

    let (q,rx) = new_set_query(set_req.key, set_req.value); //as (T, Receiver<String>;

    {
        let in_ch = in_ch_mut.lock().unwrap();
        in_ch.send(Box::new(q));
    }

    let result : Vec<u8> = rx.recv().unwrap();

    let response = api_structs::SetResponse{ status: String::from_utf8(result).unwrap() };
    let response_string = json::encode(&response).unwrap();

    debug!("Result for /set/json: {}", response_string);
    Ok(Response::with(status::Ok))
}
