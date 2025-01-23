pub mod elements;
pub mod style;
pub mod utils;

pub use std::io::Result;
use std::sync::Arc;

pub mod prelude {
    pub use crate::{
        style::{Color, Style},
        utils::init,
        Props, Result, Ui,
    };
}

// Traits
pub trait Framing {
    fn draw(&mut self, element: &dyn Element, props: &Props);
}

pub trait Element {
    fn render(&self) -> String;
    fn get_colors(&self) -> std::collections::HashMap<String, (style::Color, bool)> {
        std::collections::HashMap::new()
    }
}

// Types
pub type Ui = Arc<dyn Fn(&mut dyn Framing)>;

#[derive(Debug, Clone)]
pub struct Props {
    x: style::Position,
    y: style::Position,
    width: style::Dimension,
    height: style::Dimension,
    style: style::Style,
    style_state: std::collections::HashMap<String, style::Style>,
}

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console {
    mouse: bool,
    width: u16,
    height: u16,
    pub mouse_position: Option<(u16, u16)>,
}

impl Props {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x: style::Position::Num(x),
            y: style::Position::Num(y),
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
            style_state: std::collections::HashMap::new(),
        }
    }

    pub fn center() -> Self {
        Self {
            x: style::Position::Center,
            y: style::Position::Center,
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
            style_state: std::collections::HashMap::new(),
        }
    }

    pub fn center_horizontal() -> Self {
        Self {
            x: style::Position::Center,
            y: style::Position::Num(0),
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
            style_state: std::collections::HashMap::new(),
        }
    }

    pub fn center_vertical() -> Self {
        Self {
            x: style::Position::Num(0),
            y: style::Position::Center,
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
            style_state: std::collections::HashMap::new(),
        }
    }

    pub fn style(&mut self, style: style::Style) -> Self {
        self.style = style;
        self.clone()
    }

    pub fn state(&mut self, state: &str, style: style::Style) -> Self {
        self.style_state.insert(state.to_string(), style);
        self.clone()
    }
}

impl Framing for Console {
    fn draw(&mut self, element: &dyn Element, props: &Props) {
        let written = element.render();

        let (written_width, written_height) = utils::str_size(&written);

        let (width, height) = (
            props.width.get_root(written_width, self.width),
            props.height.get_root(written_height, self.height),
        );

        let (x, y) = (
            props.x.get(width, self.width),
            props.y.get(height, self.height),
        );

        let mut style = props.style;

        if let Some((mouse_x, mouse_y)) = self.mouse_position {
            if (x..x + width).contains(&mouse_x) && (y..y + height).contains(&mouse_y) {
                if let Some(hover_style) = props.style_state.get("hover") {
                    style.color = hover_style.color;
                }
            }
        }

        for (i, line) in written.lines().enumerate() {
            println!(
                "\x1b[{};{}H{}{}",
                y as usize + i + 1,
                x + 1,
                style.color.to_ansi(true),
                line
            );
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
