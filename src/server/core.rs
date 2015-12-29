
use std::collections::VecDeque;
use std::sync::{Mutex, Arc, Condvar};
use std::thread;

use env_logger;
use iron::prelude::*;
use router::Router;

use server::routes::{get_value,set_value};
use server::jobs::Job;


pub fn run() {
    env_logger::init().unwrap();

    let queue : Arc<Mutex<VecDeque<Job>>> = Arc::new(Mutex::new(VecDeque::new()));
    let signal : Arc<Condvar> = Arc::new(Condvar::new());


    let num_workers = 3;

    for i in 0..num_workers {
        let queue_clone = queue.clone();
        let signal_clone = signal.clone();

        thread::spawn(move || {
            debug!("Starting worker {}", i);
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
                debug!("Worker {}: received job {}", i, counter);
                let new_value = j.number + 100;
                thread::sleep_ms(10000);
                j.chan.send(new_value).unwrap();
                counter += 1;
            }
            //println!("Ending worker {}", i);
        });
    }

    let mut router = Router::new();

    let queue_2 = queue.clone();
    let queue_3 = queue.clone();
    let signal_2 = signal.clone();
    let signal_3 = signal.clone();

    router.get("/json", move |r: &mut Request| get_value(r, &queue_2, &signal_2));
    router.post("/json/set", move |r: &mut Request| set_value(r, &queue_3, &signal_3));

    info!("Starting ComposeDB.");
    Iron::new(router).http("localhost:3000").unwrap();

}
