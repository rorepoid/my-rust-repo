use std::io;

const MESSAGE: &str = "El Psy Kongroo";
const NAME_REQUEST: &str = "Enter your name";
const USERNAME_REQUEST: &str = "Enter your username";

fn main() {
    // ask for username
    let username = ask(USERNAME_REQUEST);

    // ask for name
    println!("{}", NAME_REQUEST);
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Couldn't read name").to_string();
    let name = name.trim();

    println!("{} says {}", username, MESSAGE);
    println!("But the real name of {} is: {}", username, name);
}

fn ask(request: &str) -> String {
    println!("{}", request);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Couldn't read input");
    let response = response.trim().to_string();
    return response;
}
