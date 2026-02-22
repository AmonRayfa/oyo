// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use super::{check_init_repo, check_last_commit, git};
use mabe::{Context, Result, bail};
use regex::Regex;

pub(crate) fn run_phase(name: String) -> Result<()> {
    check_init_repo()?;
    check_last_commit()?;

    let current_branch = git(&["branch", "--show-current"])?.trim().to_string();
    println!("🔍 Inspecting current branch ({})...", current_branch);

    let branch_pattern = Regex::new(r"^v(0|[1-9]\d*)$").unwrap();
    let r#gen = branch_pattern
        .captures(&current_branch)
        .context("Current branch is not a valid version branch (vN). Cannot create phase on this branch.")?[1]
        .to_string();

    println!("📋 Validating phase name...");

    if !name.chars().all(|c| c.is_ascii_lowercase()) {
        bail!("Phase name '{}' is invalid. Only lowercase ASCII characters [a-z] are allowed..", name);
    }

    let tag_pattern = Regex::new(&format!(r"^v{}-([a-z]+)\.(0|[1-9]\d*)$", r#gen)).unwrap();
    let tags: Vec<String> = git(&["tag", "--list", &format!("v{}-*", r#gen), "--sort=creatordate"])?
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    match tags.iter().rev().find_map(|tag| tag_pattern.captures(tag)) {
        Some(last_version_tag) => {
            let previous_phase = &last_version_tag[1];
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

            git(&["tag", &format!("v{}-{}.0", r#gen, name)])?;
            println!("↪️ Transitioned from phase {} to phase {} on branch v{}...", previous_phase, name, r#gen);
        }
        None => {
            if !name.starts_with('a') {
                bail!(
                    "Phase name '{}' is invalid. There are no prior version tags for the current version branch, so the phase name must start with 'a'.",
                    name
                );
            }

            git(&["tag", &format!("v{}-{}.0", r#gen, name)])?;
            println!("✨ Initialized branch v{} with phase {}...", r#gen, name);
        }
    }

    Ok(())
}
