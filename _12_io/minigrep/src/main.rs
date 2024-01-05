use std::error::Error;
use std::{env, fs, process};

fn main() {
    println!("一个 I/O 项目：构建一个命令行程序");

    // // 1. 接收命令行参数
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // if &args.len() < &3 {
    //     println!("请输入参数1.查询字符串 2.文件名");
    //     exit(1);
    // }
    // // 2.存储参数
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("query[{query}] from {file_path}");
    //
    // // 3.读取文件
    // let content = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    // println!("file[{file_path}]:\n======================================\n{content}\n======================================");

    // 1. 解析参数
    let args: Vec<String> = env::args().collect();
    // 方式2 提取参数解析器
    // let (query, file_path) = parse_config(&args);
    // println!("query[{query}] from {file_path}");
    // 方式3 组合配置值
    // let config = parse_config(&args);
    // 方式4 创建一个 Config 的构造函数
    //let config = Config::new(&args);
    //println!("config[{:?}]", config);
    // 方式5 build创建，返回Result
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    // 2. 读取文件
    // run(&config).unwrap_or_else(|err| {
    //     println!("程序异常:{err}");
    //     process::exit(1);
    // });
    // 使用if let
    if let Err(e) = run(&config) {
        println!("程序异常:{e}");
        process::exit(1);
    }
}

//
fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    println!("file[{}]:\n======================================\n{content}\n======================================", config.file_path);

    // 返回
    Ok(())
}

// // 解析参数
// fn parse_config(args: &[String]) -> (&str, &str) {
//     dbg!(&args);
//     if &args.len() < &3 {
//         println!("请输入参数1.查询字符串 2.文件名");
//         exit(1);
//     }
//     (&args[1], &args[2])
// }

// 解析参数
#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}
// fn parse_config(args: &[String]) -> Config {
//     dbg!(&args);
//     if &args.len() < &3 {
//         println!("请输入参数1.查询字符串 2.文件名");
//         exit(1);
//     }
//     // args 为借用
//     // Config定义为拥有所有权的String
//     // 用最简单的clone方法，牺牲一小部分性能换取简洁性
//     Config {
//         query: args[1].clone(),
//         file_path: args[2].clone(),
//     }
// }
impl Config {
    // 构造函数
    fn new(args: &[String]) -> Config {
        dbg!(&args);
        if &args.len() < &3 {
            //println!("请输入参数1.查询字符串 2.文件名");
            panic!("请输入参数1.查询字符串 2.文件名")
        }
        // args 为借用
        // Config定义为拥有所有权的String
        // 用最简单的clone方法，牺牲一小部分性能换取简洁性
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
    // build 返回Result
    fn build(args: &[String]) -> Result<Config, &'static str> {
        dbg!(&args);
        if args.len() < 3 {
            println!("请输入参数1.查询字符串 2.文件名");
            return Result::Err("请输入参数1.查询字符串 2.文件名");
        }
        // args 为借用
        // Config定义为拥有所有权的String
        // 用最简单的clone方法，牺牲一小部分性能换取简洁性
        return Result::Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}
