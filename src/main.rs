use std::sync::Arc;

use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(false)?;

    console.render(app())?;

    crossterm::event::read()?;

    console.close()
}

pub fn app() -> Ui {
    Arc::new(|frame| {
        frame.draw(&"Hello, World", &Props::center());
    })
}
