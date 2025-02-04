use osmium_engine::prelude::*;

fn main() -> Result<()> {
    let mut console = init(false)?;

    let mut ui = app(use_state(0));

    console.render(&mut ui)?;

    read()?;

    console.close()
}

pub fn app(mut _count: State<i32>) -> Component {
    let mut my_test = test();
    Box::new(move |frame| {
        if let Some(_) = frame.event {
            _count += 1;
        }

        frame.draw(&mut my_test, Props::center_vertical());

        my_test(frame);
    })
}

fn test() -> Component {
    Box::new(move |frame| {
        frame.draw_str("Hello, World", Props::center());
    })
}
