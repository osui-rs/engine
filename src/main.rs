use std::sync::Arc;

use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(false)?;

    let ui = app();

    console.render(ui.clone())?;

    read()?;

    console.close()
}

pub fn app() -> Ui {
    Arc::new(move |frame| {
        frame.draw(&mut "test", &Props::center());
    })
}
