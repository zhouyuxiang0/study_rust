use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // env::args() 返回一个迭代器 迭代器可以使用collect收集
    // let args: Vec<String> = env::args().collect();

    // 将处理命令行参数 移动到lib.rs中
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
