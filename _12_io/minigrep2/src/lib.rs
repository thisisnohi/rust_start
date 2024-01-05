use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // build 返回Result
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("请输入参数1.查询字符串 2.文件名");
        }
        // 从环境变量获取参数
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // args 为借用
        // Config定义为拥有所有权的String
        // 用最简单的clone方法，牺牲一小部分性能换取简洁性
        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        });
    }
}

// 处理逻辑
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    let mut rs = vec![];
    if config.ignore_case {
        rs = search_case_insensitive(&config.query, &content);
    } else {
        rs = search(&config.query, &content);
    }
    for line in rs {
        println!("{line}")
    }
    // 返回
    Ok(())
}

// 遍历内容的每一行文本。
// 查看这一行是否包含要搜索的字符串。
// 如果有，将这一行加入列表返回值中。
// 如果没有，什么也不做。
// 返回匹配到的结果列表
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 大小写不敏感
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_uppercase().contains(query.to_uppercase().as_str()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // 大小写敏感
    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /*
     大小写不敏感测试
    */
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
