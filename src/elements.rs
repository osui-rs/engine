use crate::Element;

impl Element for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Element for String {
    fn render(&self) -> String {
        self.to_owned()
    }
}
