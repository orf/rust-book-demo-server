use std::fmt;
use std::thread;

extern crate crossbeam_channel;

use crossbeam_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct PoolCreationError {
    cause: String
}

impl PoolCreationError {
    pub fn new(message: String) -> PoolCreationError {
        PoolCreationError { cause: message }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.cause)
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}


type Job = Box<FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Receiver<Job>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.recv().unwrap();
                job.call_box();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}


impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Result::Err(PoolCreationError::new(format!("Size {} is invalid", size)));
        }

        let (sender, receiver) = crossbeam_channel::unbounded();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()))
        }

        return Result::Ok(ThreadPool {
            workers,
            sender,
        });
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
