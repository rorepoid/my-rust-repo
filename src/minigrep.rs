use std::env;
use std::fs;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    let (query, filename): (&str, &str) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents: String = fs::read_to_string(filename)
        .expect("Could't find file");

    println!("With text: \n{}", contents);
}

pub fn parse_config(args: &[String]) -> (&str, &str) {
    let query: &str = &args[1];
    let filename: &str = &args[2];

    (query, filename)
}
