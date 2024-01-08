use std::{env, process};

fn main() {
    // 1. 解析参数
    let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        // 标准错误输出
        eprintln!("解析参数错误: {err}");
        process::exit(1);
    });

    // 2. 读取文件
    // 使用if let
    if let Err(e) = minigrep::run(&config) {
        // 标准错误输出
        eprintln!("程序运行错误: {e}");
        process::exit(1);
    }
}
