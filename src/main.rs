use std::env;
use std::process;

use my_rust_repo::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem passing arguments, {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = my_rust_repo::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
