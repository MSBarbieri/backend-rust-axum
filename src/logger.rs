use clap::ValueEnum;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, ValueEnum)]

pub enum LogLevel {
    #[default]
    Debug,
    Trace,
    Info,
    Wawn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Wawn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}
