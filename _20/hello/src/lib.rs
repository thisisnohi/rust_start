use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建线程池
    /// 线程池中线程的数量由size指定
    /// # Panics
    /// `new` 函数在size为0时会panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let rec1 = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let arc_rec = Arc::clone(&rec1);
            workers.push(Worker::new(id, arc_rec));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // 与 self.sender.as_ref().unwrap().send(job).unwrap(); 结果相同
        // if let Some(sender)= &self.sender {
        //     sender.send(job).unwrap();
        // }
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        // 释放sender
        drop(self.sender.take());

        for item in &mut self.workers {
            println!("shutting worker {}", item.id);
            // 可以使用，但不优雅
            // item.handler.take().unwrap().join().unwrap();
            if let Some(thread) = item.handler.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    handler: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} got a job, executing");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down....");
                    break;
                }
            }

        });
        Worker {
            id,
            handler: Some(thread),
        }
    }
}
