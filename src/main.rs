use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(true)?;

    let mut count = use_state(0);

    let ui = app(count);

    loop {
        console.render(&ui)?;
        read()?;
        count += 1;
    }

    console.close()
}

pub fn app(count: State<i32>) -> Ui {
    Box::new(move |frame| {
        frame.draw(Box::new(format!("Events: {count}")), Props::center());
    })
}
