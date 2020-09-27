use std::io;

pub struct User {
    username: String,
    name: String
}

impl User {
    pub fn new(username: String, name: String) -> User {
        return User {username, name}
    }

    pub fn ask_username(&mut self) {
        println!("Enter your name");
        io::stdin().read_line(&mut self.username).expect("Couldn't read input");
    }

    pub fn ask_name(&mut self) {
        println!("Enter your name");
        io::stdin().read_line(&mut self.name).expect("Couldn't read input");
    }

    pub fn say_name(&self) {
        print!("{}", self.name);
    }

    pub fn say_username(&self) {
        print!("{}", self.username);
    }
}
