fn main() {
    println!("高级函数与闭包");
    println!("\n函数指针");
    println!("5 add one is {}", add_one(5));
    println!("5 add twice is {}", add_twice(add_one, 5));

    let rs: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("{:?}", rs);

    println!("\n返回闭包");
    let f = return_closual();
    println!("{}", f(1));
    println!("{}", f(2));
}

fn return_closual() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn add_one(val: i32) -> i32 {
    val + 1
}

fn add_twice(f: fn(i32) -> i32, val: i32) -> i32 {
    f(val) + f(val)
}
