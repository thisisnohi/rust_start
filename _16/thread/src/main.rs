use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("使用线程运行代码");

    println!("创建线程..并运行");
    let handler = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("创建线程结束");
    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("等待线程结束");
    handler.join().unwrap();
    println!("线程结束");

    println!("\n将 move 闭包与线程一同使用");
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || println!("Here's a vector: {:?}", v));

    // v 已经被move
    // println!("Here's a vector: {:?}", v);

    handler.join().unwrap();

    println!("\n使用消息传递在线程间传送数据");

    let (tx, rx) = mpsc::channel();
    // 克隆tx,必须在tx使用前clone
    let tx1 = tx.clone();

    let handler = thread::spawn(move || {
        println!("thread run ");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for item in vals {
            tx.send(item).unwrap();
            // sleep 1s
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handler = thread::spawn(move || {
        println!("another thread run ");
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for item in vals {
            tx1.send(item).unwrap();
            // sleep 1s
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 阻塞读取
    let received = rx.recv().unwrap();
    println!("Got {}", received);
    // rx当作一个迭代器
    for rec in rx {
        println!("Got {}", rec);
    }

    handler.join().unwrap();
}
