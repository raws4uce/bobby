use anyhow::Ok;
use std::thread;

use super::ThreadPool;

pub struct NaiveTP;

impl ThreadPool for NaiveTP {
    fn new(threads: usize) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(NaiveTP)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
