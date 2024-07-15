use std::{env, process};

use minigrep::Config;
fn main() {

    //直接传入args迭代器
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }
}
