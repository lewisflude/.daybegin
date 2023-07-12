use anyhow::{Context, Result};
use log::info;
use std::process::{Command, Output};

pub fn execute_shell_command(command: &str) -> Result<()> {
    info!("Executing shell command: {}", command);

    let mut shell = Command::new("sh");
    shell.arg("-c").arg(command);
    shell.stdout(std::process::Stdio::inherit());
    shell.stderr(std::process::Stdio::inherit());

    let output = shell
        .output()
        .with_context(|| format!("Failed to execute shell command: {}", command))?;
    log_output(output)?;

    Ok(())
}

fn log_output(output: Output) -> Result<()> {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Shell command failed: {}", stderr);
    }

    Ok(())
}
