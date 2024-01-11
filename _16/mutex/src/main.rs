use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("共享状态并发");

    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));
    println!("m = {:?}", counter);
    {
        let mut num = counter.lock().unwrap();
        *num = 1;
    }
    println!("m = {:?}", counter);

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let hander = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(hander);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("m = {:?}", counter.lock().unwrap());
}
