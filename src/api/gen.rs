// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module defines the function behind the `oyo gen [-n/--number <generation_number>]` command.

use super::{check_init_repo, git};
use mabe::Result;
use regex::Regex;

/// Handles the `gen` subcommand.
pub(crate) fn run_gen(number: Option<u64>) -> Result<()> {
    check_init_repo()?;

    let branch_name = match number {
        Some(r#gen) => format!("v{}", r#gen),
        None => {
            println!("🧮 Calculating generation number...");

            let branch_pattern = Regex::new(r"^v(0|[1-9]\d*)$").unwrap();
            let mut generations: Vec<u64> = Vec::new();

            let local_branches = git(&["branch", "--list", "--format=%(refname:short)"])?;
            let remote_branches = git(&["branch", "--remotes", "--format=%(refname:short)"])?;

            // Remote branches are listed as `<remote>/<branch>`, so the remote prefix is stripped.
            let branch_names = local_branches
                .lines()
                .map(str::trim)
                .chain(remote_branches.lines().filter_map(|line| line.trim().split_once('/').map(|(_, name)| name)));

            for branch_name in branch_names {
                if let Some(cap) = branch_pattern.captures(branch_name)
                    && let Ok(r#gen) = cap[1].parse::<u64>()
                {
                    generations.push(r#gen);
                }
            }

            match generations.iter().max() {
                Some(&max_gen) => format!("v{}", &max_gen + 1),
                None => String::from("v1"),
            }
        }
    };

    println!("🌿 Creating version branch {}...", branch_name);
    git(&["checkout", "-b", &branch_name])?;
    println!("✅ Switched to new version branch.");

    Ok(())
}
