use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct User{
    name: String
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl User {
    pub fn new(size: usize) -> User {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        User { "Name", workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker { id, thread }
    }
}

