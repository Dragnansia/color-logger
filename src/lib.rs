//! Simple logger
//! Just change the colors of logs
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

struct ColorLogger;
impl log::Log for ColorLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            log::Level::Error => "\x1b[31m",
            log::Level::Warn => "\x1b[93m",
            _ => "\x1b[0m",
        };

        println!("{}{}\x1b[0m", color, record.args());
    }

    fn flush(&self) {}
}

static LOGGER: ColorLogger = ColorLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}

#[test]
fn test_display() {
    init().unwrap();

    log::info!("Info");
    log::error!("Error");
    log::warn!("Warn");
    log::trace!("Trace");
}
