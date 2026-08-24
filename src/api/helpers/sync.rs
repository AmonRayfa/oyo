// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains helper functions for synchronizing a repository's version metadata (the `[package]` version in
//! `Cargo.toml`, its `Cargo.lock` entry, and the files listed in `.oyo.toml`) with the tags created by the CLI.

use super::git;
use mabe::{Result, bail};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Counts the distinct phases already tagged on the current version branch. The count is the cumulative phase index used as
/// the MINOR number of the SemVer projection, so it keeps growing through the z -> a wrap of the alphabetic progression.
pub(crate) fn count_distinct_phases(tags: &[String], tag_pattern: &Regex) -> usize {
    tags.iter().filter_map(|tag| tag_pattern.captures(tag)).map(|cap| cap[1].to_string()).collect::<HashSet<String>>().len()
}

/// Synchronizes the repository's version metadata with the provided full version and SemVer projection, and commits the
/// updated files so that the release tag can be placed on the commit carrying the right metadata. Returns without doing
/// anything when the repository declares no metadata (no `Cargo.toml` with a `[package]` version and no `.oyo.toml`), or when
/// the metadata is already up to date.
pub(crate) fn sync_version_metadata(full_version: &str, semver: &str) -> Result<()> {
    let root = git(&["rev-parse", "--show-toplevel"])?;
    let root = Path::new(&root);
    let mut updated: Vec<String> = Vec::new();

    if let Some(crate_name) = sync_cargo_manifest(root, semver, &mut updated)? {
        sync_cargo_lockfile(root, &crate_name, semver, &mut updated)?;
    }

    sync_version_files(root, full_version, &mut updated)?;

    if !updated.is_empty() {
        let message = format!("chore: Bump the version to {}.", full_version);
        let paths: Vec<String> = updated.iter().map(|file| root.join(file).to_string_lossy().into_owned()).collect();
        let mut args = vec!["commit", "-m", &message, "--"];
        args.extend(paths.iter().map(String::as_str));
        git(&args)?;
        println!("🧬 Version metadata synchronized: {}.", updated.join(", "));
    }

    Ok(())
}

/// Updates the `[package]` version in `Cargo.toml` with the SemVer projection, if the repository has one. Returns the name of
/// the crate when the manifest was updated, so that its `Cargo.lock` entry can be updated as well.
fn sync_cargo_manifest(root: &Path, semver: &str, updated: &mut Vec<String>) -> Result<Option<String>> {
    let path = root.join("Cargo.toml");
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)?;

    // Isolates the `[package]` section (i.e. everything until the next section header).
    let section_pattern = Regex::new(r"(?m)^\[package\][ \t]*\r?$").unwrap();
    let Some(section_start) = section_pattern.find(&content).map(|m| m.end()) else {
        return Ok(None);
    };
    let section_end =
        Regex::new(r"(?m)^\[").unwrap().find_at(&content, section_start).map(|m| m.start()).unwrap_or(content.len());
    let section = &content[section_start..section_end];

    let version_pattern = Regex::new(r#"(?m)^(version[ \t]*=[ \t]*")[^"]*(")"#).unwrap();
    let name_pattern = Regex::new(r#"(?m)^name[ \t]*=[ \t]*"([^"]+)""#).unwrap();
    let (Some(_), Some(name_cap)) = (version_pattern.find(section), name_pattern.captures(section)) else {
        return Ok(None);
    };
    let crate_name = name_cap[1].to_string();

    let new_section = version_pattern.replace(section, format!("${{1}}{}${{2}}", semver)).into_owned();
    if new_section != section {
        fs::write(&path, format!("{}{}{}", &content[..section_start], new_section, &content[section_end..]))?;
        updated.push(String::from("Cargo.toml"));
    }

    Ok(Some(crate_name))
}

/// Updates the crate's own entry in `Cargo.lock` with the SemVer projection, if the repository has one.
fn sync_cargo_lockfile(root: &Path, crate_name: &str, semver: &str, updated: &mut Vec<String>) -> Result<()> {
    let path = root.join("Cargo.lock");
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let entry_pattern =
        Regex::new(&format!(r#"(?m)^(name = "{}"\r?\nversion = ")[^"]*(")"#, regex::escape(crate_name))).unwrap();

    let new_content = entry_pattern.replace(&content, format!("${{1}}{}${{2}}", semver)).into_owned();
    if new_content != content {
        fs::write(&path, new_content)?;
        updated.push(String::from("Cargo.lock"));
    }

    Ok(())
}

/// Updates every Phased Versioning version string (e.g. `v1-alpha.0`) with the full version in the files listed under the
/// `version-files` key of `.oyo.toml`, if the repository has one.
fn sync_version_files(root: &Path, full_version: &str, updated: &mut Vec<String>) -> Result<()> {
    let config_path = root.join(".oyo.toml");
    if !config_path.exists() {
        return Ok(());
    }

    let config = fs::read_to_string(&config_path)?;
    let array_pattern = Regex::new(r"(?s)version-files[ \t]*=[ \t]*\[(.*?)\]").unwrap();
    let Some(array_cap) = array_pattern.captures(&config) else {
        return Ok(());
    };
    let file_pattern = Regex::new(r#""([^"]+)""#).unwrap();
    let version_pattern = Regex::new(r"\bv(0|[1-9]\d*)-[a-z]+\.(0|[1-9]\d*)").unwrap();

    for file_cap in file_pattern.captures_iter(&array_cap[1]) {
        let file = file_cap[1].to_string();
        let path = root.join(&file);
        if !path.exists() {
            bail!("The version-files key of .oyo.toml lists '{}', but the file doesn't exist.", file);
        }

        let content = fs::read_to_string(&path)?;
        if !version_pattern.is_match(&content) {
            bail!("The version-files key of .oyo.toml lists '{}', but the file contains no version string to update.", file);
        }

        let new_content = version_pattern.replace_all(&content, full_version).into_owned();
        if new_content != content {
            fs::write(&path, new_content)?;
            updated.push(file);
        }
    }

    Ok(())
}
