//! Simple logger
//! Just change the colors of logs
//!
//! Can change color for every Level
//!
//! # Example
//! ```
//! use color_logger::ColorLogger;
//!
//! fn main() -> Result<(), log::SetLoggerError> {
//!     ColorLogger::new();
//!     log::info!("Info");
//!
//!     Ok(())
//! }
//! ```

pub mod color;

use color::Color;
use log::*;

static ONCE: std::sync::Once = std::sync::Once::new();

pub struct ColorLogger {
    color: [(log::Level, Color); 5],
    pub level: Level,
}

impl ColorLogger {
    /// Initialise ColorLogger
    ///
    /// The default value of level is `Level::Info`
    pub fn new() -> &'static mut ColorLogger {
        static mut LOGGER: ColorLogger = ColorLogger {
            color: [
                (Level::Error, Color::rgb([255, 0, 0])),
                (Level::Warn, Color::rgb([255, 255, 0])),
                (Level::Info, Color::Default),
                (Level::Debug, Color::Default),
                (Level::Trace, Color::Default),
            ],
            level: Level::Info,
        };

        ONCE.call_once(|| unsafe {
            set_logger(&LOGGER).unwrap();
        });

        set_max_level(LevelFilter::Info);

        unsafe { &mut LOGGER }
    }

    /// Change color logger for a specifique Level
    ///
    /// # Example
    /// ```
    /// use log::*;
    /// use color_logger::{*, color::Color};
    ///
    /// fn main() -> Result<(), SetLoggerError> {
    ///     ColorLogger::new().change_color(
    ///        Level::Info,
    ///        Color::rgb([255, 255, 255])
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn change_color(&'static mut self, level: Level, color: Color) -> &'static mut ColorLogger {
        self.color
            .iter_mut()
            .filter(|c| c.0 == level)
            .for_each(|c| c.1 = color);

        self
    }

    /// Return color for specific Level
    pub fn find_color(&self, level: Level) -> Color {
        self.color
            .iter()
            .find_map(|c| match c.0 == level {
                true => Some(c.1),
                false => None,
            })
            .unwrap_or(Color::Default)
    }

    /// Change level for logger message
    ///
    /// # Example
    /// ```
    /// use log::*;
    /// use color_logger::{*, color::Color};
    ///
    /// fn main() -> Result<(), SetLoggerError> {
    ///    ColorLogger::new().set_level(Level::Error);
    ///    Ok(())
    /// }
    /// ```
    pub fn set_level(&'static mut self, level: Level) -> &'static mut ColorLogger {
        self.level = level;
        self
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn change_color() {
        let color = Color::rgb([255, 0, 255]);
        let logger = ColorLogger::new().change_color(Level::Info, color);

        let info_color = logger.find_color(Level::Info);
        assert_eq!(color, info_color);

        logger.change_color(Level::Info, Color::Default);
    }

    #[test]
    fn change_level() {
        let logger = ColorLogger::new().set_level(Level::Debug);

        assert_eq!(Level::Debug, logger.level);
    }
}
