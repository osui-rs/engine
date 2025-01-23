use std::sync::Arc;

use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(true)?;

    let ui = app();

    loop {
        console.render(ui.clone())?;
        if let crossterm::event::Event::Mouse(crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Moved,
            row,
            column,
            ..
        }) = crossterm::event::read()?
        {
            console.mouse_position = Some((column, row));
        } else {
            break;
        }
    }

    console.close()
}

pub fn app() -> Ui {
    Arc::new(|frame| {
        frame.draw(
            &"Hello, World",
            &Props::center().state("hover", Style { color: Color::Blue }),
        );
    })
}
