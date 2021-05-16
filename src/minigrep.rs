use std::env;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];
    let filename: &str = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}