// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod r#gen;
mod helpers;
mod phase;
mod rev;

pub(crate) use r#gen::run_gen;
use helpers::{check_init_repo, check_last_commit, git};
pub(crate) use phase::run_phase;
pub(crate) use rev::run_rev;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oyo")]
#[command(about = "A CLI tool that integrates Phased Versioning in Git repositories.", long_about = None)]
pub(crate) struct Cli {
    /// Returns the current version of the program.
    #[arg(short, long, exclusive = true)]
    pub(crate) version: bool,

    /// The subcommand to execute.
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Creates a new version branch.
    Gen {
        /// The generation number.
        #[arg(short, long)]
        number: Option<u64>,
    },

    /// Creates a tag with the provided phase name and active version branch (the revision is set to 0).
    Phase {
        /// The name of the new phase.
        name: String,
    },

    /// Creates a new tag by incrementing the revision number of the active version branch's last tag.
    Rev,
}
