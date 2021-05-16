use std::env;
use std::fs;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];
    let filename: &str = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents: String = fs::read_to_string(filename)
        .expect("Could't find file");

    println!("With text: \n{}", contents);
}
