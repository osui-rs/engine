pub mod elements;
#[cfg(feature = "state")]
pub mod state;
pub mod style;
pub mod utils;

pub use std::io::Result;
use std::{io::Write, sync::Arc};

pub mod prelude {
    pub use crate::{
        func,
        style::{Color, Style},
        utils::init,
        Props, Result, Ui,
    };

    #[cfg(feature = "state")]
    pub use crate::state::use_state;

    pub use crossterm::event::read;
}

// Traits
pub trait Framing {
    fn draw(&mut self, element: &mut dyn Element, props: &Props);
    fn draw_str(&mut self, s: &str) {
        _ = s
    }
}

pub trait Element {
    fn render(&self, frame: &mut dyn Framing);
    fn event(&self, event: crossterm::event::Event) {
        _ = event
    }
}

// Types
pub type Handler<T> = std::sync::Arc<std::sync::Mutex<T>>;
pub type Ui = Arc<dyn Fn(&mut dyn Framing)>;

#[derive(Debug, Clone)]
pub struct Props {
    x: style::Position,
    y: style::Position,
    width: style::Dimension,
    height: style::Dimension,
    style: style::Style,
}

/// Represents the console state, containing a frame for rendering and a mouse capture flag.
pub struct Console<'a> {
    mouse: bool,
    width: u16,
    height: u16,
    handle: Option<std::io::StdoutLock<'a>>,
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
        }
    }

    pub fn auto() -> Self {
        Self::new(0, 0)
    }

    pub fn center() -> Self {
        Self {
            x: style::Position::Center,
            y: style::Position::Center,
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
        }
    }

    pub fn center_horizontal() -> Self {
        Self {
            x: style::Position::Center,
            y: style::Position::Num(0),
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
        }
    }

    pub fn center_vertical() -> Self {
        Self {
            x: style::Position::Num(0),
            y: style::Position::Center,
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
            style: style::Style::default(),
        }
    }

    pub fn x_percent(&mut self, p: u16) -> Self {
        self.x = style::Position::Percent(p);
        self.clone()
    }

    pub fn y_percent(&mut self, p: u16) -> Self {
        self.y = style::Position::Percent(p);
        self.clone()
    }

    pub fn style(&mut self, style: style::Style) -> Self {
        self.style = style;
        self.clone()
    }

    pub fn width(&mut self, w: u16) -> Self {
        self.width = style::Dimension::Num(w);
        self.clone()
    }

    pub fn height(&mut self, h: u16) -> Self {
        self.height = style::Dimension::Num(h);
        self.clone()
    }

    pub fn width_percent(&mut self, w: u16) -> Self {
        self.width = style::Dimension::Percent(w);
        self.clone()
    }

    pub fn height_percent(&mut self, h: u16) -> Self {
        self.height = style::Dimension::Percent(h);
        self.clone()
    }
}

impl Framing for Console<'_> {
    fn draw(&mut self, element: &mut dyn Element, props: &Props) {
        let mut written = String::new();
        element.render(&mut written);

        let (written_width, written_height) = utils::str_size(&written);

        let (width, height) = (
            props.width.get_root(written_width, self.width),
            props.height.get_root(written_height, self.height),
        );

        let (x, y) = (
            props.x.get(width, self.width),
            props.y.get(height, self.height),
        );

        let ansi = if props.style.background == style::Color::NoColor {
            format!("\x1b[{}m", props.style.color.ansi_number(true))
        } else {
            format!(
                "\x1b[{};{}m",
                props.style.color.ansi_number(true),
                props.style.background.ansi_number(false)
            )
        };

        let (x, y) = (x - props.style.px, y - props.style.py);
        let px = " ".repeat(props.style.px as usize);
        written
            .push_str(&format!("\n{}", " ".repeat(width as usize)).repeat(props.style.py as usize));
        written.insert_str(
            0,
            &format!("{}\n", " ".repeat(width as usize)).repeat(props.style.py as usize),
        );

        if let Some(handle) = &mut self.handle {
            for (i, line) in written.lines().enumerate() {
                write!(
                    handle,
                    "\x1b[{};{}H{ansi}{px}{}{px}\x1b[0m",
                    y as usize + i + 1,
                    x + 1,
                    line,
                )
                .unwrap();
            }
        } else {
            panic!("Handle not found");
        }
    }
}

impl Console<'_> {
    pub fn render(&mut self, ui: Ui) -> Result<()> {
        (self.width, self.height) = crossterm::terminal::size()?;
        let stdout = std::io::stdout();
        self.handle = Some(stdout.lock());
        utils::clear()?;
        ui(self);
        if let Some(handle) = &mut self.handle {
            handle.flush()?;
        }
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

impl crate::Framing for String {
    fn draw(&mut self, element: &mut dyn crate::Element, _props: &crate::Props) {
        element.render(self);
    }

    fn draw_str(&mut self, s: &str) {
        *self += s;
    }
}

pub fn func<T>(t: T) -> Handler<T> {
    std::sync::Arc::new(std::sync::Mutex::new(t))
}
