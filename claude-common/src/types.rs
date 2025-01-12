use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub model: String,
    pub messages: Vec<Message>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub default_model: String,
    pub output_format: OutputFormat,
    pub log_level: LogLevel,
    pub config_dir: PathBuf,
    pub history_file: PathBuf,
    pub log_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("claude-cli");

        Self {
            api_key: std::env::var("CLAUDE_API_KEY").unwrap_or_default(),
            default_model: std::env::var("CLAUDE_MODEL")
                .unwrap_or_else(|_| String::from("claude-3-sonnet")),
            output_format: OutputFormat::Text,
            log_level: LogLevel::Info,
            config_dir: config_dir.clone(),
            history_file: config_dir.join("history.json"),
            log_dir: config_dir.join("logs"),
        }
    }
}