use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub allowed_commands: Vec<String>,
    pub log_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            allowed_commands: crate::tools::ALLOWED_COMMANDS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            log_path: "audit.log".to_string(), // Relativo ao diretório de config
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().context("Failed to get config directory")?;
    path.push("linux-mcp");
    Ok(path)
}

pub fn load() -> Result<Config> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        // Criar arquivo de configuração padrão
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config)?;
        fs::write(&config_path, toml_str).context("Failed to write default config")?;
        return Ok(config);
    }

    let contents = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;

    Ok(config)
}
