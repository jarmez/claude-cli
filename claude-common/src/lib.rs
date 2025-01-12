pub mod config;
pub mod api;

// Re-export main types
pub use config::Config;
pub use config::OutputFormat;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, Error>;