use std::fs;
use crate::grepper::domain::read::ReadContent;
use crate::grepper::domain::searcher::search::SearchContent;

pub struct FileContent;

impl ReadContent for FileContent {
    fn read(&self, haystack: &str) -> String {
        match fs::read_to_string(haystack.clone()) {
            Ok(content) => content,
            Err(error) => panic!("{}", error)
        }
    }
}

impl SearchContent for FileContent {
    fn search(&self, needle: &str, haystack: &str) -> Vec<String> {
        FileContent
            .read(haystack)
            .lines()
            .filter(|line: &&str| line.contains(needle))
            .map(|matches: &str| matches.to_string())
            .collect()
    }
}
