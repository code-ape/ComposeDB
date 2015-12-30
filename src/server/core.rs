
use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

use env_logger;
use iron::prelude::*;
use router::Router;

use server::routes::{get_value,set_value,ping};
use db::worker::{WorkerPool, Job};


pub fn run() {
    env_logger::init().unwrap();

    // let queue : Arc<Mutex<VecDeque<Job>>> = Arc::new(Mutex::new(VecDeque::new()));
    // let signal : Arc<Condvar> = Arc::new(Condvar::new());
    let (in_ch, out_ch) : (SyncSender<Job>, Receiver<Job>) = sync_channel(20);


    let num_workers = 3;
    let worker_queue_size = 2;

    let mut pool = WorkerPool::new(num_workers, worker_queue_size, out_ch);

    thread::Builder::new().name("Pool thread".to_string()).spawn(move || {
        pool.run();
    });

    let mut router = Router::new();

    // let signal_2 = signal.clone();
    // let signal_3 = signal.clone();
    let in_ch_2 = Mutex::new(in_ch.clone());
    let in_ch_3 = Mutex::new(in_ch.clone());

    router.get("/json", move |r: &mut Request| get_value( r, &in_ch_2 ));
    router.post("/json/set", move |r: &mut Request| set_value( r, &in_ch_3 ));
    router.get("/ping", move |r: &mut Request| ping(r));

    info!("Starting ComposeDB.");
    Iron::new(router).http("localhost:3000").unwrap();

    // for i in 0..num_workers {
    //     let queue_clone = queue.clone();
    //     let signal_clone = signal.clone();
    //
    //     thread::spawn(move || {
    //         debug!("Starting worker {}", i);
    //         let mut counter = 0;
    //         let mut j: Job;
    //         loop {
    //             loop {
    //                 let mut q = queue_clone.lock().unwrap();
    //                 q = signal_clone.wait(q).unwrap();
    //                 j = match q.pop_front() {
    //                     Some(x) => x,
    //                     None => continue
    //                 };
    //                 break;
    //             }
    //             debug!("Worker {}: received job {}", i, counter);
    //             let new_value = j.number + 100;
    //             thread::sleep_ms(10000);
    //             j.chan.send(new_value).unwrap();
    //             counter += 1;
    //         }
    //         //println!("Ending worker {}", i);
    //     });
    // }

}
