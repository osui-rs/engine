use crossterm::event::Event;

pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub event: Option<Event>,
}

impl Frame {
    pub fn new((width, height): (u16, u16)) -> Frame {
        Frame {
            width,
            height,
            event: None,
        }
    }

    pub fn draw_component(&self, component: &mut crate::Component) {
        component(self)
    }

    pub fn draw<E: crate::Element>(&self, mut element: E, props: crate::Props) {
        if let Some(event) = &self.event {
            element.event(event.to_owned());
        }

        let s = element.render((props.width.get(self.width), props.height.get(self.height)));

        let (width, height) = crate::utils::str_size(&s);

        let (width, height) = (
            props.width.get_after(width, self.width),
            props.height.get_after(height, self.height),
        );

        let (x, y) = (
            props.x.get(width, self.width),
            props.y.get(height, self.height),
        );

        for (i, line) in s.lines().enumerate() {
            println!("\x1b[{};{}H{}\x1b[0m", (y + 1) as usize + i, x + 1, line,);
        }
    }
}
