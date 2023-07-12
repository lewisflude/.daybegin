use anyhow::{Context, Result};
use log::info;
use std::process::Command;

pub fn execute_shell_command(command: &str) -> Result<()> {
    info!("Executing shell command: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .with_context(|| format!("Failed to execute shell command: {}", command))?;

    log_output(output)?;

    Ok(())
}

fn log_output(output: std::process::Output) -> Result<()> {
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        info!("Shell stdout: {}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        info!("Shell stderr: {}", stderr);
    }

    Ok(())
}
