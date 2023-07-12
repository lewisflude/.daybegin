use std::process::Command;
use anyhow::{Result, Context};

pub fn sync_git_repo(branch: Option<&String>) -> Result<()> {
  if let Some(branch) = branch {
      // Convert the branch String to a string slice
      let branch_slice: &str = branch.as_ref();

      Command::new("git")
          .arg("fetch")
          .arg("--all")
          .output()
          .context("Failed to execute git fetch")?;

      Command::new("git")
          .arg("rebase")
          .arg(branch_slice)
          .output()
          .context("Failed to execute git rebase")?;
  }

  Ok(())
}


