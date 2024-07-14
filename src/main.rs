// in main.rs
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

// fn parse_config(args: &[String]) -> Config {
//     //解析命令行参数
//     let query = args[1].clone();
//     //clone函数避免所有权的转移
//     let file_path = args[2].clone();

//     // 返回值表达式
//     Config { query, file_path }
// }

//struct Config method
impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len()<3{
    //         panic!("not enough arguments!");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
