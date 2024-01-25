use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    handler: Option<JoinHandle<()>>,
}

impl Worker {
    fn build(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        println!("创建线程[{id}]");
        let handler = thread::spawn(move || loop {
            let job = receiver.as_ref().lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} got a job, executing");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down....");
                    // 不停读取job,直到异常时退出循环
                    break;
                }
            }


            // if let rec = receiver.as_ref().lock() {
            //     if let Ok(job) = rec.unwrap().recv() {
            //         println!("Worker {id} got a job, executing");
            //         job();
            //     } else {
            //         println!("Worker {id} disconnected; shutting down....");
            //     }
            // };
        });
        Worker {
            id,
            handler: Some(handler),
        }
    }
}

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    threads: Vec<Worker>,
}

impl ThreadPool {
    /// 创建指定大小线程池
    /// 线程池大小由size决定
    pub fn new(size: usize) -> ThreadPool {
        println!("创建线程池[{size}]");
        // 创建线程池变量
        let mut threads: Vec<Worker> = Vec::with_capacity(size);
        // 创建通讯
        let (sender, receiver) = mpsc::channel();
        let rec = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            // clone 后传递进每一工作线程中
            let rec_clone = Arc::clone(&rec);
            threads.push(Worker::build(id, rec_clone));
        }
        ThreadPool {
            sender: Some(sender),
            threads,
        }
    }

    /// 执行任务
    /// 参见thread::spawn方法
    /// 些方法暂无返回值
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        if let Some(sender) = self.sender.as_ref() {
            match sender.send(job) {
                Ok(_) => {
                    println!("消息发送成功")
                }
                Err(_) => {
                    println!("消息发送失败")
                }
            }
        }
    }
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    //         F: FnOnce() -> T,
    //         F: Send + 'static,
    //         T: Send + 'static,
    // {
    //     Builder::new().spawn(f).expect("failed to spawn thread")
    // }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("==========线程将要关闭============");

        // 释放发送端
        drop(self.sender.take());

        for worker in &mut self.threads {
            // worker.handler.take().unwrap().join();
            if let Some(handler) = worker.handler.take() {
                handler.join().unwrap();
                println!("线程[{}]已关闭完成", worker.id);
            } else {
                println!("线程[{}]已关闭", worker.id);
            }
        }
    }
}
