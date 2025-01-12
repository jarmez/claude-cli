use anyhow::Result;
use clap::Parser;
use claude_common::Config;  // This should work now with the re-export

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Show current configuration
    #[arg(short, long)]
    show: bool,

    /// Reset configuration to defaults
    #[arg(short, long)]
    reset: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    if cli.show {
        println!("Current configuration:");
        let config = Config::load()?;
        println!("{:#?}", config);
    } else if cli.reset {
        let config = Config::default();
        config.save()?;
        println!("Configuration reset to defaults");
    } else {
        println!("Use --help to see available options");
    }
    
    Ok(())
}