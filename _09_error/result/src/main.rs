use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Write};

fn main() {
    println!("用 Result 处理可恢复的错误");

    // 读取一个不存在的文件，返回Result.Err
    let greeting_file_result = File::open("README.md");
    println!("Open README.md result is {:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut fc) => {
                    fc.write("this is new hello.txt".as_bytes());
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("=========返回存在的=============");
    // 返回存在的
    let greeting_file_result =
        File::open("/Users/nohi/work/workspaces-nohi/rust/rust_start/README.md");
    println!("Open README.md result is {:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => panic!("Open README.md result is {:?}", err),
    };
    println!("Open README.md greeting_file is {:?}", greeting_file);

    println!("=========失败时 panic 的简写：unwrap 和 expect=============");
    // let greeting_file = File::open("hello11.txt").unwrap();

    println!("=========传播错误=============");
    let rs = read_username_from_file();
    println!("read username is {rs:?}");
    println!("=========传播错误的简写：? 运算符=============");
    let rs = read_username_from_file2();
    println!("1read username is {rs:?}");
    let rs = read_username_from_file3();
    println!("2read username is {rs:?}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
// 如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello1.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello1.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
