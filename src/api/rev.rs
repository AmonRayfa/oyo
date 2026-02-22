// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use super::{check_init_repo, check_last_commit, git};
use mabe::{Context, Result, bail};
use regex::Regex;

pub(crate) fn run_rev() -> Result<()> {
    check_init_repo()?;
    check_last_commit()?;

    let current_branch = git(&["branch", "--show-current"])?.trim().to_string();
    println!("🔍 Inspecting current branch ({})...", current_branch);

    let branch_pattern = Regex::new(r"^v(0|[1-9]\d*)$").unwrap();
    let r#gen = branch_pattern
        .captures(&current_branch)
        .context("Current branch is not a valid version branch (vN). Cannot create phase on this branch.")?[1]
        .to_string();

    let tag_pattern = Regex::new(&format!(r"^v{}-([a-z]+)\.(0|[1-9]\d*)$", r#gen)).unwrap();
    let tags: Vec<String> = git(&["tag", "--list", &format!("v{}-*", r#gen), "--sort=creatordate"])?
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    match tags.iter().rev().find_map(|tag| tag_pattern.captures(tag)) {
        Some(last_version_tag) => {
            let phase: &str = &last_version_tag[1];
            let previous_rev: u64 = last_version_tag[2].parse()?;

            git(&["tag", &format!("v{}-{}.{}", r#gen, phase, previous_rev + 1)])?;
            println!("⬆️ Bumped revision number on phase {}: {} -> {}", phase, previous_rev, previous_rev + 1);
        }
        None => {
            bail!("The current branch (v{}) has no phase yet.", r#gen);
        }
    }

    Ok(())
}
