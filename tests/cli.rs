// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! Integration tests for the CLI commands. Each test runs the compiled `oyo` binary against an isolated scratch
//! [Git](https://git-scm.com/) repository, so the tests are independent of the host's Git configuration and can run on any
//! platform of the CI pipeline.

use std::process::{Command, Output};
use tempfile::TempDir;

/// An isolated scratch Git repository for exercising the CLI.
struct TestRepo {
    dir: TempDir,
}

impl TestRepo {
    /// Creates a Git repository with a `dev` branch and an initial commit.
    fn new() -> Self {
        let repo = Self::empty();
        repo.git(&["commit", "--allow-empty", "-m", "chore: Init."]);
        repo
    }

    /// Creates a Git repository with a `dev` branch and no commits.
    fn empty() -> Self {
        let repo = Self { dir: TempDir::new().unwrap() };
        repo.git(&["init", "-b", "dev"]);
        repo
    }

    /// Runs a Git command in the repository, asserting success.
    fn git(&self, args: &[&str]) -> String {
        let output = self.command("git").args(args).output().unwrap();
        assert!(output.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    /// Runs the `oyo` binary in the repository.
    fn oyo(&self, args: &[&str]) -> Output {
        self.command(env!("CARGO_BIN_EXE_oyo")).args(args).output().unwrap()
    }

    /// Creates an empty commit in the repository.
    fn commit(&self) {
        self.git(&["commit", "--allow-empty", "-m", "chore: Change."]);
    }

    /// Returns the name of the currently checked-out branch.
    fn current_branch(&self) -> String {
        self.git(&["branch", "--show-current"])
    }

    /// Returns the list of tags in the repository.
    fn tags(&self) -> String {
        self.git(&["tag", "--list"])
    }

    /// Writes a file in the repository.
    fn write(&self, file: &str, content: &str) {
        let path = self.dir.path().join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// Reads a file from the repository.
    fn read(&self, file: &str) -> String {
        std::fs::read_to_string(self.dir.path().join(file)).unwrap()
    }

    /// Stages and commits all the changes in the repository.
    fn commit_all(&self, message: &str) {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-m", message]);
    }

    /// Returns the subject line of the last commit.
    fn head_subject(&self) -> String {
        self.git(&["log", "-1", "--pretty=%s"])
    }

    /// Populates the repository with version metadata (a Cargo manifest, a lockfile, a version constant, and a `.oyo.toml`).
    fn write_version_metadata(&self) {
        self.write(
            "Cargo.toml",
            "[package]\nname = \"demo\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies.serde]\nversion = \"1.0.219\"\n",
        );
        self.write(
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"demo\"\nversion = \"0.0.0\"\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.219\"\n",
        );
        self.write("src/main.rs", "const VERSION: &str = \"v0-zeta.9\";\n");
        self.write(".oyo.toml", "version-files = [\"src/main.rs\"]\n");
        self.commit_all("chore: Add version metadata.");
    }

    /// Prepares a command that runs in the repository, shielded from the host's Git configuration.
    fn command(&self, program: &str) -> Command {
        let mut command = Command::new(program);
        command
            .current_dir(self.dir.path())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.dir.path().join("global-gitconfig"))
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com");
        command
    }
}

/// Asserts that a command failed gracefully (i.e. with an error, not a panic).
fn assert_graceful_failure(output: &Output) {
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(!output.status.success(), "the command was expected to fail");
    assert!(!stderr.contains("panicked"), "the command panicked: {}", stderr);
}

#[test]
fn gen_creates_the_first_version_branch() {
    let repo = TestRepo::new();
    assert!(repo.oyo(&["gen"]).status.success());
    assert_eq!(repo.current_branch(), "v1");
}

#[test]
fn gen_increments_the_highest_local_generation() {
    let repo = TestRepo::new();
    repo.git(&["branch", "v1"]);
    repo.git(&["branch", "v4"]);
    assert!(repo.oyo(&["gen"]).status.success());
    assert_eq!(repo.current_branch(), "v5");
}

#[test]
fn gen_includes_remote_version_branches() {
    let repo = TestRepo::new();
    repo.git(&["update-ref", "refs/remotes/origin/v3", "HEAD"]);
    assert!(repo.oyo(&["gen"]).status.success());
    assert_eq!(repo.current_branch(), "v4");
}

#[test]
fn gen_accepts_an_explicit_generation_number() {
    let repo = TestRepo::new();
    assert!(repo.oyo(&["gen", "-n", "7"]).status.success());
    assert_eq!(repo.current_branch(), "v7");
}

#[test]
fn gen_fails_when_the_branch_already_exists() {
    let repo = TestRepo::new();
    repo.git(&["branch", "v1"]);
    assert_graceful_failure(&repo.oyo(&["gen", "-n", "1"]));
}

#[test]
fn gen_fails_in_an_uninitialized_repository() {
    let repo = TestRepo::empty();
    assert_graceful_failure(&repo.oyo(&["gen"]));
}

#[test]
fn phase_initializes_the_first_phase_with_an_annotated_tag() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    assert!(repo.oyo(&["phase", "alameda"]).status.success());
    assert_eq!(repo.tags(), "v1-alameda.0");
    assert_eq!(repo.git(&["cat-file", "-t", "v1-alameda.0"]), "tag");
}

#[test]
fn phase_requires_the_first_phase_to_start_with_a() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    assert_graceful_failure(&repo.oyo(&["phase", "bravo"]));
    assert_eq!(repo.tags(), "");
}

