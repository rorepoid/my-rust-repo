pub struct Request<'a> {
    pub needle: &'a str,
    pub haystack: &'a str,
    pub searcher_type: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Request<'a> {
    pub fn new(needle: &'a str, haystack: &'a str, searcher_type: &'a str, case_sensitive: bool) -> Request<'a> {
        Request {
            needle,
            haystack,
            searcher_type,
            case_sensitive,
        }
    }
    pub fn needle(&self) -> &'a str {
        self.needle
    }
    pub fn haystack(&self) -> &'a str {
        self.haystack
    }
    pub fn searcher_type(&self) -> &'a str {
        self.searcher_type
    }
}
