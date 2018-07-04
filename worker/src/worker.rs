use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub enum TerminateMessage {
    Terminate,
}

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    sender_terminate: mpsc::Sender<TerminateMessage>,
}

type Fp = Box<Fn() -> () + Send + Sync>;

struct Closure {
    fp: Fp,
}

impl Closure {
    fn call(&self) {
        (*self.fp)()
    }
}

impl Worker {
    pub fn start<F>(f: F) -> Worker
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        let (sender, receiver) = mpsc::channel();
        let closure = Arc::new(Closure { fp: Box::new(f) });

        let thread = thread::spawn(move || {
            let f = closure.clone();
            loop {
                f.call();
                if let Ok(TerminateMessage::Terminate) = receiver.try_recv() {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
            sender_terminate: sender,
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sender_terminate
            .send(TerminateMessage::Terminate)
            .unwrap();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn worker_drop() {
        println!("\n Begin to create thread defined by us");
        let check = Arc::new(Mutex::new(0));
        {
            let check_inner = check.clone();
            let _worker = Worker::start(move || {
                {
                    let mut inner = check_inner.lock().unwrap();
                    *inner += 1;
                    println!("\n check changed to {}", *inner);
                }
                thread::sleep(Duration::new(1, 0));
            });
            thread::sleep(Duration::new(3, 0));
        }
        assert_eq!(*check.lock().unwrap(), 3);
        println!("\n Thread is dropped automatically in graceful way now");
    }
}
