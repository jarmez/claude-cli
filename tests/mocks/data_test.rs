use proptest::prelude::*;
use claude_common::{Config, types::{Session, Message}};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

proptest! {
    #[test]
    fn test_config_serialization(
        api_key in "[A-Za-z0-9]{32}",
        model in prop::sample::select(vec!["claude-3-opus", "claude-3-sonnet"]),
    ) {
        let config = Config {
            api_key: api_key.clone(),
            default_model: model.to_string(),
            config_dir: PathBuf::from("/tmp/test"),
            ..Config::default()
        };
        
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        
        prop_assert_eq!(config.api_key, deserialized.api_key);
        prop_assert_eq!(config.default_model, deserialized.default_model);
    }
    
    #[test]
    fn test_session_history(
        messages in prop::collection::vec(
            any::<String>(), 
            0..100
        )
    ) {
        let mut session = Session {
            id: "test".to_string(),
            model: "claude-3-sonnet".to_string(),
            messages: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Add generated messages to session
        for content in messages {
            session.messages.push(Message {
                role: "user".to_string(),
                content: content.clone(),
                timestamp: Utc::now(),
            });
            
            session.messages.push(Message {
                role: "assistant".to_string(),
                content: format!("Response to: {}", content),
                timestamp: Utc::now(),
            });
        }
        
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&serialized).unwrap();
        
        prop_assert_eq!(session.messages.len(), deserialized.messages.len());
        prop_assert_eq!(session.model, deserialized.model);
    }
    
    #[test]
    fn test_message_content(
        content in ".*",
        role in prop::sample::select(vec!["user", "assistant"]),
    ) {
        let message = Message {
            role: role.to_string(),
            content: content.clone(),
            timestamp: Utc::now(),
        };
        
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        
        prop_assert_eq!(message.role, deserialized.role);
        prop_assert_eq!(message.content, deserialized.content);
    }
    
    #[test]
    fn test_config_paths(
        dir in "[a-zA-Z0-9/._-]{1,100}"
    ) {
        let config = Config {
            config_dir: PathBuf::from(&dir),
            ..Config::default()
        };
        
        // Test path validity
        prop_assert!(config.config_dir.to_str().is_some());
        
        // Test history file path
        let history_path = config.config_dir.join("history.json");
        prop_assert!(history_path.to_str().is_some());
    }
}

// Custom strategy for generating valid timestamps
fn timestamp_strategy() -> impl Strategy<Value = DateTime<Utc>> {
    // Generate timestamps within a reasonable range
    (1990i32..2030)
        .prop_map(|year| {
            Utc::now()
                .with_year(year)
                .unwrap_or_else(|| Utc::now())
        })
}

proptest! {
    #[test]
    fn test_message_timestamps(
        ts in timestamp_strategy(),
    ) {
        let message = Message {
            role: "user".to_string(),
            content: "test".to_string(),
            timestamp: ts,
        };
        
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        
        prop_assert_eq!(message.timestamp, deserialized.timestamp);
    }
}