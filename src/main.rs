mod greetings;
mod user;

fn main() {
    let username: String = "".to_string();
    let name: String = "".to_string();

    let mut user: user::User = user::User::new(username, name);
    user.ask_username();
    user.ask_name();
    user.say_username();
    user.say_name();
}
