use crate::grepper::domain::read::ReadContent;
use crate::grepper::domain::searcher::search::SearchContent;

pub struct StringContent;

impl ReadContent for StringContent {
    fn read(&self, haystack: &str) -> String {
        haystack.to_string()
    }
}

impl SearchContent for StringContent {
    fn search(&self, needle: &str, haystack: &str) -> Vec<String> {
        StringContent
            .read(haystack)
            .lines()
            .filter(|line: &&str| line.contains(needle))
            .map(|matches: &str| matches.to_string())
            .collect()
    }
}
