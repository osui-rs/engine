use osmium_engine::{prelude::*, utils::clear};

fn main() -> Result<()> {
    let mut console = init(true)?;

    let mut ui = app();

    console.render(&mut ui)?;

    loop {
        console.event(&mut ui, read()?)?;
    }

    console.close()
}

pub fn app() -> Component {
    let mut count = use_state(0);

    Box::new(move |frame| {
        frame.draw(
            Button {
                text: format!("The current count: {count}"),
                on_click: || count += 1,
            },
            Props::center(),
        );
    })
}

pub struct Button<F: FnMut()> {
    text: String,
    on_click: F,
}

impl<F: FnMut()> Element for Button<F> {
    fn render(&self, _size: (u16, u16)) -> String {
        self.text.to_owned()
    }

    fn event(&mut self, event: crossterm::event::Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => self.action(0),
            _ => (),
        }
    }

    fn action(&mut self, action: u16) {
        if action == 0 {
            (self.on_click)()
        }
    }
}
