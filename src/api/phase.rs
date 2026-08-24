// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module defines the function behind the `oyo phase <name>` command.

use super::{check_init_repo, check_last_commit, count_distinct_phases, git, sync_version_metadata};
use mabe::{Context, Result, bail};
use regex::Regex;

/// Handles the `phase` subcommand.
pub(crate) fn run_phase(name: String, dry_run: bool) -> Result<()> {
    check_init_repo()?;
    check_last_commit()?;

    let current_branch = git(&["branch", "--show-current"])?.trim().to_string();
    println!("🔍 Inspecting current branch ({})...", current_branch);

    let branch_pattern = Regex::new(r"^v([1-9]\d*)$").unwrap();
    let r#gen =
        branch_pattern.captures(&current_branch).context("Current branch is not a valid version branch (vN).")?[1].to_string();

    println!("📋 Validating phase name...");

    if name.is_empty() {
        bail!("Invalid phase name: the name cannot be empty.");
    }

    if !name.chars().all(|c| c.is_ascii_lowercase()) {
        bail!("Invalid phase name: '{}'. Only lowercase ASCII characters [a-z] are allowed.", name);
    }

    let tag_pattern = Regex::new(&format!(r"^v{}-([a-z]+)\.(0|[1-9]\d*)$", r#gen)).unwrap();
    let tags: Vec<String> = git(&["tag", "--list", &format!("v{}-*", r#gen), "--sort=creatordate"])?
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let previous_version = match tags.iter().rev().find_map(|tag| tag_pattern.captures(tag)) {
        Some(last_version_tag) => {
            let previous_phase = last_version_tag[1].to_string();
            let previous_rev = last_version_tag[2].to_string();
            let previous_char = previous_phase.chars().next().unwrap();
            let new_char = name.chars().next().unwrap();

            if previous_char == 'z' {
                if new_char != 'a' {
                    bail!(
                        "Invalid phase jump: '{}' -> '{}'. The previous phase starts with 'z', so the new phase name must start with 'a'.",
                        previous_phase,
                        name
                    );
                }
            } else if new_char as u8 != previous_char as u8 + 1 {
                bail!(
                    "Invalid phase jump: '{}' -> '{}'. The previous phase starts with '{}', so the new phase name must start with '{}'.",
                    previous_phase,
                    name,
                    previous_char,
                    (previous_char as u8 + 1) as char
                );
            }

            Some((previous_phase, previous_rev))
        }
        None => {
            if !name.starts_with('a') {
                bail!(
                    "Invalid phase name: '{}'. There are no prior version tags on the current branch, so the phase name must start with 'a'.",
                    name
                );
            }

            None
        }
    };

    let new_version = format!("v{}-{}.0", r#gen, name);
    let semver = format!("{}.{}.0", r#gen, count_distinct_phases(&tags, &tag_pattern));

    if dry_run {
        println!("🔎 Dry run: the next version would be {} (SemVer projection: {}).", new_version, semver);
        return Ok(());
    }

    sync_version_metadata(&new_version, &semver)?;
    git(&["tag", "-a", &new_version, "-m", &new_version])?;

    match previous_version {
        Some((previous_phase, previous_rev)) => {
            println!("🔀 Phase transition: v{}-{}.{} -> {}", r#gen, previous_phase, previous_rev, new_version)
        }
        None => println!("✨ Phase initialization: {}", new_version),
    }

    Ok(())
}