#[test]
fn phase_rejects_empty_names() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    assert_graceful_failure(&repo.oyo(&["phase", ""]));

    // The empty name must also be rejected when prior version tags exist.
    repo.oyo(&["phase", "alameda"]);
    repo.commit();
    assert_graceful_failure(&repo.oyo(&["phase", ""]));
}

#[test]
fn phase_rejects_invalid_characters() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    assert_graceful_failure(&repo.oyo(&["phase", "Alameda"]));
    assert_graceful_failure(&repo.oyo(&["phase", "alameda1"]));
    assert_eq!(repo.tags(), "");
}

#[test]
fn phase_enforces_alphabetic_progression() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.oyo(&["phase", "alameda"]);
    repo.commit();
    assert_graceful_failure(&repo.oyo(&["phase", "cyrus"]));
    assert!(repo.oyo(&["phase", "bravo"]).status.success());
    assert_eq!(repo.tags(), "v1-alameda.0\nv1-bravo.0");
}

#[test]
fn phase_wraps_the_progression_after_z() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.git(&["tag", "v1-zulu.0"]);
    repo.commit();
    assert_graceful_failure(&repo.oyo(&["phase", "bravo"]));
    assert!(repo.oyo(&["phase", "apple"]).status.success());
}

#[test]
fn phase_and_rev_refuse_an_already_tagged_commit() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.oyo(&["phase", "alameda"]);
    assert_graceful_failure(&repo.oyo(&["phase", "bravo"]));
    assert_graceful_failure(&repo.oyo(&["rev"]));
    assert_eq!(repo.tags(), "v1-alameda.0");
}

#[test]
fn phase_and_rev_require_a_version_branch() {
    let repo = TestRepo::new();
    assert_graceful_failure(&repo.oyo(&["phase", "alameda"]));
    assert_graceful_failure(&repo.oyo(&["rev"]));
}

#[test]
fn rev_bumps_the_revision_with_an_annotated_tag() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.oyo(&["phase", "alameda"]);
    repo.commit();
    assert!(repo.oyo(&["rev"]).status.success());
    assert_eq!(repo.tags(), "v1-alameda.0\nv1-alameda.1");
    assert_eq!(repo.git(&["cat-file", "-t", "v1-alameda.1"]), "tag");
}

#[test]
fn rev_requires_an_initialized_phase() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    assert_graceful_failure(&repo.oyo(&["rev"]));
}

