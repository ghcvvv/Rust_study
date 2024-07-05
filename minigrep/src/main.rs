//grep 是 “Globally search a Regular Expression and Print.” 的首字母缩写。grep 最简单的使用场景是在特定文件中搜索指定字符串。
// 为此，grep 获取一个文件路径和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。


use std::{fs, process};
use std::env;
use minigrep::{Config, run};
//std::env::args 函数返回一个给定程序的命令行参数的迭代器，
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);//args[0]是程序名c
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config);
}

