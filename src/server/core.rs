
use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;
use std::path::Path;

use env_logger;
use iron::prelude::*;
use router::Router;

use lmdb::EnvBuilder;
use server::routes::{get_value,set_value,ping};
use db::worker::WorkerPool;
use core::query::{Query, GetQuery, SetQuery, QueryType};

fn receive_query(q: Box<Query>) -> Result<(),()> {
    match q.get_type() {
        QueryType::Get => {
            println!("A wild GET query appears!");
            Ok(())
        },
        QueryType::Set => {
            println!("A wild SET query appears!");
            Ok(())
        }
    }
}

pub fn run() {
    env_logger::init().unwrap();

    let (in_ch, out_ch) : (SyncSender<Box<Query>>, Receiver<Box<Query>>) = sync_channel(20);


    let path = Path::new("composedb_data");
    let db_env = EnvBuilder::new().open(&path, 0o777).unwrap();

    let num_workers = 3;
    let worker_queue_size = 2;

    let mut pool =
            WorkerPool::new(num_workers, worker_queue_size, out_ch, db_env, receive_query);

    thread::Builder::new().name("Pool thread".to_string()).spawn(move || {
        pool.run();
    });

    let mut router = Router::new();

    let in_ch_2 = Mutex::new(in_ch.clone());
    let in_ch_3 = Mutex::new(in_ch.clone());

    router.get("/json", move |r: &mut Request| get_value( r, &in_ch_2 ));
    router.post("/json/set", move |r: &mut Request| set_value( r, &in_ch_3 ));
    router.get("/ping", move |r: &mut Request| ping(r));

    info!("Starting ComposeDB.");
    Iron::new(router).http("localhost:3000").unwrap();

}