#[test]
fn no_arguments_prints_the_usage_and_fails() {
    let repo = TestRepo::new();
    let output = repo.oyo(&[]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Usage"));
}

#[test]
fn version_flag_prints_a_version_string() {
    let repo = TestRepo::new();
    let output = repo.oyo(&["-v"]);
    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn phase_syncs_the_version_metadata_before_tagging() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.write_version_metadata();
    assert!(repo.oyo(&["phase", "alameda"]).status.success());

    // The metadata carries the new versions, and the tag sits on the bump commit.
    assert!(repo.read("Cargo.toml").contains("version = \"1.0.0\""));
    assert!(repo.read("Cargo.lock").contains("name = \"demo\"\nversion = \"1.0.0\""));
    assert!(repo.read("src/main.rs").contains("\"v1-alameda.0\""));
    assert_eq!(repo.head_subject(), "chore: Bump the version to v1-alameda.0.");
    assert_eq!(repo.git(&["rev-parse", "v1-alameda.0^{commit}"]), repo.git(&["rev-parse", "HEAD"]));

    // The version fields of the dependencies are left untouched.
    assert!(repo.read("Cargo.toml").contains("version = \"1.0.219\""));
    assert!(repo.read("Cargo.lock").contains("name = \"serde\"\nversion = \"1.0.219\""));
}

#[test]
fn rev_projects_the_cumulative_minor_and_patch() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.write_version_metadata();
    repo.oyo(&["phase", "apple"]);
    repo.commit();
    repo.oyo(&["phase", "bravo"]);
    repo.commit();
    assert!(repo.oyo(&["rev"]).status.success());
    assert!(repo.read("Cargo.toml").contains("version = \"1.1.1\""));
    assert!(repo.read("src/main.rs").contains("\"v1-bravo.1\""));
}

#[test]
fn phase_minor_keeps_counting_through_the_wrap() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.git(&["tag", "v1-apple.0"]);
    repo.git(&["tag", "v1-zulu.3"]);
    repo.commit();
    repo.write_version_metadata();
    assert!(repo.oyo(&["phase", "arrow"]).status.success());

    // The third phase wraps back to 'a', but its minor is the cumulative count (2), not the alphabet index (0).
    assert!(repo.read("Cargo.toml").contains("version = \"1.2.0\""));
}

#[test]
fn dry_run_reports_the_versions_without_side_effects() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.write_version_metadata();
    let output = repo.oyo(&["phase", "alameda", "--dry-run"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(stdout.contains("v1-alameda.0") && stdout.contains("1.0.0"), "unexpected dry run output: {}", stdout);

    // Nothing was tagged, committed, or modified.
    assert_eq!(repo.tags(), "");
    assert_eq!(repo.head_subject(), "chore: Add version metadata.");
    assert!(repo.read("Cargo.toml").contains("version = \"0.0.0\""));
}

#[test]
fn phase_tags_head_directly_when_the_metadata_is_already_synced() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.write_version_metadata();
    repo.write("Cargo.toml", "[package]\nname = \"demo\"\nversion = \"1.0.0\"\n");
    repo.write("Cargo.lock", "version = 4\n\n[[package]]\nname = \"demo\"\nversion = \"1.0.0\"\n");
    repo.write("src/main.rs", "const VERSION: &str = \"v1-alameda.0\";\n");
    repo.commit_all("chore: Bump the version manually.");
    assert!(repo.oyo(&["phase", "alameda"]).status.success());

    // No bump commit was created: the tag sits on the last manual commit.
    assert_eq!(repo.head_subject(), "chore: Bump the version manually.");
    assert_eq!(repo.git(&["rev-parse", "v1-alameda.0^{commit}"]), repo.git(&["rev-parse", "HEAD"]));
}

#[test]
fn phase_fails_when_a_listed_version_file_is_invalid() {
    let repo = TestRepo::new();
    repo.oyo(&["gen"]);
    repo.write(".oyo.toml", "version-files = [\"missing.rs\"]\n");
    repo.commit_all("chore: Add a broken config.");
    assert_graceful_failure(&repo.oyo(&["phase", "alameda"]));

    repo.write("empty.rs", "const NOT_A_VERSION: u64 = 0;\n");
    repo.write(".oyo.toml", "version-files = [\"empty.rs\"]\n");
    repo.commit_all("chore: Point the config at a file without a version.");
    assert_graceful_failure(&repo.oyo(&["phase", "alameda"]));
    assert_eq!(repo.tags(), "");
}
