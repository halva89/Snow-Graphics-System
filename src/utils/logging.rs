use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }
}

pub struct Logger {
    pub enabled: bool,
    pub min_level: LogLevel,
    pub start_time: Instant,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            enabled: true,
            min_level: LogLevel::Info,
            start_time: Instant::now(),
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if !self.enabled {
            return;
        }

        if (level as u8) < (self.min_level as u8) {
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let timestamp = format!("{:.3}", elapsed);
        let level_str = level.as_str();

        match level {
            LogLevel::Error => eprintln!("[{}][{}] {}", timestamp, level_str, message),
            _ => println!("[{}][{}] {}", timestamp, level_str, message),
        }
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warning, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn trace(&self, msg: &str) {
        self.log(LogLevel::Trace, msg);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}