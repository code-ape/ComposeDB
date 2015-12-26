use std::sync::mpsc::Sender;

pub struct Job {
    pub number: u32,
    pub chan: Sender<u32>
}
