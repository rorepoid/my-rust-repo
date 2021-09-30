pub struct Response {
    matches: Vec<String>,
}

impl Response {
    pub fn new(matches: Vec<String>) -> Self {
        Response { matches }
    }

    pub fn matches(&self) -> &Vec<String> {
        &self.matches
    }
}
