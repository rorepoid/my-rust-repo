use std::io;
mod greetings;

const MESSAGE: &str = greetings::english();
const NAME_REQUEST: &str = "Enter your name";
const USERNAME_REQUEST: &str = "Enter your username";

fn main() {
    let username: String = ask(USERNAME_REQUEST);
    let name: String = ask(NAME_REQUEST);

    answer(&username, &name)
}

fn ask(request: &str) -> String {
    println!("{}", request);
    let mut response: String = String::new();
    io::stdin().read_line(&mut response).expect("Couldn't read input");
    let response: String = response.trim().to_string();

    response
}

fn answer(username: &str, name: &str) {
    println!("{} says {}", username, MESSAGE);
    println!("But the real name of {} is: {}", username, name);
}
