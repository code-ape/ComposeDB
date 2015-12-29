
use std::sync::{Arc};
use std::sync::mpsc::{channel, sync_channel, SyncSender, Sender, Receiver};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

use std::thread;

struct WorkerPool {
    workers: Vec<Worker>,
    recv_queue: Receiver<Job>,
    running: Arc<AtomicBool>
}

impl WorkerPool {
    pub fn new(num_workers: usize, worker_queue_size: usize, recv_queue: Receiver<Job>) -> WorkerPool {
        let mut workers_vec : Vec<Worker> = Vec::with_capacity(num_workers);

        for i in 0..num_workers {
            let w = Worker::new(i, worker_queue_size);
            workers_vec.push(w);
        }

        return WorkerPool {
            workers: workers_vec,
            recv_queue: recv_queue,
            running: Arc::new(AtomicBool::new(false))
        };
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::AcqRel)
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        println!("Attempting to start workers.");

        if self.is_running() {
            return Err("Worker pool already runnng.");
        }

        for worker in &mut self.workers {
            worker.start().unwrap();
        }

        println!("Workers started.");

        println!("Starting pool head");


        loop {
            let worker_slots: Vec<(usize, usize)> = Vec::new();
            let mut counter : usize = 0;
            for worker in &self.workers {
                worker_slots.push(( counter, worker.get_queue_availability() ));
                counter += 1;
            }

            println!("Pool head receiving job.");
            let j : Job = match self.recv_queue.recv() {
                Ok(x) => x,
                RecvErr => break
            };
            queue_available.fetch_add(1, Ordering::SeqCst);

            j.chan.send(j.number*j.number);
        }
        println!("Pool head ended");

        Ok(())

    }

}

pub struct Job {
    pub number: u32,
    pub chan: Sender<u32>
}

struct Worker {
    id: usize,
    name: String,
    running: Arc<AtomicBool>,
    alive: Arc<AtomicBool>,
    queue_in: Option<Sender<Job>>,
    queue_size: usize,
    queue_available: Arc<AtomicUsize>
}

impl Worker {
    fn new(id: usize, queue_size: usize) -> Worker {

        Worker {
            id: id,
            name: format!("Worker number {}", id),
            running: Arc::new(AtomicBool::new(false)),
            alive: Arc::new(AtomicBool::new(true)),
            queue_in: None,
            queue_size: queue_size,
            queue_available: Arc::new(AtomicUsize::new(queue_size))
        }
    }

    fn start(&mut self) -> Result<(), &'static str> {
        if self.is_running() {
            return Err("Worker already running");
        }

        let (in_ch, out_ch) : (Sender<Job>, Receiver<Job>) = channel();

        self.queue_in = Some(in_ch);

        let id = self.id;
        let running = self.running.clone();
        let alive = self.alive.clone();
        let queue_available = self.queue_available.clone();

        thread::spawn(move || {
            loop {
                println!("Worker {} receiving job.", id);
                let j : Job = match out_ch.recv() {
                    Ok(x) => x,
                    RecvErr => break
                };
                queue_available.fetch_add(1, Ordering::SeqCst);

                j.chan.send(j.number*j.number);
            }

            running.store(false, Ordering::SeqCst);
            alive.store(false, Ordering::SeqCst);

            println!("Worker {} exiting.", id);
        });

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::AcqRel)
    }

    fn get_queue_availability(&self) -> usize {
        self.queue_available.load(Ordering::Acquire)
    }

    fn get_queue_in(&self) -> &Sender<Job> {
        match self.queue_in.as_ref() {
            Some(x) => x,
            None => panic!("Tried to get queue from worker before it started!")
        }
    }

    fn give_job(&self, j: Job) -> Result<(), &'static str> {
        if self.get_queue_availability() > 0 {
            return Err("Queue full for worker");
        }
        let q = self.get_queue_in();
        q.send(j);
        Ok(())
    }
}
