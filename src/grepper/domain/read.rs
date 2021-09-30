pub trait ReadContent {
    fn read(&self, haystack: &str) -> String;
}
