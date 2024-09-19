use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

type Job = dyn FnOnce() + Send + 'static;

enum WorkerAssignment {
    Job(Box<Job>),
    Shutdown,
}

pub enum ThreadPoolError {
    Poisoned,
    JobSendFailure,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<WorkerAssignment>,
    panic_flag: Arc<Mutex<bool>>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        assert!(worker_count > 0);

        let (sender, receiver) = channel();
        let rx = Arc::new(Mutex::new(receiver));
        let panic_flag = Arc::new(Mutex::new(false));

        let mut workers = Vec::new();

        for _ in 0..worker_count {
            let worker = Worker::new(Arc::clone(&rx), Arc::clone(&panic_flag));
            workers.push(worker);
        }

        Self {
            workers,
            tx: sender,
            panic_flag,
        }
    }

    pub fn execute(&mut self, f: impl FnOnce() + Send + 'static) -> Result<(), ThreadPoolError> {
        if *self.panic_flag.lock().unwrap() {
            return Err(ThreadPoolError::Poisoned);
        }

        if self.tx.send(WorkerAssignment::Job(Box::new(f))).is_err() {
            return Err(ThreadPoolError::JobSendFailure);
        }

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            let _ = self.tx.send(WorkerAssignment::Shutdown);
        }

        self.workers.clear();
    }
}

/// Ensures the panic flag gets set when a job panics
struct WorkerPanicFlagger {
    panic_flag: Arc<Mutex<bool>>,
}

impl Drop for WorkerPanicFlagger {
    fn drop(&mut self) {
        *self.panic_flag.lock().unwrap() = true;
    }
}

struct Worker {
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(rx: Arc<Mutex<Receiver<WorkerAssignment>>>, panic_flag: Arc<Mutex<bool>>) -> Self {
        let thread_handle = std::thread::spawn(move || {
            // panic_flagger destructor gets called in the event of a panic or the thread shuts
            // down, preventing the thread pool from doing any more work
            let panic_flagger = WorkerPanicFlagger { panic_flag };

            'running: loop {
                if rx.is_poisoned() {
                    break 'running;
                }

                let message = rx.lock().unwrap().recv();

                let assignment = match message {
                    Ok(a) => a,
                    Err(_) => {
                        break 'running;
                    }
                };

                match assignment {
                    WorkerAssignment::Job(job) => job(),
                    WorkerAssignment::Shutdown => break 'running,
                };
            }
            drop(panic_flagger);
        });

        Self {
            thread_handle: Some(thread_handle),
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thd) = self.thread_handle.take() {
            let _ = thd.join();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let mut pool = ThreadPool::new(4);

        let mut had_error = false;

        for i in 0..4 {
            if let Err(_) = pool.execute(move || println!("#{}", i)) {
                had_error = true;
            }
        }

        assert!(!had_error);
    }

    #[test]
    fn panic_test() {
        let mut pool = ThreadPool::new(4);

        let mut had_error = false;

        for i in 0..4 {
            if let Err(_) = pool.execute(move || panic!("Intentional Poison {}", i)) {
                had_error = true;
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        if had_error {
            println!("Thread Pool Job Panicked!");
        }
        assert!(had_error);
    }
}
