use std::io;

const MESSAGE: &str = "El Psy Kongroo";
const USERNAME_REQUEST: &str = "Enter your username";

fn main() {
    println!("{}", USERNAME_REQUEST);
    let  mut username = String::new();
    io::stdin().read_line(&mut username).expect("Couldn't read username");
    let username = username.trim();

    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Couldn't read name").to_string();
    let name = name.trim();

    println!("{} says {}", username, MESSAGE);
    println!("But the real name of {} is: {}", username, name);
}
