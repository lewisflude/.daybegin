use anyhow::{Context, Result};
use clap::{App, Arg};
use log::{debug, info, LevelFilter};

mod application;
mod config;
mod git;
mod shell;

#[tokio::main]
async fn main() -> Result<()> {
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

    // Execute Git operations
    git::sync_git_repo(config.git_branch.as_ref()).context("Failed to sync Git repository")?;
    info!("Git operations completed");

    // Execute shell commands
    for command in config.shell_commands.unwrap_or_default() {
        shell::execute_shell_command(&command).context("Failed to execute shell command")?;
    }
    info!("Shell commands executed");

    // Launch applications
    for app in config.applications.unwrap_or_default() {
        application::launch_application(&app).context("Failed to launch application")?;
    }
    info!("Applications launched");

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
