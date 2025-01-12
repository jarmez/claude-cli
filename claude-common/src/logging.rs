use anyhow::Result;
use chrono::Local;
use syslog::{Facility, Formatter3164};
use std::fs::{self, File};
use std::path::Path;
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::FormatTime},
    EnvFilter,
};

use crate::types::LogLevel;

struct CustomTime;

impl FormatTime for CustomTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn setup_logging(log_dir: &Path, level: LogLevel) -> Result<()> {
    // Create log directory if it doesn't exist
    fs::create_dir_all(log_dir)?;
    
    let log_file = log_dir.join(format!(
        "claude-{}.log",
        Local::now().format("%Y-%m-%d")
    ));
    
    // Create the file appender
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("claude")
            .filename_suffix("log")
            .max_files(7)
            .build(log_dir)?
    );
    
    // Set up syslog formatter
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "claude-cli".into(),
        pid: std::process::id(),
    };
    
    // Create multi-writer for both file and stdout
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Set up subscriber with formatting
    tracing_subscriber::fmt()
        .with_timer(CustomTime)
        .with_thread_ids(true)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(level_to_directive(level)))
        .with_writer(non_blocking)
        .init();
    
    Ok(())
}

fn level_to_directive(level: LogLevel) -> tracing::Level {
    match level {
        LogLevel::Emergency | LogLevel::Alert | LogLevel::Critical => tracing::Level::ERROR,
        LogLevel::Error => tracing::Level::ERROR,
        LogLevel::Warning => tracing::Level::WARN,
        LogLevel::Notice | LogLevel::Info => tracing::Level::INFO,
        LogLevel::Debug => tracing::Level::DEBUG,
    }
}

pub fn log_to_syslog(message: &str, level: LogLevel) -> Result<()> {
    use syslog::Error::*;
    match level {
        LogLevel::Emergency => syslog::unix(Facility::LOG_USER)?.err(message),
        LogLevel::Alert => syslog::unix(Facility::LOG_USER)?.alert(message),
        LogLevel::Critical => syslog::unix(Facility::LOG_USER)?.crit(message),
        LogLevel::Error => syslog::unix(Facility::LOG_USER)?.err(message),
        LogLevel::Warning => syslog::unix(Facility::LOG_USER)?.warning(message),
        LogLevel::Notice => syslog::unix(Facility::LOG_USER)?.notice(message),
        LogLevel::Info => syslog::unix(Facility::LOG_USER)?.info(message),
        LogLevel::Debug => syslog::unix(Facility::LOG_USER)?.debug(message),
    }
    Ok(())
}