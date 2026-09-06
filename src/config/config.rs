use super::defaults;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "defaults::notes_directory")]
    pub directory: PathBuf,
    #[serde(default = "defaults::reader")]
    pub reader: String,
    #[serde(default = "defaults::reader")]
    pub preview_reader: String,
    #[serde(default = "defaults::editor")]
    pub editor: String,
    #[serde(default = "defaults::file_extension")]
    pub file_extension: String,
}

fn config_directory() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/fznote")
}

fn config_file_path() -> PathBuf {
    config_directory()
        .join("config.yaml")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: defaults::notes_directory(),
            reader: defaults::reader(),
            preview_reader: defaults::reader(),
            editor: defaults::editor(),
            file_extension: defaults::file_extension(),
        }
    }
}

impl Config {
    // Load config from file, or create default if it doesn't exist
    pub fn load() -> anyhow::Result<Self> {
        let config_file_path = config_file_path();

        let config = if config_file_path.exists() {
            // Load from file
            let content = std::fs::read_to_string(&config_file_path)?;
            serde_yaml::from_str(&content)?
        } else {
            // Create default config
            println!("Creating default config");
            let default_config = Config::default();

            // Create config directory if it doesn't exit
            let config_dir = config_directory();
            if !config_dir.exists() {
            println!("Creating config directory: {}", config_dir.display());
                std::fs::create_dir_all(&config_dir)?;
            }

            // Write config file
            let yaml = serde_yaml::to_string(&default_config)?;
            std::fs::write(&config_file_path, yaml)?;

            default_config
        };

        // Create notes directory if it doesn't exist
        if !config.directory.exists() {
        println!("Creating notes directory: {}", config.directory.display());
            std::fs::create_dir_all(&config.directory)?;
        }

        Ok(config)
    }
}
