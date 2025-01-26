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
            e => console.event = Some(e),
        }
    }

    console.close()
}

pub fn app() -> Ui {
    let mut count = use_state(0);
    Arc::new(move |frame| {
        frame.draw(
            &mut Button {
                on_click: func(move || count += 1),
                text: format!("Count: {count}"),
            },
            &Props::center().state(
                "hover",
                Style {
                    color: Color::Red,
                    ..Default::default()
                },
            ),
        );
    })
}
