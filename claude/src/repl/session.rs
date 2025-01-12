use anyhow::Result;
use claude_common::{Config, ClaudeClient, types::{Session, Message}};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use chrono::Utc;

#[derive(Debug)]
enum Mode {
    Chat,
    Command,
}

#[derive(Debug)]
enum Command {
    Quit,
    Help,
    List(Option<String>),
    Save(String),
    Load(String),
    Model(String),
    Clear,
    Unknown(String),
}

pub struct ReplSession {
    mode: Mode,
    client: ClaudeClient,
    config: Config,
    command_buffer: String,
    input_buffer: String,
    history: Vec<Message>,
    current_model: String,
}

impl ReplSession {
    pub fn new(client: ClaudeClient, config: Config) -> Self {
        Self {
            mode: Mode::Chat,
            client,
            config: config.clone(),
            command_buffer: String::new(),
            input_buffer: String::new(),
            history: Vec::new(),
            current_model: config.default_model,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("Claude CLI (Press <Esc> and type :help for commands, :q to quit)\n");
        enable_raw_mode()?;
        
        self.show_prompt();
        
        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match self.mode {
                    Mode::Chat => self.handle_chat_input(code).await?,
                    Mode::Command => {
                        if self.handle_command_input(code).await? {
                            break;
                        }
                    }
                }
            }
        }
        
        disable_raw_mode()?;
        println!("\nGoodbye!");
        Ok(())
    }

    async fn handle_chat_input(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Esc => {
                self.mode = Mode::Command;
                println!();
                print!(":");
                io::stdout().flush()?;
            }
            KeyCode::Enter => {
                if !self.input_buffer.trim().is_empty() {
                    println!();
                    let response = self.client.chat(&self.input_buffer, &self.current_model).await?;
                    
                    self.history.push(Message {
                        role: "user".to_string(),
                        content: self.input_buffer.clone(),
                        timestamp: Utc::now(),
                    });
                    
                    self.history.push(Message {
                        role: "assistant".to_string(),
                        content: response.clone(),
                        timestamp: Utc::now(),
                    });
                    
                    println!("\n{}\n", response);
                    self.input_buffer.clear();
                    self.show_prompt();
                }
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
                print!("{}", c);
                io::stdout().flush()?;
            }
            KeyCode::Backspace => {
                if !self.input_buffer.is_empty() {
                    self.input_buffer.pop();
                    print!("\x08 \x08");
                    io::stdout().flush()?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_command_input(&mut self, code: KeyCode) -> Result<bool> {
        match code {
            KeyCode::Enter => {
                println!();
                let should_quit = self.execute_command().await?;
                if !should_quit {
                    self.mode = Mode::Chat;
                    self.command_buffer.clear();
                    self.show_prompt();
                }
                Ok(should_quit)
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
                print!("{}", c);
                io::stdout().flush()?;
                Ok(false)
            }
            KeyCode::Backspace => {
                if !self.command_buffer.is_empty() {
                    self.command_buffer.pop();
                    print!("\x08 \x08");
                    io::stdout().flush()?;
                }
                Ok(false)
            }
            KeyCode::Esc => {
                self.mode = Mode::Chat;
                self.command_buffer.clear();
                println!();
                self.show_prompt();
                Ok(false)
            }
            _ => Ok(false)
        }
    }

    async fn execute_command(&mut self) -> Result<bool> {
        match self.parse_command() {
            Command::Quit => Ok(true),
            Command::Help => {
                self.show_help();
                Ok(false)
            }
            Command::List(filter) => {
                self.show_history(filter);
                Ok(false)
            }
            Command::Save(name) => {
                self.save_session(&name)?;
                Ok(false)
            }
            Command::Load(name) => {
                self.load_session(&name)?;
                Ok(false)
            }
            Command::Model(name) => {
                self.current_model = name;
                println!("Switched to model: {}", self.current_model);
                Ok(false)
            }
            Command::Clear => {
                self.history.clear();
                println!("History cleared");
                Ok(false)
            }
            Command::Unknown(cmd) => {
                println!("Unknown command: {}", cmd);
                Ok(false)
            }
        }
    }

    fn parse_command(&self) -> Command {
        let cmd = self.command_buffer.trim();
        if cmd.is_empty() {
            return Command::Unknown(String::new());
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts[0] {
            ":q" | ":quit" => Command::Quit,
            ":help" => Command::Help,
            ":list" => Command::List(parts.get(1).map(|s| s.to_string())),
            ":save" => {
                if parts.len() > 1 {
                    Command::Save(parts[1].to_string())
                } else {
                    Command::Unknown(":save requires a name".to_string())
                }
            }
            ":load" => {
                if parts.len() > 1 {
                    Command::Load(parts[1].to_string())
                } else {
                    Command::Unknown(":load requires a name".to_string())
                }
            }
            ":model" => {
                if parts.len() > 1 {
                    Command::Model(parts[1].to_string())
                } else {
                    Command::Unknown(":model requires a model name".to_string())
                }
            }
            ":clear" => Command::Clear,
            _ => Command::Unknown(cmd.to_string()),
        }
    }

    fn show_help(&self) {
        println!("\nAvailable Commands:");
        println!("  :help            Show this help message");
        println!("  :q, :quit        Exit the session");
        println!("  :list [filter]   List chat history");
        println!("  :save <name>     Save current session");
        println!("  :load <name>     Load a saved session");
        println!("  :model <name>    Switch Claude model");
        println!("  :clear           Clear current session");
        println!("\nIn chat mode:");
        println!("  <Esc>            Enter command mode");
        println!("  <Enter>          Send message");
    }

    fn show_history(&self, filter: Option<String>) {
        if self.history.is_empty() {
            println!("\nNo messages in current session");
            return;
        }

        println!("\nChat History:");
        for (i, msg) in self.history.iter().enumerate() {
            if let Some(ref f) = filter {
                if !msg.content.contains(f) {
                    continue;
                }
            }
            println!("\n[{}] {}: {}", i + 1, msg.role, msg.content);
        }
    }

    fn show_prompt(&self) {
        match self.mode {
            Mode::Chat => print!("chat> "),
            Mode::Command => print!(":"),
        }
        io::stdout().flush().unwrap();
    }

    fn save_session(&self, name: &str) -> Result<()> {
        let session = Session {
            id: name.to_string(),
            model: self.current_model.clone(),
            messages: self.history.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let session_dir = self.config.config_dir.join("sessions");
        std::fs::create_dir_all(&session_dir)?;
        
        let file_path = session_dir.join(format!("{}.json", name));
        let content = serde_json::to_string_pretty(&session)?;
        std::fs::write(file_path, content)?;
        
        println!("Session saved as: {}", name);
        Ok(())
    }

    fn load_session(&mut self, name: &str) -> Result<()> {
        let file_path = self.config.config_dir
            .join("sessions")
            .join(format!("{}.json", name));
            
        let content = std::fs::read_to_string(file_path)?;
        let session: Session = serde_json::from_str(&content)?;
        
        self.history = session.messages;
        self.current_model = session.model;
        
        println!("Loaded session: {}", name);
        self.show_history(None);
        Ok(())
    }
}