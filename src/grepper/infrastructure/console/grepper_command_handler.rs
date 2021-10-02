use std::env;
use std::process;

use crate::grepper::infrastructure::shared::grepper_request::Request;
use crate::grepper::infrastructure::shared::grepper_requester::{GrepperRequester, Requester};
use crate::grepper::infrastructure::shared::grepper_response::Response;

pub fn handle() {
    let args: Vec<String> = env::args().collect();
    let request: Request = get_request(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem passing arguments, {}", err);
        process::exit(1);
    });
    let response: Response = Requester::generate_report(request);

    for line in response.matches() {
        println!("{}", line);
    }
}

pub fn get_request(args: &[String]) -> Result<Request, &str> {
    if args.len() < 4 {
        return Err("Not enough arguments");
    }
    let needle: &str = args[1].as_str();
    let haystack: &str = args[2].as_str();
    let searcher_type: &str = args[3].as_str();
    let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

    Ok(Request::new(
        needle,
        haystack,
        searcher_type,
        case_sensitive,
    ))
}
