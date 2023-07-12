use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub work_dir: Option<String>,
    pub git_branch: Option<String>,
    pub shell_commands: Option<Vec<String>>,
    pub applications: Option<Vec<String>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config_content = fs::read_to_string(&config_path)
            .context("Failed to read the config file")?;
        let config: Config = toml::from_str(&config_content)
            .context("Failed to parse the config file")?;
        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut config_dir = dirs::home_dir().context("Failed to determine user's home directory")?;
        config_dir.push(".daybegin");
        fs::create_dir_all(&config_dir)
            .context("Failed to create the config directory")?;
        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            fs::write(&config_file, include_str!("default_config.toml"))
                .context("Failed to create the default config file")?;
        }
        Ok(config_file)
    }
}
