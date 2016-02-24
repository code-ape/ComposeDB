
use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;
use std::path::Path;

use env_logger;
use iron::prelude::*;
use router::Router;

use server::routes::{get_handle,post_handle,ping};
use db::worker::WorkerPool;
use core::db::DB;
use core::query::{Query, run_query};


pub fn run() {
    env_logger::init().unwrap();

    let (in_ch, out_ch) : (SyncSender<Box<Query>>, Receiver<Box<Query>>) = sync_channel(20);

    let path = Path::new("composedb_data");
    let db = DB::new(path);

    let num_workers = 3;
    let worker_queue_size = 2;

    let run_query = move |q: Box<Query>| run_query(q, db.clone());

    let mut pool = WorkerPool::new(num_workers, worker_queue_size,
                                   out_ch, run_query);

    thread::Builder::new().name("Pool thread".to_string()).spawn(move || {
        pool.run();
    });

    let mut router = Router::new();

    let in_ch_2 = Mutex::new(in_ch.clone());
    let in_ch_3 = Mutex::new(in_ch.clone());
    let in_ch_4 = Mutex::new(in_ch.clone());

    router.get("/json", move |r: &mut Request| get_handle( r, &in_ch_2 ));
    router.post("/json", move |r: &mut Request| post_handle( r, &in_ch_3 ));
    router.get("/ping", move |r: &mut Request| ping(r));

    info!("Starting ComposeDB.");
    Iron::new(router).http("localhost:3000").unwrap();

}
