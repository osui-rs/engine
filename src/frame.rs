use crossterm::event::Event;

use crate::style;

// Structs
#[derive(Debug, Clone)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub struct Frame {
    pub area: Area,
    pub event: Option<Event>,
}

impl Frame {
    pub fn new(area: Area) -> Frame {
        Frame { area, event: None }
    }

    pub fn draw(&self, c: &mut crate::Component, props: Props) {
        let (width, height) = (
            props.width.get(0, self.area.width),
            props.height.get(0, self.area.height),
        );

        let (x, y) = (
            props.x.get(0, self.area.width),
            props.y.get(0, self.area.height),
        );

        let mut frame = Frame::new(Area {
            x,
            y,
            width,
            height,
        });

        c(&mut frame)
    }

    pub fn draw_str(&self, s: &str, props: Props) {
        let (written_width, written_height) = crate::utils::str_size(s);

        let (width, height) = (
            props.width.get_root(written_width, self.area.width),
            props.height.get_root(written_height, self.area.height),
        );

        let (x, y) = (
            props.x.get(width, self.area.width),
            props.y.get(height, self.area.height),
        );

        for (i, line) in s.lines().enumerate() {
            println!(
                "\x1b[{};{}H{}\x1b[0m",
                (y + self.area.y + 1) as usize + i,
                x + self.area.x + 1,
                line,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Props {
    x: style::Position,
    y: style::Position,
    width: style::Dimension,
    height: style::Dimension,
}

impl Props {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x: style::Position::Num(x),
            y: style::Position::Num(y),
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
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
        }
    }

    pub fn center_horizontal() -> Self {
        Self {
            x: style::Position::Center,
            y: style::Position::Num(0),
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
        }
    }

    pub fn center_vertical() -> Self {
        Self {
            x: style::Position::Num(0),
            y: style::Position::Center,
            width: style::Dimension::Auto,
            height: style::Dimension::Auto,
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
