//! This Module includes functions and structs implementing the connections for the
//! threaded server.

pub mod check_connections {
    use std::thread;
    use std::sync::{Arc, Mutex, mpsc};

    /// The ThreadPool struct includes a vector of type worker and a sender of type mspc (which
    /// is a queue used for communicating between threads
    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }


    /// The Job type includes the traits FnOnce, which means it is meant to be called once
    /// for every instance, and the Send trait which transfers info between threads
    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
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



    /// The Worker struct is part of the ThreadPool struct, and its job is to spawn new threads
    /// from said ThreadPool. Each worker has its ID and a thread.
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job();
            });

            Worker { id, thread }
        }
    }
}