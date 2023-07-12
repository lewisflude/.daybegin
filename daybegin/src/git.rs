use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::exit;
use std::process::Command;

pub fn perform_standard_git_steps(branch: &str) -> Result<()> {
    println!("Performing standard Git steps...");

    let current_branch = get_current_branch()?;
    println!("Current branch: {}", current_branch);

    sync_git_repo(&current_branch)?;
    println!("Git repository synchronized");

    let choice = ask_for_rebase()?;
    if choice {
        if let Some(default_branch) = get_default_branch() {
            perform_rebase(&default_branch, branch)?; // Pass the 'branch' parameter
            println!("Rebase completed");
        } else {
            println!("Failed to determine the default branch");
            exit(1);
        }
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
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Failed to get the current branch: {}", stderr);
        exit(1);
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(branch)
}

fn sync_git_repo(branch: &str) -> Result<()> {
    println!("Synchronizing Git repository...");

    let fetch_output = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(branch)
        .output()
        .context("Failed to fetch latest changes from the remote repository")?;

    if !fetch_output.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_output.stderr);
        println!(
            "Warning: Failed to fetch latest changes from the remote repository: {}",
            stderr
        );
    }

    let pull_output = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .output()
        .context("Failed to perform a pull to update the local branch")?;

    if !pull_output.status.success() {
        let stderr = String::from_utf8_lossy(&pull_output.stderr);
        println!(
            "Failed to perform a pull to update the local branch: {}",
            stderr
        );
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

fn get_default_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("symbolic-ref")
        .arg("refs/remotes/origin/HEAD")
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout)
            .trim_start_matches("refs/remotes/origin/")
            .trim()
            .to_string();
        Some(branch)
    } else {
        None
    }
}

fn stash_changes() -> Result<()> {
    let stash_output = Command::new("git")
        .arg("stash")
        .arg("save")
        .arg("--include-untracked")
        .output()
        .context("Failed to stash changes")?;

    if !stash_output.status.success() {
        let stderr = String::from_utf8_lossy(&stash_output.stderr);
        println!("Failed to stash changes: {}", stderr);
        exit(1);
    }

    Ok(())
}

fn read_user_choice() -> Result<bool> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("Failed to read user input")?;

    let choice = buffer.trim().eq_ignore_ascii_case("y");
    Ok(choice)
}

fn perform_rebase(default_branch: &str, branch: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .context("Failed to check Git status")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Failed to check Git status: {}", stderr);
        exit(1);
    }

    let status_output = String::from_utf8_lossy(&output.stdout);
    let has_changes = !status_output.trim().is_empty();

    if has_changes {
        println!("You have unstaged changes. Would you like to stash them before performing the rebase? (y/n): ");
        let choice = read_user_choice()?;

        if choice {
            stash_changes()?;
        } else {
            println!("Please commit or stash your changes before performing the rebase.");
            return Ok(());
        }
    }

    let rebase_output = Command::new("git")
        .arg("rebase")
        .arg(default_branch)
        .arg(branch)
        .output()
        .context("Failed to perform rebase")?;

    if !rebase_output.status.success() {
        let stderr = String::from_utf8_lossy(&rebase_output.stderr);
        println!("Failed to perform rebase: {}", stderr);
        exit(1);
    }

    if has_changes {
        pop_stash()?;
    }

    Ok(())
}

fn pop_stash() -> Result<()> {
    let pop_output = Command::new("git")
        .arg("stash")
        .arg("pop")
        .output()
        .context("Failed to pop the stash")?;

    if !pop_output.status.success() {
        let stderr = String::from_utf8_lossy(&pop_output.stderr);
        println!("Failed to pop the stash: {}", stderr);
        exit(1);
    }

    Ok(())
}
