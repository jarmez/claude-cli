use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub default_model: String,
    pub output_format: OutputFormat,
    pub config_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Markdown,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: std::env::var("CLAUDE_API_KEY").unwrap_or_default(),
            default_model: std::env::var("CLAUDE_MODEL")
                .unwrap_or_else(|_| String::from("claude-3-sonnet")),
            output_format: OutputFormat::Text,
            config_dir: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("claude-cli"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("claude-cli/config.json");

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = self.config_dir.join("config.json");

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }
}