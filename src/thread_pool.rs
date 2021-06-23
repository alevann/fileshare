use std::sync::{
    mpsc::{self, Sender, Receiver},
    Arc,
    Mutex
};
use std::thread::{self,JoinHandle};


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    // TODO: do this 
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Sending terminate message to all threads in pool");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            info!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    thread: Option<JoinHandle<()>>,
    id: usize,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = receiver
                .lock().expect("Attempted to lock Receiver<Job> but lock was poisoned")
                .recv().expect("Pool shutdown before its workers");
            info!("Worker {} got a job!", id);

            if let Message::NewJob(job) = msg {
                job()
            } else {
                break
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
