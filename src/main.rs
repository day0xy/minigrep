// in main.rs
use std::env;

fn main() {
    //解析命令行参数
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("in file{}", file_path);
}
