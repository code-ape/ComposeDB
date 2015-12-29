use std::collections::{VecDeque, };
use std::sync::{Mutex, Arc, Condvar};
use std::sync::mpsc::Sender;

struct WorkerPool<'a> {
    workers: Vec<&'a Worker>,
    recv_queue: Arc<Mutex<VecDeque<Job>>>,
    working_queue: Arc<Mutex<VecDeque<Job>>> //this shouldn't be a VecDeque
}


pub struct Job {
    pub number: u32,
    pub chan: Sender<u32>
}


struct Worker {
    id: u32,
    name: String,

}
