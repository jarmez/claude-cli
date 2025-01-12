use claude_common::{Config, types::{Session, Message}};
use tempfile::TempDir;
use std::path::PathBuf;
use chrono::Utc;

pub struct TestHelper {
    pub temp_dir: TempDir,
}

impl TestHelper {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().unwrap(),
        }
    }
    
    pub fn create_test_config(&self) -> Config {
        Config {
            api_key: "test-key".to_string(),
            default_model: "claude-3-sonnet".to_string(),
            config_dir: self.temp_dir.path().to_path_buf(),
            ..Config::default()
        }
    }
    
    pub fn create_test_session(&self, message_count: usize) -> Session {
        let mut messages = Vec::new();
        for i in 0..message_count {
            messages.push(Message {
                role: "user".to_string(),
                content: format!("Test message {}", i),
                timestamp: Utc::now(),
            });
            messages.push(Message {
                role: "assistant".to_string(),
                content: format!("Test response {}", i),
                timestamp: Utc::now(),
            });
        }
        
        Session {
            id: "test-session".to_string(),
            model: "claude-3-sonnet".to_string(),
            messages,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn create_test_file(&self, filename: &str, content: &str) -> PathBuf {
        let path = self.temp_dir.path().join(filename);
        std::fs::write(&path, content).unwrap();
        path
    }
}

pub fn assert_messages_equal(msg1: &Message, msg2: &Message) {
    assert_eq!(msg1.role, msg2.role);
    assert_eq!(msg1.content, msg2.content);
    // Note: We don't compare timestamps exactly as they might have microsecond differences
    assert!(msg1.timestamp.timestamp() == msg2.timestamp.timestamp());
}

pub fn create_mock_response(message: &str) -> String {
    format!("{{\"response\": \"{}\"}}", message)
}

#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        assert!($result.is_ok(), "Expected Ok, got Err: {:?}", $result.err().unwrap());
    };
}