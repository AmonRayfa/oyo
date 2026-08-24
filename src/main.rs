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
//! * `oyo phase <name> [--dry-run]`: Creates a new **full version** tag on the latest commit for the active **version branch**
//!   (e.g., `v1-bravo.0`). It enforces _looped_ alphabetic progression rules and automatically resets the revision to `0`.
//!
//! * `oyo rev [--dry-run]`: Creates a new **full version** tag on the latest commit for the active **version branch** by
//!   incrementing the revision number of the last tag by `1` (e.g., from `v1-bravo.0` to `v1-bravo.1`).
//!
//! # Version Metadata Synchronization
//!
//! Before creating a tag, the `oyo phase <name>` and `oyo rev` commands synchronize the repository's version metadata, so
//!   that the tag, the manifest, and the source code can never disagree on the version:
//!
//! * The `[package]` version in `Cargo.toml` (and the crate's own entry in `Cargo.lock`) is updated with the
//!   [SemVer projection](https://phased-versioning.koseka.net) of the new version, where MAJOR is the generation number,
//!   MINOR is the cumulative count of phases opened in the generation (starting at `0`, and counting through the `z` -> `a`
//!   wrap of the alphabetic progression), and PATCH is the revision number. For example, if `cyrus` is the third phase of the
//!   `v1` generation, `v1-cyrus.5` projects to `1.2.5`.
//!
//! * Every version string (e.g. `v1-alpha.0`) in the files listed under the `version-files` key of a `.oyo.toml` file at the
//!   root of the repository is updated with the new **full version**. This is how an in-source version constant (e.g.
//!   `const VERSION: &str = "v1-alpha.0";`) is kept in sync: declare the file holding it with
//!   `version-files = ["src/main.rs"]`.
//!
//! The updated files are committed as a `chore: Bump the version to <full_version>.` commit, and the tag is placed on that
//!   commit. If the repository declares no version metadata — or if the metadata already carries the new version — no commit
//!   is created and the tag is placed on the latest commit, as before. The `--dry-run` flag prints the resolved full version
//!   and its SemVer projection without creating a tag or modifying any file.

mod api;
use api::{Cli, Commands, run_gen, run_phase, run_rev};
use clap::Parser;
use mabe::{Result, bail};

#[mabe::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", env!("OYO_VERSION"));
        return Ok(());
    }

    match cli.command {
        Some(Commands::Gen { number }) => run_gen(number),
        Some(Commands::Phase { name, dry_run }) => run_phase(name, dry_run),
        Some(Commands::Rev { dry_run }) => run_rev(dry_run),
        None => bail!("No subcommand or flag was provided."),
    }
}
