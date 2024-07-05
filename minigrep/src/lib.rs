use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
//我们返回的两个值是相关的并都是一个配置值的一部分。
// 目前除了将这两个值组合进元组之外并没有表达这个数据结构的意义：我们可以将这两个值放入一个结构体并给每个字段一个有意义的名字。

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();//检查环境变量
        let config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        };
        Ok(config)
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string("poem.txt").expect("Should have been able to read the file");//读取文件
    let mut results = Vec::new();
    if config.ignore_case {
        results = search_case_insensitive(&config.query, &content);
    } else {
        results = search(&config.query, &content);
    }
    for line in results {
        println!("{line}");
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();//将查询变为小写
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {//将内容变为小写
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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