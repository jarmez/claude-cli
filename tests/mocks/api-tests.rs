use mockall::automock;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
#[automock]
trait ClaudeApi {
    async fn chat(&self, message: &str, model: &str) -> Result<String>;
    async fn list_models(&self) -> Result<Vec<String>>;
}

struct MockClient {
    api: MockClaudeApi,
}

#[tokio::test]
async fn test_chat_response() {
    let mut mock_api = MockClaudeApi::new();
    
    // Set up expectations
    mock_api.expect_chat()
        .with(mockall::predicate::eq("Hello"), mockall::predicate::eq("claude-3-sonnet"))
        .times(1)
        .returning(|_, _| Ok("Hello! How can I help you today?".to_string()));
    
    let client = MockClient { api: mock_api };
    
    // Test the chat interaction
    let response = client.api.chat("Hello", "claude-3-sonnet").await.unwrap();
    assert_eq!(response, "Hello! How can I help you today?");
}

#[tokio::test]
async fn test_error_handling() {
    let mut mock_api = MockClaudeApi::new();
    
    // Simulate API error
    mock_api.expect_chat()
        .returning(|_, _| Err(anyhow::anyhow!("API Error")));
    
    let client = MockClient { api: mock_api };
    
    // Test error handling
    let result = client.api.chat("Test", "claude-3-sonnet").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_model_list() {
    let mut mock_api = MockClaudeApi::new();
    
    mock_api.expect_list_models()
        .times(1)
        .returning(|| Ok(vec![
            "claude-3-opus".to_string(),
            "claude-3-sonnet".to_string(),
        ]));
    
    let client = MockClient { api: mock_api };
    
    let models = client.api.list_models().await.unwrap();
    assert_eq!(models.len(), 2);
    assert!(models.contains(&"claude-3-sonnet".to_string()));
}

#[tokio::test]
async fn test_concurrent_requests() {
    let mut mock_api = MockClaudeApi::new();
    
    mock_api.expect_chat()
        .returning(|msg, _| Ok(format!("Response to: {}", msg)));
    
    let client = MockClient { api: mock_api };
    
    // Test multiple concurrent requests
    let futures = vec![
        client.api.chat("First", "claude-3-sonnet"),
        client.api.chat("Second", "claude-3-sonnet"),
        client.api.chat("Third", "claude-3-sonnet"),
    ];
    
    let results = futures::future::join_all(futures).await;
    assert!(results.iter().all(|r| r.is_ok()));
}