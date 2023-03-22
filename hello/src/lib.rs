use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = reciver
                .lock()
                .expect("problem with locking. Thread id: {id}")
                .recv()
                .unwrap();

            println!("thread with id {} has a job", id);

            job();
        });

        Self { id: id, thread }
    }
}

impl ThreadPool {
    /// creates a new threadpool
    ///
    /// size is the amout of threads that you want
    ///
    /// # panics
    ///
    /// when size is zero. There should be at least 1 thread
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&reciever));

            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
