use log::{LevelFilter, Metadata, Record, SetLoggerError};
use std::time::{SystemTime, UNIX_EPOCH};

struct Logger;

impl log::Log for Logger {
    // logger has to implement this trait
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Validation is typically handled by the global max_level,
        // but this allows for custom filtering logic if needed.
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Calculate a simple HH:MM:SS timestamp using only std
            let duration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let secs = duration.as_secs();
            let (h, m, s) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);

            // Print with fixed-width alignment for better scannability
            println!(
                "{:02}:{:02}:{:02} {:<5} [{}] {}",
                h,
                m,
                s,
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

/// Initializes the logger. You can now pass the desired LevelFilter.
pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}
