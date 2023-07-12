use std::{
    io::{self, BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use crate::config::Config;

pub fn execute_shell_command(command: &str) -> io::Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let mut stdout_handle = io::stdout();
    let mut stderr_handle = io::stderr();

    // Copy stdout
    for line in stdout_reader.lines() {
        writeln!(stdout_handle, "{}", line?)?;
    }

    // Copy stderr
    for line in stderr_reader.lines() {
        writeln!(stderr_handle, "{}", line?)?;
    }

    let exit_status = child.wait()?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Command failed with exit code {}: {}",
                exit_status.code().unwrap_or(0),
                command
            ),
        ))
    }
}

pub fn execute_shell_commands(config: &Config) -> io::Result<()> {
    for command in &config.shell_commands {
        execute_shell_command(command)?;
    }
    Ok(())
}
