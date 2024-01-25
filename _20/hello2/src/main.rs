use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use hello2::ThreadPool;

fn main() {
    println!("不看教程,手写http server");

    // 创建TCP监听
    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    let pool = ThreadPool::new(4);
    let mut index = 0;
    // 获取链接
    loop {
        let (tcp_stream, socket_addr) = listener.accept().unwrap();
        println!("Client[{}] connected", socket_addr);

        // 线程执行
        // thread::spawn(||handle_connection(tcp_stream));
        // 线程池执行
        pool.execute(||handle_connection(tcp_stream));
        if index >= 5 {
            println!("===> 接收{index}请求后，不在接收");
            break;
        }
        index += 1;
    }

    // 同样可以循环获取链接，不过没有socket_addr
    // take(5) 只接手5次数据
    // for stream in listener.incoming() {
    // for stream in listener.incoming().take(5) {
    //     match stream {
    //         Ok(stream) => {
    //             handle_connection(stream);
    //         }
    //         Err(e) => { /* connection failed */ }
    //     }
    // }
    println!("Main thread exit`");
}

fn handle_connection(mut stream: TcpStream) {
    // read_to_string 一直读取流中内容，直接EOF。
    // let mut buffer = String::new();
    // stream.read_to_string(&mut buffer);
    // println!("{}", buffer);

    // 读取指定长度流
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    // println!("start {}", "=".repeat(100));
    // println!(
    //     "读取长度为[{size}] 内容如下:\n{}",
    //     String::from_utf8_lossy(&buffer)
    // );
    // println!("end {}", "=".repeat(100));
    // 返回响应
    let (status_line, file_path) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 OK", "404.html")
    };
    println!("{status_line} {file_path}");

    // 返回
    let content = fs::read_to_string(file_path).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
