use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default = "default_root_dir")]
    pub root_dir: String,
    #[serde(default = "default_manifest_path")]
    pub manifest_path: String,
    #[serde(default)]
    pub work_dir: String,
    #[serde(default)]
    pub git_branch: String,
    #[serde(default = "default_shell_commands")]
    pub shell_commands: Vec<String>,
    #[serde(default = "default_applications")]
    pub applications: Vec<String>,
    #[serde(default = "default_app_dir")]
    pub app_dir: String,
}

fn default_root_dir() -> String {
    String::from("~/.daybegin")
}

fn default_manifest_path() -> String {
    String::from("~/.daybegin/daybegin/Cargo.toml")
}

fn default_shell_commands() -> Vec<String> {
    vec![String::from("make clean"), String::from("make build")]
}

fn default_applications() -> Vec<String> {
    vec![
        String::from("Docker.app"),
        String::from("Visual Studio Code.app"),
    ]
}

fn default_app_dir() -> String {
    if cfg!(target_os = "macos") {
        "/Applications".to_string()
    } else if cfg!(target_os = "windows") {
        "C:\\Program Files".to_string()
    } else {
        "/usr/bin".to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_dir: default_root_dir(),
            manifest_path: default_manifest_path(),
            work_dir: String::new(),
            git_branch: String::new(),
            shell_commands: default_shell_commands(),
            applications: default_applications(),
            app_dir: default_app_dir(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            println!("No config file found. Let's create one.");
            Self::create_config_file(&config_path)?;
        }
        let config_content =
            fs::read_to_string(&config_path).context("Failed to read the config file")?;
        let config: Config =
            toml::from_str(&config_content).context("Failed to parse the config file")?;
        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut config_dir =
            dirs::home_dir().context("Failed to determine user's home directory")?;
        config_dir.push(".daybegin");
        fs::create_dir_all(&config_dir).context("Failed to create the config directory")?;
        Ok(config_dir.join("config.toml"))
    }

    fn create_config_file(path: &Path) -> Result<()> {
        let default_config: Config = Default::default();

        let mut config = Config {
            root_dir: default_config.root_dir.clone(),
            manifest_path: default_config.manifest_path.clone(),
            work_dir: default_config.work_dir.clone(),
            git_branch: String::new(),
            shell_commands: default_config.shell_commands.clone(),
            applications: default_config.applications.clone(),
            app_dir: default_config.app_dir.clone(),
        };

        println!("🌞 Let's create a new config file for Daybegin! 🌞");

        // Prompt for work_dir
        let work_dir = prompt_with_default(
            "Enter the desired working directory",
            &default_config.work_dir,
        )?;
        config.work_dir = work_dir;

        // Prompt for git_branch
        let git_branch =
            prompt_with_default("Enter the desired Git branch", &default_config.git_branch)?;
        config.git_branch = git_branch;

        // Prompt for shell_commands
        let shell_commands = prompt_csv_with_default(
            "Enter the desired shell commands (comma-separated)",
            &default_config.shell_commands,
        )?;
        config.shell_commands = shell_commands;

        // Prompt for applications
        let applications = prompt_csv_with_default(
            "Enter the desired applications (comma-separated)",
            &default_config.applications,
        )?;
        config.applications = applications;

        // Save the updated config
        let config_path = Config::get_config_path()?;
        let toml_string = toml::to_string(&config).context("Failed to serialize config to TOML")?;
        fs::write(&config_path, &toml_string).context("Failed to write config to file")?;

        println!("✨ Config file created at: {} ✨", path.display());
        Ok(())
    }
}

fn prompt_with_default(prompt: &str, default_value: &str) -> Result<String> {
    print!("{} [{}]: ", prompt, default_value);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("Failed to read user input")?;
    let trimmed = buffer.trim();
    if !trimmed.is_empty() {
        Ok(trimmed.to_string())
    } else {
        Ok(default_value.to_string())
    }
}

fn prompt_csv_with_default(prompt: &str, default_values: &[String]) -> Result<Vec<String>> {
    let default_string = default_values.join(", ");
    print!("{} [{}]: ", prompt, default_string);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("Failed to read user input")?;
    let trimmed = buffer.trim();
    if !trimmed.is_empty() {
        Ok(trimmed
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>())
    } else {
        Ok(default_values.to_vec())
    }
}
