use anyhow::Result;
use mlua::Lua;
use std::path::Path;
use crate::types::{Config, OutputFormat, LogLevel};

pub fn load_lua_config(path: &Path) -> Result<Option<Config>> {
    if !path.exists() {
        return Ok(None);
    }

    let lua = Lua::new();
    let chunk = std::fs::read_to_string(path)?;
    
    // Execute the Lua code
    lua.load(&chunk).exec()?;
    
    // Get the claude_config table
    let globals = lua.globals();
    let config_table: mlua::Table = globals.get("claude_config")?;
    
    let config = Config {
        api_key: config_table.get("api_key")
            .unwrap_or_else(|_| std::env::var("CLAUDE_API_KEY").unwrap_or_default()),
            
        default_model: config_table.get("default_model")
            .unwrap_or_else(|_| String::from("claude-3-sonnet")),
            
        output_format: parse_output_format(config_table.get("output_format")?)?,
        log_level: parse_log_level(config_table.get("log_level")?)?,
        
        config_dir: dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("claude-cli"),
            
        history_file: config_table.get("history_file")
            .unwrap_or_else(|_| Ok(dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
                .join("claude-cli/history.json")))?,
                
        log_dir: config_table.get("log_dir")
            .unwrap_or_else(|_| Ok(dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
                .join("claude-cli/logs")))?,
    };
    
    Ok(Some(config))
}

fn parse_output_format(value: mlua::Value) -> Result<OutputFormat> {
    match value {
        mlua::Value::String(s) => {
            match s.to_str()? {
                "text" => Ok(OutputFormat::Text),
                "json" => Ok(OutputFormat::Json),
                "csv" => Ok(OutputFormat::Csv),
                "markdown" => Ok(OutputFormat::Markdown),
                _ => Ok(OutputFormat::Text),
            }
        }
        _ => Ok(OutputFormat::Text),
    }
}

fn parse_log_level(value: mlua::Value) -> Result<LogLevel> {
    match value {
        mlua::Value::String(s) => {
            match s.to_str()? {
                "emergency" => Ok(LogLevel::Emergency),
                "alert" => Ok(LogLevel::Alert),
                "critical" => Ok(LogLevel::Critical),
                "error" => Ok(LogLevel::Error),
                "warning" => Ok(LogLevel::Warning),
                "notice" => Ok(LogLevel::Notice),
                "info" => Ok(LogLevel::Info),
                "debug" => Ok(LogLevel::Debug),
                _ => Ok(LogLevel::Info),
            }
        }
        _ => Ok(LogLevel::Info),
    }
}