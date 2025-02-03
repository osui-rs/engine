use crate::Element;

impl Element for &str {
    fn render(&self, frame: &mut String) {
        *frame += self;
    }
}

impl Element for String {
    fn render(&self, frame: &mut String) {
        *frame += self;
    }
}
