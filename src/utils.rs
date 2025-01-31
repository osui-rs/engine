//! The `utils` module provides utility functions for terminal manipulation
//! and string operations, designed to enhance terminal-based applications.

use std::io::Write;

/// Clears the terminal screen and moves the cursor to the top-left corner.
///
/// # Returns
/// A `crate::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::clear().unwrap();
/// ```
pub fn clear() -> crate::Result<()> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()
}

/// Hides the terminal cursor.
///
/// # Returns
/// A `crate::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::hide_cursor().unwrap();
/// ```
pub fn hide_cursor() -> crate::Result<()> {
    print!("\x1b[?25l");
    std::io::stdout().flush()
}

/// Shows the terminal cursor.
///
/// # Returns
/// A `crate::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::show_cursor().unwrap();
/// ```
pub fn show_cursor() -> crate::Result<()> {
    print!("\x1B[?25h");
    std::io::stdout().flush()
}

/// Flushes the terminal's stdout buffer.
///
/// # Returns
/// A `crate::Result<()>` indicating whether the operation succeeded.
///
/// # Example
/// ```
/// utils::flush().unwrap();
/// ```
pub fn flush() -> crate::Result<()> {
    std::io::stdout().flush()
}

/// Calculates the width and height of a string when rendered in a terminal.
///
/// # Arguments
/// - `s`: The input string.
///
/// # Returns
/// A tuple `(u16, u16)` where:
/// - The first value is the maximum width of the string in characters.
/// - The second value is the height of the string in lines.
///
/// # Example
/// ```
/// let (width, height) = utils::str_size("Hello\nWorld!");
/// assert_eq!((5, 2), (width, height));
/// ```
pub fn str_size(s: &str) -> (u16, u16) {
    let mut height = 1;
    let mut max_width = 0;
    let mut current_width = 0;

    for b in s.bytes() {
        if b == b'\n' {
            height += 1;
            max_width = max_width.max(current_width);
            current_width = 0;
        } else {
            current_width += 1;
        }
    }

    max_width = max_width.max(current_width);

    (max_width, height)
}

/// Initializes the console with raw mode enabled and optionally mouse capture.
///
/// # Arguments
/// * `mouse` - A boolean flag indicating whether to enable mouse capture.
///
/// # Returns
/// A `Console` instance wrapped in a `Result`.
pub fn init<'a>(mouse: bool) -> crate::Result<crate::Console> {
    crossterm::terminal::enable_raw_mode()?;
    clear()?;
    hide_cursor()?;
    if mouse {
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    }
    let (width, height) = crossterm::terminal::size()?;
    Ok(crate::Console {
        width,
        height,
        mouse,
        mouse_position: None,
    })
}
