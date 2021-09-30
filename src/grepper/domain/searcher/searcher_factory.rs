use crate::grepper::domain::searcher::search::SearchContent;
use crate::grepper::domain::file_content::FileContent;
use crate::grepper::domain::string_content::StringContent;

pub struct SearcherFactory;
impl SearcherFactory {
    pub fn get(searcher_name: &str) -> Box<dyn SearchContent> {
        match searcher_name {
            "file" => Box::new(FileContent),
            "string" => Box::new(StringContent),
            "clipboard" => Box::new(FileContent),
            _ => panic!("Could not find searcher type"),
        }
    }
}