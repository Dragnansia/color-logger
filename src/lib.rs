//! Simple logger
//! Just change the colors of logs
//!
//! Can change color for every Level
//!
//! # Example
//! ```
//! fn main() -> Result<(), log::SetLoggerError> {
//!     color_logger::init()?;
//!     log::info!("Info");
//!
//!     Ok(())
//! }
//! ```

pub mod color;

use color::Color;
use log::*;

struct ColorLogger {
    color: [(log::Level, Color); 5],
}

impl ColorLogger {
    pub const fn new() -> Self {
        Self {
            color: [
                (Level::Error, Color::rgb([255, 0, 0])),
                (Level::Warn, Color::rgb([255, 255, 0])),
                (Level::Info, Color::Default),
                (Level::Debug, Color::Default),
                (Level::Trace, Color::Default),
            ],
        }
    }

    pub fn change_color(&mut self, level: Level, color: Color) {
        for i in 0..5 {
            if self.color[i].0 == level {
                self.color[i].1 = color;
            }
        }
    }

    pub fn find_color(&self, level: Level) -> Color {
        for (lvl, cl) in self.color {
            if lvl == level {
                return cl;
            }
        }

        Color::Default
    }
}

impl Log for ColorLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = self.find_color(record.level());

        println!("{}{}\x1b[0m", color.terminal_format(), record.args());
    }

    fn flush(&self) {}
}

static mut LOGGER: ColorLogger = ColorLogger::new();

/// Initialise ColorLogger
pub fn init() -> Result<(), SetLoggerError> {
    unsafe {
        set_logger(&LOGGER)?;
    }

    set_max_level(LevelFilter::Info);

    Ok(())
}

/// Change color logger for a specifique Level
///
/// # Example
/// ```
/// use log::*;
/// use color_logger::{*, color::Color};
///
/// fn main() -> Result<(), SetLoggerError> {
///     init()?;
///     set_level_color(Level::Info, Color::rgb([255, 255, 255]));
///     
///     Ok(())
/// }
/// ```
pub fn set_level_color(level: Level, color: Color) {
    unsafe {
        LOGGER.change_color(level, color);
    }
}

/// Return the current color for the Level
pub fn get_level_color(level: Level) -> Color {
    unsafe { LOGGER.find_color(level) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        init().unwrap_or_default();

        info!("Info");
        error!("Error");
        warn!("Warn");
        trace!("Trace");
    }

    #[test]
    fn change_color() {
        init().unwrap_or_default();

        let color = Color::rgb([255, 0, 255]);
        set_level_color(Level::Info, color);

        let info_color = get_level_color(Level::Info);
        assert_eq!(color, info_color);

        set_level_color(Level::Info, Color::Default);
    }
}
