use std::sync::Arc;

use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(true)?;

    let ui = app();

    loop {
        console.render(ui.clone())?;
        match crossterm::event::read()? {
            crossterm::event::Event::Mouse(crossterm::event::MouseEvent {
                kind: crossterm::event::MouseEventKind::Moved,
                row,
                column,
                ..
            }) => console.mouse_position = Some((column, row)),
            crossterm::event::Event::Resize(_, _) => {}
            _ => break,
        }
    }

    console.close()
}

pub fn app() -> Ui {
    Arc::new(|frame| {
        frame.draw(&"this is centered!", &Props::center());
    })
}
