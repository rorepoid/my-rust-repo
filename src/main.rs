use std::io;

const MESSAGE: &str = "El Psy Kongroo";
const NAME_REQUEST: &str = "Enter your name";
const USERNAME_REQUEST: &str = "Enter your username";

fn main() {
    // ask for username
    println!("{}", USERNAME_REQUEST);
    let mut username = String::new();

    io::stdin().read_line(&mut username).expect("Couldn't read username");
    let username = username.trim();

    // ask for name
    println!("{}", NAME_REQUEST);
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Couldn't read name").to_string();
    let name = name.trim();

    println!("{} says {}", username, MESSAGE);
    println!("But the real name of {} is: {}", username, name);
}
