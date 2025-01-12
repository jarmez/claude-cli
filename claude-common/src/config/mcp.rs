use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub url: String,
    pub api_version: String,
    pub tools: Vec<McpTool>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, ToolParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub type_name: String,
    pub description: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Default)]
pub struct McpConfig {
    pub servers: Vec<McpServer>,
    config_path: PathBuf,
}

impl McpConfig {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("claude-cli");
        
        let config_path = config_dir.join("mcp_servers.json");
        
        if !config_path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&config_path)?;
        let servers = serde_json::from_str(&content)?;
        
        Ok(Self {
            servers,
            config_path,
        })
    }
    
    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(&self.servers)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
    
    pub fn add_server(&mut self, server: McpServer) -> Result<()> {
        // Check if server with same name exists
        if let Some(existing) = self.servers.iter_mut()
            .find(|s| s.name == server.name) 
        {
            *existing = server;
        } else {
            self.servers.push(server);
        }
        self.save()?;
        Ok(())
    }
    
    pub fn remove_server(&mut self, name: &str) -> Result<()> {
        self.servers.retain(|s| s.name != name);
        self.save()?;
        Ok(())
    }
    
    pub fn get_enabled_servers(&self) -> Vec<&McpServer> {
        self.servers.iter()
            .filter(|s| s.enabled)
            .collect()
    }
}