// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains helper functions for manipulating [Git](https://git-scm.com/).

mod sync;
pub(super) use sync::{count_distinct_phases, sync_version_metadata};

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

/// Ensures the repository was initialized with a commit.
/// Returns an `Error` if it wasn't.
pub(super) fn check_init_repo() -> Result<()> {
    if git(&["branch", "--list"])?.is_empty() {
        bail!("The repository must be initialized with a commit before \x1b[4moyo\x1b[0m can be used.");
    }
    Ok(())
}

/// Ensures that there are no tags on the last commit of the current branch.
/// Returns an `Error` if there are.
pub(super) fn check_last_commit() -> Result<()> {
    let existing_tags = git(&["tag", "--points-at", "HEAD"])?;
    if !existing_tags.is_empty() {
        bail!("The last commit on this branch is already associated with one or more tags:\n{}", existing_tags);
    }
    Ok(())
}
