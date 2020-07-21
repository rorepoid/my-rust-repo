use std::io;

fn main() {
    println!("Enter your username:");
    let  mut username = String::new();
    io::stdin().read_line(&mut username).expect("Couldn't read username");
    let username = username.trim();

    println!("Enter your name:");
    let mut name = String::new();
    let variable: String = io::stdin().read_line(&mut name).expect("Couldn't read name").to_string();
    println!("variable content: {}", variable);
    let name = name.trim();

    println!("{} says El Psy Kongroo", username);
    println!("But the real name of {} is: {}", username, name);
}
