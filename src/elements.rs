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

pub struct Button {
    pub on_click: crate::Handler<dyn FnMut()>,
    pub text: String,
}

impl Element for Button {
    fn render(&self) -> String {
        self.text.to_owned()
    }
    fn event(&self, _: crossterm::event::Event) {
        (self.on_click.clone().lock().unwrap())()
    }
}
