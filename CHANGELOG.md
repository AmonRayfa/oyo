# Changelog

This project is licensed under the [Apache License (Version 2.0)](LICENSE). The format of this file (and the project as a whole) follows [Phased Versioning](https://phased-versioning.koseka.net).

## v1-alpha.3 (2026-08-24)

- Fixed the `phase` command to reject empty phase names instead of crashing.
- Fixed the CLI to print a usage error and exit with a non-zero code when no arguments are provided.
- Fixed the `gen` command to account for remote version branches when calculating the generation number.
- The `phase` and `rev` commands now create annotated tags.
- The `-v/--version` flag now derives the version string from the state of the Git repository at build time.
- Added integration tests for the CLI commands.

## v1-alpha.2 (2026-02-27)

- Improved the clarity of error messages.

## v1-alpha.2 (2026-02-27)

- Improved the clarity of error messages.

## v1-alpha.1 (2026-02-26)

- Refined the progress indicators.
- Added a `-v/--version` flag to the CLI.

## v1-alpha.0 (2026-02-21)

First stable version.
