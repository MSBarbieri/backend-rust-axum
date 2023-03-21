use clap::ValueEnum;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, ValueEnum)]
pub enum LogLevel {
    #[default]
    DEBUG,
    TRACE,
    INFO,
    WARN,
    ERROR,
}

impl Into<tracing::Level> for LogLevel {
    fn into(self) -> tracing::Level {
        match self {
            LogLevel::TRACE => tracing::Level::TRACE,
            LogLevel::DEBUG => tracing::Level::DEBUG,
            LogLevel::INFO => tracing::Level::INFO,
            LogLevel::WARN => tracing::Level::WARN,
            LogLevel::ERROR => tracing::Level::ERROR,
        }
    }
}
