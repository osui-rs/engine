pub mod frame;
#[cfg(feature = "state")]
pub mod state;
pub mod style;
pub mod utils;

use crossterm::event::Event;
use frame::{Area, Frame};
pub use std::io::Result;

pub mod prelude {
    pub use crate::{
        frame::{Frame, Props},
        func,
        style::{Color, Style},
        utils::init,
        Component, Result,
    };

    #[cfg(feature = "state")]
    pub use crate::state::{default_state, use_state, State};

    pub use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
}

// Types
pub type Handler<T> = std::sync::Arc<std::sync::Mutex<T>>;
pub type Component = Box<dyn FnMut(&Frame)>;

// Structs

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console {
    mouse: bool,
}

impl Console {
    pub fn render(&mut self, ui: &mut Component) -> Result<()> {
        let (width, height) = crossterm::terminal::size()?;
        utils::clear()?;
        let mut frame = Frame::new(Area {
            width,
            height,
            x: 0,
            y: 0,
        });
        ui(&mut frame);
        Ok(())
    }

    pub fn event(&mut self, ui: &mut Component, event: Event) -> Result<()> {
        let (width, height) = crossterm::terminal::size()?;
        utils::clear()?;
        let mut frame = Frame {
            area: Area {
                width,
                height,
                x: 0,
                y: 0,
            },
            event: Some(event),
        };
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

pub fn func<T>(t: T) -> Handler<T> {
    std::sync::Arc::new(std::sync::Mutex::new(t))
}
