fn main() {
    println!("用 panic! 处理不可恢复的错误");

    // panic!("这里是异常处理")

    println!("使用 panic! 的 backtrace");
    let v = vec![1, 2, 3];
    v[99];
}
