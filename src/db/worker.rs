
use std::sync::{Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

use std::thread;
use std::time::Duration;

pub struct WorkerPool {
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
        self.running.load(Ordering::Acquire)
    }

    fn worker_has_opening(&self) -> bool {
        match self.worker_with_opening() {
            Some(_) => true,
            None => false
        }
    }

    fn worker_with_opening(&self) -> Option<usize> {
        let mut counter : usize = 0;
        for worker in &self.workers {
            if worker.get_queue_availability() > 0 {
                return Some(counter);
            }
            counter += 1;
        }
        None
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        debug!("Attempting to start workers.");

        if self.is_running() {
            return Err("Worker pool already runnng.");
        }

        for worker in &mut self.workers {
            worker.start().unwrap();
        }

        debug!("Workers started.");

        debug!("Starting pool head");

        loop {
            let mut worker_slots: Vec<(usize, usize)> = Vec::new(); // TODO: allow this to be reordered
            let mut counter : usize = 0;
            let mut total_slots : usize = 0;
            for worker in &self.workers {
                let num_slots = worker.get_queue_availability();
                total_slots += num_slots;
                worker_slots.push(( counter, num_slots ));
                counter += 1;
            }

            let j : Job = match self.recv_queue.recv() {
                Ok(x) => {
                    debug!("Pool head: job received");
                    x
                },
                _ => {
                    error!("Pool head: job failed to be received");
                    break;
                }
            };

            match total_slots {
                0 => {
                    let mut keep_looping = true;
                    let mut worker_num : usize = 0;
                    while keep_looping {
                        match self.worker_with_opening() {
                            Some(x) => {
                                worker_num = x;
                                keep_looping = false;
                            },
                            None => {
                                thread::sleep(Duration::from_millis(1));
                            }
                        }

                    }
                    let ref worker = self.workers[worker_num];
                    match worker.give_job(j) {
                        Err(_) => panic!("Failed to give worker job"),
                        Ok(_) => {}
                    };
                },
                1 => {
                    let (worker_num, _) = worker_slots[0];
                    let ref worker = self.workers[worker_num];
                    worker.give_job(j);
                },
                x => {
                    debug!("More than 1 worker slot available.");
                    let mut jobs : Vec<Job> = Vec::new();
                    jobs.push(j);

                    for _ in 0..(x-1) {
                        match self.recv_queue.try_recv() {
                            Ok(x) => jobs.push(x),
                            Err(_) => break
                        }
                    }

                    debug!("{} jobs obtained.", jobs.len());
                    let mut jobs_left = true;

                    for (worker_num, num_available) in worker_slots {
                        let ref worker = self.workers[worker_num];
                        for _ in 0..num_available {
                            match jobs.pop() {
                                Some(x) => {
                                    debug!("Giving worker {} job.", worker_num);
                                    worker.give_job(x).unwrap();
                                },
                                None => {
                                    jobs_left = false;
                                    break;
                                }
                            }
                        }
                        if !jobs_left {
                            break;
                        }
                    }


                }
            }



        }
        debug!("Pool head ended");

        Ok(())

    }

}

// TODO: allow different job types how? Enum inside job? Or make Job a enum?
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

    // TODO: how do we reclaim queue if worker dies?
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

        thread::Builder::new().name(format!("Worker {}", id)).spawn(move || {
            debug!("Worker {}: starting", id);
            loop {
                let j : Job = match out_ch.recv() {
                    Ok(x) => {
                        debug!("Worker {}: job received", id);
                        x
                    },
                    _ => {
                        error!("Worker {}: job failed to be received", id);
                        break;
                    }
                };

                queue_available.fetch_add(1, Ordering::SeqCst);

                // TODO: so this needs to actually access the database now
                let resp = j.chan.send(j.number*j.number);
                if resp.is_err() {
                    error!("Worker {}: couldn't reply to job", id);
                }
            }

            running.store(false, Ordering::SeqCst);
            alive.store(false, Ordering::SeqCst);

            debug!("Worker {} exiting.", id);
        });

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
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
        if self.get_queue_availability() == 0 {
            return Err("Queue full for worker");
        }
        let q = self.get_queue_in();
        q.send(j);
        self.queue_available.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }
}
