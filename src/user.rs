use std::io;

pub struct User {
    username: String,
    name: String
}

impl User {
    pub fn new(username: String, name: String) -> User {
        return User {username, name}
    }

    pub fn say_name(&self) {
        print!("Your name is {}", self.name);
    }

    pub fn say_username(&self) {
        print!("Your username is {}", self.username);
    }
}

pub fn ask_username() -> String {
    let mut username: String;
    println!("Enter your username");
    io::stdin().read_line(&mut username).expect("Couldn't read input");

    username
}

pub fn ask_name() -> String {
    let mut name: String;
    println!("Enter your name");
    io::stdin().read_line(&mut name).expect("Couldn't read input");
    name
}
