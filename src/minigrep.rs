use std::env;
use std::fs;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents: String = fs::read_to_string(config.filename)
        .expect("Could't find file");

    println!("With text: \n{}", contents);
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Config { query, filename }
    }
}