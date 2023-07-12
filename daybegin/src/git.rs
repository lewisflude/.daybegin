use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::exit;
use std::process::Command;

pub fn perform_standard_git_steps(branch: &str) -> Result<()> {
    println!("Performing standard Git steps...");

    let current_branch = get_current_branch()?;
    println!("Current branch: {}", current_branch);

    sync_git_repo(branch)?;
    println!("Git repository synchronized");

    let choice = ask_for_rebase()?;
    if choice {
        perform_rebase(branch)?;
        println!("Rebase completed");
    } else {
        println!("Rebase skipped");
    }

    Ok(())
}

fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .context("Failed to get the current branch")?;

    if !output.status.success() {
        println!("Failed to get the current branch");
        exit(1);
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(branch)
}

fn sync_git_repo(branch: &str) -> Result<()> {
    let fetch_output = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(branch)
        .output()
        .context("Failed to fetch latest changes from the remote repository")?;

    if !fetch_output.status.success() {
        println!("Failed to fetch latest changes from the remote repository");
        exit(1);
    }

    let pull_output = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .output()
        .context("Failed to perform a pull to update the local branch")?;

    if !pull_output.status.success() {
        println!("Failed to perform a pull to update the local branch");
        exit(1);
    }

    Ok(())
}

fn ask_for_rebase() -> Result<bool> {
    print!("Do you want to perform a rebase against your default branch? (y/n): ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("Failed to read user input")?;

    let choice = buffer.trim().eq_ignore_ascii_case("y");
    Ok(choice)
}

fn perform_rebase(branch: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("rebase")
        .arg(branch)
        .output()
        .context("Failed to perform rebase")?;

    if !output.status.success() {
        println!("Failed to perform rebase");
        exit(1);
    }

    Ok(())
}
