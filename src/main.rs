use clap::Parser;
use std::io::{self, Read};
mod config;
mod gemini;

#[derive(Parser)]
#[command(name = "fai", about = "Format with AI", version = "1.0")]
struct Cli {
    /// Prompt
    prompt: String,
}

#[tokio::main]
async fn main() {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let config_path = home_dir.join(".config").join("fai").join("config.toml");
    let config = config::load_or_create_config(&config_path).expect("Failed to load config");

    let cli = Cli::parse();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    if config.default_provider == "gemini" {
        match gemini::format_text(&config.providers.gemini, &buffer, &cli.prompt).await {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("Request failed: {}", err),
        }
    }
}
