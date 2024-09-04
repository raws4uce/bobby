use anyhow::Ok;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::ThreadPool;

pub struct TP {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool for TP {
    fn new(threads: usize) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        assert!(threads > 0);

        let (sender, reciever) = mpsc::channel::<Job>();

        let mut workers: Vec<Worker> = Vec::with_capacity(threads);

        let reciever = Arc::new(Mutex::new(reciever));

        for id in 0..threads {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        Ok(TP {
            threads: workers,
            sender,
        })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        self.sender.send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();

            job();
        });

        Worker { id, thread }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
