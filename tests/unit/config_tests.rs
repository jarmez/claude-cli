use claude_common::config::Config;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_config_default_values() {
    let config = Config::default();
    assert_eq!(config.default_model, "claude-3-sonnet");
    assert!(config.config_dir.ends_with("claude-cli"));
}

#[test]
fn test_config_save_load() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.json");
    
    let mut config = Config::default();
    config.api_key = "test-key".to_string();
    config.save(&config_path)?;
    
    let loaded = Config::load(&config_path)?;
    assert_eq!(loaded.api_key, "test-key");
    
    Ok(())
}

#[test]
fn test_config_environment_override() {
    std::env::set_var("CLAUDE_API_KEY", "env-key");
    std::env::set_var("CLAUDE_MODEL", "claude-3-opus");
    
    let config = Config::default();
    assert_eq!(config.api_key, "env-key");
    assert_eq!(config.default_model, "claude-3-opus");
    
    std::env::remove_var("CLAUDE_API_KEY");
    std::env::remove_var("CLAUDE_MODEL");
}

#[test]
fn test_config_lua_loading() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.lua");
    
    fs::write(&config_path, r#"
    claude_config = {
        api_key = "lua-key",
        default_model = "claude-3-opus",
        output_format = "json"
    }
    "#)?;
    
    let config = Config::load_from_lua(&config_path)?;
    assert_eq!(config.api_key, "lua-key");
    assert_eq!(config.default_model, "claude-3-opus");
    
    Ok(())
}