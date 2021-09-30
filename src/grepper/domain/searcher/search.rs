pub trait SearchContent {
    fn search(&self, needle: &str, haystack: &str) -> Vec<String>;
}
