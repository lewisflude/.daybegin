use crate::{config::Config, shell::execute_shell_commands};
use anyhow::{Context, Result};
use clap::{App, Arg};
use log::{debug, info, LevelFilter};

mod application;
mod config;
mod git;
mod shell;

fn main() -> Result<()> {
    // Parse command-line arguments
    let matches = App::new("daybegin")
        .version("0.1.0")
        .author("Lewis Flude <lewis@lewisflude.com>")
        .about("Performs common tasks at the start of your work day")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turn on verbose mode"),
        )
        .arg(
            Arg::with_name("work_dir")
                .short("d")
                .long("dir")
                .value_name("WORK_DIR")
                .help("Overrides the work directory")
                .takes_value(true),
        )
        .get_matches();

    // Setup logging
    setup_logging(matches.is_present("verbose"))?;

    info!("Starting daybegin");

    // Load configuration
    let config = config::Config::load().context("Failed to load configuration")?;
    debug!("Loaded config: {:?}", config);

    // Sync the Git repository
    if !config.git_branch.is_empty() {
        sync_git_repo(&config)?;
    }

    // Launch applications
    if !config.applications.is_empty() {
        launch_applications(&config)?;
    }

    // Wait for launched applications to finish
    if !config.applications.is_empty() {
        wait_for_applications(&config)?;
    }

    // Execute shell commands
    if !config.shell_commands.is_empty() {
        execute_shell_commands(&config)?;
    }

    info!("daybegin completed");
    Ok(())
}

fn setup_logging(verbose: bool) -> Result<()> {
    let log_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let mut builder = env_logger::Builder::new();
    builder.filter(None, log_level);
    builder.init();

    Ok(())
}

fn sync_git_repo(config: &Config) -> Result<()> {
    git::perform_standard_git_steps(&config.git_branch).with_context(|| {
        format!(
            "Failed to sync Git repository with branch: {}",
            &config.git_branch
        )
    })?;
    info!("Git repository synchronized");
    Ok(())
}

fn launch_applications(config: &Config) -> Result<()> {
    for app in &config.applications {
        application::launch_application(app, &config)
            .with_context(|| format!("Failed to launch application: {}", app))?;
        info!("Application {} launched", app);
    }
    Ok(())
}

fn wait_for_applications(config: &Config) -> Result<(), anyhow::Error> {
    if let Err(err) = application::wait_for_applications(&config.applications, &config) {
        anyhow::bail!("Error waiting for applications: {}", err);
    }
    Ok(())
}
