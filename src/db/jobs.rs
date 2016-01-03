use std::sync::mpsc::{channel, Sender, Receiver, SendError};
use core::query::{Query, GetQuery, SetQuery };

trait Job {
    fn into_query(self) -> Query;
    fn get_type(&self) -> JobType;
    fn send_result(&self) -> Result<(), SendError<u32>>;
}

enum JobType {
    Set,
    Get
}

pub struct GetJob {
    pub key: String,
    pub chan: Sender<u32>
}


pub struct SetJob {
    pub number: u32,
    pub chan: Sender<u32>
}

impl GetJob {
    fn new(key: String) -> (GetJob, Sender<u32>) {
        let (tx, rx) : (Sender<u32>, Receiver<u32>) = channel();
        (GetJob{ key: key, chan: tx }, tx)
    }
}


impl Job for GetJob {
    fn into_query(self) -> Query {
        Query::Get(GetQuery::new(self.key))
    }

    fn get_type(&self) -> JobType { JobType::Get }

    fn send_result(&self) -> Result<(), SendError<u32>> {

    }
}

impl Job for SetJob {
    fn into_query(self) -> Query {

    }

    fn get_type(&self) -> JobType { JobType::Set }

    fn send_result(&self) -> Result<(), SendError<u32>> {

    }
}
