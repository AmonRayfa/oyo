// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Oyo**](https://github.com/AmonRayfa/oyo) is a Rust CLI tool designed to seamlessly integrate and automate
//! [Phased Versioning](https://phased-versioning.koseka.net) within your [Git](https://git-scm.com/) repositories.
//! It acts as a bridge between your project's versioning scheme and Git by mapping generations to branches and automatically
//! managing tags for phases and revisions, ensuring your releases remain perfectly consistent and free of manual errors.
//!
//! # Cargo Features
//!
//! This project has no public
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section).
//!
//! # Installation
//!
//! To use the **latest stable version** of the project, run the following command targeting the `v1` branch in your terminal:
//!
//! ```sh
//! cargo install --git https://github.com/AmonRayfa/oyo --branch v1
//! ```
//!
//! To use the **nightly version**, you can change the branch to `dev`:
//!
//! ```sh
//! cargo install --git https://github.com/AmonRayfa/oyo --branch dev
//! ```
//!
//! You can now start using the `oyo` CLI tool to manage your repository's version branches and tags directly from your
//! terminal.
//!
//! # Usage
//!
//! **Oyo** simplifies version management into three core commands that enforce the rules of
//! [Phased Versioning](https://phased-versioning.koseka.net) automatically:
//!
//! * `oyo gen [-n/--number <generation_number>]`: Creates a new **version branch** representing a **base version** (e.g., `v1`,
//!   `v2`) by incrementing the generation number of the last version branch by `1`. If no version branches currently exist, it
//!   defaults to creating `v1`. You can explicitly specify the generation number using the `-n` or `--number` flag.
//!
//! * `oyo phase <name>`: Creates a new **full version** tag on the latest commit for the active **version branch** (e.g.,
//!   `v1-bravo.0`). It enforces _looped_ alphabetic progression rules and automatically resets the revision to `0`.
//!
//! * `oyo rev`: Creates a new **full version** tag on the latest commit for the active **version branch** by incrementing the
//!   revision number of the last tag by `1` (e.g., from `v1-bravo.0` to `v1-bravo.1`).

mod api;
use api::{Cli, Commands, run_gen, run_phase, run_rev};
use clap::Parser;
use mabe::{Result, bail};

#[mabe::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("dev");
        return Ok(());
    }

    match cli.command {
        Some(Commands::Gen { number }) => run_gen(number),
        Some(Commands::Phase { name }) => run_phase(name),
        Some(Commands::Rev) => run_rev(),
        None => bail!("No subcommand or flag was provided."),
    }
}
