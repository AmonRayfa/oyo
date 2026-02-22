// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::{Result, bail};
use std::process::Command;

/// Runs a git command.
/// Returns the stdout as a trimmed `String` on success.
/// Returns an `Error` with stderr info on failure.
pub(super) fn git(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;

    if !output.status.success() {
        // Capture the error message from git (e.g. "fatal: tag 'v1' already exists")
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git command failed: git {}\nError: {}", args.join(" "), stderr.trim());
    }

    // Return stdout, trimmed of whitespace
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub(super) fn check_init_repo() -> Result<()> {
    if git(&["branch", "--list"])?.is_empty() {
        bail!("You must initialize the repository with a commit before using oyo.");
    }
    Ok(())
}

pub(super) fn check_last_commit() -> Result<()> {
    let existing_tags = git(&["tag", "--points-at", "HEAD"])?;
    if !existing_tags.is_empty() {
        bail!("The last commit on this branch already has one or multiple tags:\n{}", existing_tags);
    }
    Ok(())
}
