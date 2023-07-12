use anyhow::{Context, Result};
use log::info;
use std::process::Command;

pub fn launch_application(app: &str) -> Result<()> {
    info!("Launching application: {}", app);

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg("-a")
            .arg(app)
            .output()
            .with_context(|| format!("Failed to launch application: {}", app))?;
        log_output(output)?;
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(app)
            .output()
            .with_context(|| format!("Failed to launch application: {}", app))?;
        log_output(output)?;
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(app)
            .output()
            .with_context(|| format!("Failed to launch application: {}", app))?;
        log_output(output)?;
    }

    Ok(())
}

fn log_output(output: std::process::Output) -> Result<()> {
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        info!("Stdout: {}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        info!("Stderr: {}", stderr);
    }

    Ok(())
}
