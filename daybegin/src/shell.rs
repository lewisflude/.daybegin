use std::{
    io::{self, Write},
    process::Command,
};

pub fn execute_shell_command(command: &str) -> io::Result<()> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {}", command),
        ))
    }
}
