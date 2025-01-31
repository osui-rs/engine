use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(true)?;

    let (mut count, ui) = app();

    loop {
        console.render(&ui)?;
        read()?;
        count += 1;
    }

    console.close()
}

pub fn app() -> (State<i32>, Ui) {
    let (count, count_ui) = counter();

    (
        count,
        Box::new(move |frame| {
            frame.draw(&count_ui, &Props::center());
        }),
    )
}

pub fn counter() -> (State<i32>, Ui) {
    let count = use_state(0);

    (
        count,
        Box::new(move |frame| {
            frame.draw(&format!("Events: {count}").as_str(), &Props::center());
        }),
    )
}
