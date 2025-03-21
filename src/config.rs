use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_provider: String,
    pub providers: Providers,
}

#[derive(Serialize, Deserialize)]
pub struct Providers {
    pub gemini: Gemini,
}

#[derive(Serialize, Deserialize)]
pub struct Gemini {
    pub api_key: String,
}

fn get_config_from_ui() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config {
        default_provider: "".to_string(),
        providers: Providers {
            gemini: Gemini {
                api_key: "".to_string(),
            },
        },
    };

    let provider = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select default AI provider")
        .default(0) // Index of the default selection
        .items(&["Gemini"])
        .interact()?;

    // Gemini
    if provider == 0 {
        let api_key: String = Input::new().with_prompt("Enter API key").interact_text()?;

        config.default_provider = "gemini".to_string();
        config.providers.gemini.api_key = api_key;
    }

    return Ok(config);
}

pub fn load_or_create_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    if !path.exists() {
        let config = get_config_from_ui()?;
        let toml_string = toml::to_string(&config).expect("Failed to serialize to TOML");

        let parent = path.parent().expect("Parent path does not exist");
        std::fs::create_dir_all(parent).expect("Cannot create path to config file");
        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(config)
    } else {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::de::from_str(&contents)?;

        Ok(config)
    }
}
