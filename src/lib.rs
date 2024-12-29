pub mod errors;
pub mod middleware;
pub mod router;
pub mod pool;
pub mod http;
pub mod server;
pub mod parser;

pub use errors::Error;
pub type Result<T> = std::result::Result<T, Error>;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + 'static + Send>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
        F: Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(j) => {
                    println!("Worker {id} has recieved a job ; and he is working on it ..");
                    j();
                }
                Err(_) => {
                    eprint!("thread dying with dignity");
                    break;
                }
            };
        }) {
            Ok(t) => Some(t),
            Err(_) => {
                eprintln!("coudlnt spawn a new thread");
                None
            }
        };
        Worker { id, thread }
    }
}
