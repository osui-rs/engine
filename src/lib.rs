pub mod elements;
pub mod utils;

pub use std::io::Result;
use std::sync::Arc;

pub mod prelude {
    pub use crate::{utils::init, Props, Result, Ui};
}

// Traits

pub trait Framing {
    fn draw(&mut self, element: &dyn Element, props: &Props);
}

pub trait Element {
    fn render(&self) -> String;
}

// Types
pub type Ui = Arc<dyn Fn(&mut dyn Framing)>;

pub enum Position {
    Num(u16),
    Center,
}

pub struct Props(Position, Position);

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console {
    mouse: bool,
    width: u16,
    height: u16,
}

impl Position {
    pub fn get(&self, written: u16, frame: u16) -> u16 {
        match self {
            Self::Center => (frame - written) / 2,
            Self::Num(n) => *n,
        }
    }
}

impl Props {
    pub fn new(x: u16, y: u16) -> Self {
        Self(Position::Num(x), Position::Num(y))
    }

    pub fn center() -> Self {
        Self(Position::Center, Position::Center)
    }

    pub fn center_horizontal() -> Self {
        Self(Position::Center, Position::Num(0))
    }

    pub fn center_vertical() -> Self {
        Self(Position::Num(0), Position::Center)
    }
}

impl Framing for Console {
    fn draw(&mut self, element: &dyn Element, props: &Props) {
        let written = element.render();

        let (written_width, written_height) = utils::str_size(&written);

        let (x, y) = (
            props.0.get(written_width, self.width),
            props.1.get(written_height, self.height),
        );

        for (i, line) in written.lines().enumerate() {
            println!("\x1b[{};{}H{}", y as usize + i + 1, x + 1, line);
        }
    }
}

impl Console {
    pub fn render(&mut self, ui: Ui) -> Result<()> {
        utils::clear()?;
        (self.width, self.height) = crossterm::terminal::size()?;
        ui(self);
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        if self.mouse {
            crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
        }
        crossterm::terminal::disable_raw_mode()?;
        utils::show_cursor()?;
        utils::clear()
    }
}
