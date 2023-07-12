use crate::config::Config;
use std::io::{self, Error, ErrorKind};
use std::process::{Command, Output};

pub struct CommandResult {
    pub exit_status: std::process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

pub fn launch_application(application: &str, config: &Config) -> io::Result<CommandResult> {
    if application.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Empty command"));
    }

    let mut command = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else {
        Command::new("sh")
    };

    if cfg!(target_os = "windows") {
        command.args(&["/C", application]);
    } else {
        command.arg(application);
    }

    let output = if cfg!(target_os = "macos") {
        command.current_dir(&config.app_dir).output()
    } else {
        command.output()
    }
    .map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to execute application '{}': {}", application, err),
        )
    })?;

    let CommandResult {
        exit_status,
        stdout,
        stderr,
    } = process_output(output);

    if exit_status.success() {
        Ok(CommandResult {
            exit_status,
            stdout,
            stderr,
        })
    } else {
        eprintln!("Error executing application '{}': {}", application, stderr);
        Err(Error::new(
            ErrorKind::Other,
            format!("Command execution error: {}", stderr),
        ))
    }
}

fn process_output(output: Output) -> CommandResult {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    CommandResult {
        exit_status: output.status,
        stdout,
        stderr,
    }
}

pub fn wait_for_applications(applications: &[String], config: &Config) -> io::Result<()> {
    for application in applications {
        let command_result = launch_application(application, config)?;

        if !command_result.exit_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Application '{}' exited with non-zero status code",
                    application
                ),
            ));
        }
    }

    Ok(())
}
