use anyhow::Result;

pub mod TP;
pub mod naive;

pub trait ThreadPool {
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
