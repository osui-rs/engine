use crate::Element;

impl Element for &str {
    fn render(&self, frame: &mut dyn crate::Framing) {
        frame.draw_str("test");
    }
}
