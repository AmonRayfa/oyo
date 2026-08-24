// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This build script derives the version string of the CLI from the state of the [Git](https://git-scm.com/) repository at
//! build time, and exposes it to the program through the `OYO_VERSION` environment variable. On a version branch, the version
//! string is the full version tagged on the checked-out commit (e.g. `v1-alpha.2`), or the base version (i.e. the branch
//! name, e.g. `v1`) if the commit isn't tagged or the tags weren't fetched (e.g. when installing with `cargo install --git`).
//! On any other branch (e.g. `dev`), the version string is the branch name itself.

use std::process::Command;

/// Runs a git command and returns the trimmed stdout on success.
fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok().filter(|output| output.status.success())?;
    let stdout = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if stdout.is_empty() { None } else { Some(stdout) }
}

/// Determines whether a branch name designates a version branch (i.e. `vN`).
fn is_version_branch(name: &str) -> bool {
    name.len() > 1 && name.starts_with('v') && name[1..].chars().all(|c| c.is_ascii_digit())
}

fn main() {
    // Rebuilds the program when the checked-out commit or the tags change.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");

    let version = match git(&["branch", "--show-current"]) {
        Some(branch) if is_version_branch(&branch) => git(&["describe", "--tags", "--exact-match"]).unwrap_or(branch),
        Some(branch) => branch,
        None => git(&["describe", "--tags", "--exact-match"]).unwrap_or_else(|| String::from("unknown")),
    };

    println!("cargo:rustc-env=OYO_VERSION={}", version);
}
