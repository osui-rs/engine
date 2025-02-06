pub mod frame;
mod props;
#[cfg(feature = "state")]
pub mod state;
pub mod utils;

use crossterm::event::Event;
use frame::Frame;
pub use props::*;
pub use std::io::Result;

pub mod prelude {
    pub use crate::{
        frame::Frame,
        props::{Color, Style},
        utils::init,
        Component, Element, Props, Result,
    };

    #[cfg(feature = "state")]
    pub use crate::state::{default_state, use_state, State};

    pub use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
}

pub trait Element {
    fn render(&self, size: (u16, u16)) -> String;
    fn event(&mut self, _event: crossterm::event::Event) {}
    fn action(&mut self, _action: u16) {}
}

pub type Component = Box<dyn FnMut(&Frame)>;

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console {
    mouse: bool,
}

impl Console {
    pub fn render(&mut self, ui: &mut Component) -> Result<()> {
        utils::clear()?;
        let mut frame = Frame::new(crossterm::terminal::size()?);
        ui(&mut frame);
        Ok(())
    }

    pub fn event(&mut self, ui: &mut Component, event: Event) -> Result<()> {
        utils::clear()?;
        let mut frame = Frame::new(crossterm::terminal::size()?);
        frame.event = Some(event);
        ui(&mut frame);
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

impl Element for &str {
    fn render(&self, _: (u16, u16)) -> String {
        self.to_string()
    }
}

impl Element for String {
    fn render(&self, _: (u16, u16)) -> String {
        self.to_owned()
    }
}
