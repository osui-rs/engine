#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    NoColor,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8), // Custom RGB color
}

impl Color {
    /// Converts the enum to its ANSI escape code
    pub fn ansi_number(&self, fg: bool) -> String {
        let f = if fg { 30 } else { 40 };

        if *self == Self::NoColor {
            return String::new();
        } else if let Self::Rgb(r, g, b) = self {
            return format!("{};2;{r};{g};{b}", f + 8);
        }

        (f + match self {
            Self::Rgb(_, _, _) => 0,
            Self::NoColor => 0,

            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::BrightBlack => 60,
            Self::BrightRed => 61,
            Self::BrightGreen => 62,
            Self::BrightYellow => 63,
            Self::BrightBlue => 64,
            Self::BrightMagenta => 65,
            Self::BrightCyan => 66,
            Self::BrightWhite => 67,
        })
        .to_string()
    }

    pub fn to_ansi(&self, foreground: bool) -> String {
        let f = if foreground { "3" } else { "4" };
        match self {
            Self::NoColor => String::new(),
            Self::Black => format!("\x1b[{f}0m"),
            Self::Red => format!("\x1b[{f}1m"),
            Self::Green => format!("\x1b[{f}2m"),
            Self::Yellow => format!("\x1b[{f}3m"),
            Self::Blue => format!("\x1b[{f}4m"),
            Self::Magenta => format!("\x1b[{f}5m"),
            Self::Cyan => format!("\x1b[{f}6m"),
            Self::White => format!("\x1b[{f}7m"),
            Self::BrightBlack => format!("\x1b[{f}90m"),
            Self::BrightRed => format!("\x1b[{f}91m"),
            Self::BrightGreen => format!("\x1b[{f}92m"),
            Self::BrightYellow => format!("\x1b[{f}93m"),
            Self::BrightBlue => format!("\x1b[{f}94m"),
            Self::BrightMagenta => format!("\x1b[{f}95m"),
            Self::BrightCyan => format!("\x1b[{f}96m"),
            Self::BrightWhite => format!("\x1b[{f}97m"),
            Self::Rgb(r, g, b) => {
                let code = if foreground { "38" } else { "48" };
                format!("\x1b[{code};2;{r};{g};{b}m")
            }
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::NoColor
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    pub color: Color,
    pub background: Color,
    pub px: u16,
    pub py: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Num(u16),
    Percent(u16),
    Center,
}
impl Position {
    pub fn get(&self, written: u16, frame: u16) -> u16 {
        match self {
            Self::Center => (frame - written) / 2,
            Self::Percent(percent) => (((*percent as f32) / 100.0) * (frame as f32)) as u16,
            Self::Num(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Num(u16),
    Percent(u16),
    Auto,
}
impl Dimension {
    pub fn get(&self, _written: u16, frame: u16) -> u16 {
        match self {
            Self::Auto => frame,
            Self::Percent(percent) => (((*percent as f32) / 100.0) * (frame as f32)) as u16,
            Self::Num(n) => *n,
        }
    }

    pub fn get_root(&self, written: u16, frame: u16) -> u16 {
        match self {
            Self::Auto => written,
            Self::Percent(percent) => (((*percent as f32) / 100.0) * (frame as f32)) as u16,
            Self::Num(n) => *n,
        }
    }
}
