use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub root_dir: String,
    pub manifest_path: String,
    pub work_dir: String,
    pub git_branch: String,
    pub shell_commands: Vec<String>,
    pub applications: Vec<String>,
    pub app_dir: String, // New field for the applications directory
}

impl Default for Config {
    fn default() -> Self {
        let app_dir = if cfg!(target_os = "macos") {
            "/Applications"
        } else if cfg!(target_os = "windows") {
            "C:\\Program Files"
        } else {
            "/usr/bin"
        };

        Self {
            root_dir: String::from("~/.daybegin"),
            manifest_path: String::from("~/.daybegin/daybegin/Cargo.toml"),
            work_dir: String::from("~/my_project"),
            git_branch: String::from("main"),
            shell_commands: vec![String::from("make clean"), String::from("make build")],
            applications: vec![
                String::from("Docker.app"),
                String::from("Visual Studio Code.app"),
            ],
            app_dir: String::from(app_dir),
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
            work_dir: String::new(),
            git_branch: String::new(),
            shell_commands: default_config.shell_commands.clone(),
            applications: default_config.applications.clone(),
            app_dir: default_config.app_dir.clone(),
        };

        println!("🌞 Let's create a new config file for Daybegin! 🌞");

        // Prompt for work_dir
        print!(
            "Enter the desired working directory [{}]: ",
            default_config.work_dir
        );
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        // Prompt for git_branch
        print!(
            "Enter the desired Git branch [{}]: ",
            default_config.git_branch
        );
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        // Prompt for shell_commands
        print!(
            "Enter the desired shell commands (comma-separated) [{:?}]: ",
            default_config.shell_commands
        );
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let shell_commands = buffer
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        config.shell_commands = shell_commands;

        // Prompt for applications
        print!(
            "Enter the desired applications (comma-separated) [{}]: ",
            default_config.applications.join(", ")
        );
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let applications = buffer
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        config.applications = applications;

        // Save the updated config
        let config_path = Config::get_config_path()?;
        let toml_string = toml::to_string(&config)?;
        fs::write(&config_path, &toml_string)?;

        let toml_content =
            toml::to_string(&config).context("Failed to serialize config to TOML")?;

        let mut config_file = fs::File::create(path).context("Failed to create the config file")?;
        config_file
            .write_all(toml_content.as_bytes())
            .context("Failed to write config to file")?;
        config_file.flush().context("Failed to flush config file")?;

        println!("✨ Config file created at: {} ✨", path.display());
        Ok(())
    }
}
