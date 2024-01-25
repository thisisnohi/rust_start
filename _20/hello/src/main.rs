use hello::ThreadPool;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, io, thread};

fn main() {
    println!("构建单线程 web server");
    println!("\n监听 TCP 连接");
    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| handler_connection(stream));
    }
    println!("Shutting down.");
}

/**
* 处理链接
*/
fn handler_connection_1(mut stream: TcpStream) {
    // BufReader 增加了缓存来替我们管理 std::io::Read trait 方法的调用。
    let bufferd_read = BufReader::new(&stream);
    println!("to read");
    let rs: Vec<_> = bufferd_read
        .lines()
        .map(|l| l.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!(
        "==============================\n{:#?}\n==============================",
        rs
    );
    println!("read finish");

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let content_length = content.len();
    let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
    stream.write_all(rs.as_bytes()).unwrap();
}

/**
 * 处理链接
 */
fn handler_connection2(mut stream: TcpStream) {
    // BufReader 增加了缓存来替我们管理 std::io::Read trait 方法的调用。
    let bufferd_read = BufReader::new(&stream);
    let line = bufferd_read.lines().next().unwrap().unwrap();
    if line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("hello.html").unwrap();
        let content_length = content.len();
        let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
        stream.write_all(rs.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap();
        let content_length = content.len();
        let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
        stream.write_all(rs.as_bytes()).unwrap();
    }
}

/**
 * 处理链接--单线程重构
 */
fn handler_connection3(mut stream: TcpStream) {
    // BufReader 增加了缓存来替我们管理 std::io::Read trait 方法的调用。
    let bufferd_read = BufReader::new(&stream);
    let line = bufferd_read.lines().next().unwrap().unwrap();
    let (status_line, resp_file) = if line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(resp_file).unwrap();
    let content_length = content.len();
    let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
    stream.write_all(rs.as_bytes()).unwrap();
}

/**
 * 处理链接--在当前 server 实现中模拟慢请求
 */
fn handler_connection_ok(mut stream: TcpStream) {
    // BufReader 增加了缓存来替我们管理 std::io::Read trait 方法的调用。
    let bufferd_read = BufReader::new(&stream);
    let request_line = bufferd_read.lines().next().unwrap().unwrap();
    let (status_line, resp_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    println!("{request_line} {resp_file}");
    let content = fs::read_to_string(resp_file).unwrap();
    let content_length = content.len();
    let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
    stream.write_all(rs.as_bytes()).unwrap();
}

/**
 * 处理链接--在当前 server 实现中模拟慢请求
 */
fn handler_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    // BufReader 增加了缓存来替我们管理 std::io::Read trait 方法的调用。
    let (status_line, resp_file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("{status_line} {resp_file}");
    let content = fs::read_to_string(resp_file).unwrap();
    let content_length = content.len();
    let rs = format!("{status_line}\r\nContent-Lenth:{content_length}\r\n\r\n{content}");
    stream.write_all(rs.as_bytes()).unwrap();
    stream.flush().unwrap();
}
