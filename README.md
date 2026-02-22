<div align="center">
  <h1 align="center">Oyo</h1>
  <p align="center">
    Contributions, corrections, and requests can be made through GitHub, and the documentation is available <a href="https://oyo.readthedocs.io/en/latest/">here</a>.
  </p>
  <p align="center">Thank you for your interest in the project. Enjoy your reading! 🚀</p>
</div>

<div align="center">
  <a href="https://koseka.net/standards/"><img src="https://img.shields.io/badge/Compliance-Koseka%20Standard-304CD3?style=flat&color=12398D" alt="Koseka Standards" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-723179?style=flat" alt="License" /></a>
  <br>
  <a href="https://github.com/AmonRayfa/oyo/releases"><img src="https://img.shields.io/github/v/tag/AmonRayfa/oyo?label=version&logo=github&color=579D52" alt="version" /></a>
  <a href="https://github.com/AmonRayfa/oyo"><img src="https://img.shields.io/github/created-at/AmonRayfa/oyo?logo=github&label=created&color=C9443C" alt="created" /></a>
  <a href="https://github.com/AmonRayfa/oyo/commits/main"><img src="https://img.shields.io/github/last-commit/AmonRayfa/oyo?display_timestamp=committer&logo=github&color=438240" alt="last commit" /></a>
  <a href="https://github.com/AmonRayfa/oyo/milestones"><img src="https://img.shields.io/github/milestones/all/AmonRayfa/oyo?logo=github&color=5288DF" alt="milestones" /></a>
  <a href="https://github.com/AmonRayfa/oyo/stargazers"><img src="https://img.shields.io/github/stars/AmonRayfa/oyo?style=flat&logo=github&color=DCB456" alt="stars" /></a>
  <br>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Dependencies-3-black?style=flat&logo=rust&logoColor=black" alt="Dependencies" /></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Size-15.9kB-black?style=flat&logo=rust&logoColor=black" alt="Size" /></a>
</div>

---

**Oyo** is a Rust CLI tool designed to seamlessly integrate and automate [Phased Versioning](https://koseka.net/standards/phased-versioning/) within your [Git](https://git-scm.com/) repositories. It acts as a bridge between your project's versioning scheme and Git by mapping generations to branches and automatically managing tags for phases and revisions, ensuring your releases remain perfectly consistent and free of manual errors.

<h2><img height="20" alt="branches" src="./img/branches.svg">&nbsp;&nbsp;Branches</h2>

| Branch | Status      | Description                                              |
| :----- | :---------- | :------------------------------------------------------- |
| `v1`   | **Stable**  | The latest production release.                           |
| `dev`  | **Nightly** | Active development branch. Merged into `v1` when stable. |

**Note for Contributors:** Please submit all feature requests and standard bug fixes to the **`dev`** branch.

<h2><img height="20" alt="installation" src="./img/installation.svg">&nbsp;&nbsp;Installation</h2>

To use the **latest stable version** of the project, run the following command targeting the `v1` branch in your terminal:

```sh
cargo install --git https://github.com/AmonRayfa/oyo --branch v1
```

To use the **nightly version**, you can change the branch to `dev`:

```sh
cargo install --git https://github.com/AmonRayfa/oyo --branch dev
```

You can now start using the `oyo` CLI tool to manage your repository's version branches and tags directly from your terminal.

<h2><img height="20" alt="usage" src="./img/usage.svg">&nbsp;&nbsp;Usage</h2>

**Oyo** simplifies version management into three core commands that enforce the rules of [Phased Versioning](https://koseka.net/standards/phased-versioning/) automatically:

- `oyo gen [-n/--number <generation_number>]`: Creates a new **version branch** representing a **base version** (e.g., `v1`, `v2`) by incrementing the generation number of the last version branch by `1`. If no version branches currently exist, it defaults to creating `v1`. You can explicitly specify the generation number using the `-n` or `--number` flag.

- `oyo phase <name>`: Creates a new **full version** tag on the latest commit for the active **version branch** (e.g., `v1-bravo.0`). It enforces _looped_ alphabetic progression rules and automatically resets the revision to `0`.

- `oyo rev`: Creates a new **full version** tag on the latest commit for the active **version branch** by incrementing the revision number of the last tag by `1` (e.g., from `v1-bravo.0` to `v1-bravo.1`).

For further details on how to use the project, please refer to the [documentation](https://oyo.readthedocs.io).

<h2><img height="20" alt="security" src="./img/security.svg">&nbsp;&nbsp;Security</h2>

Vulnerabilities and sensitive information should not be reported via public GitHub issues. Please refer to the [Security Policy](SECURITY.md) for details on supported versions and instructions on how to responsibly disclose security concerns.

<h2><img height="20" alt="contributing" src="./img/contributing.svg">&nbsp;&nbsp;Contributing</h2>

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of conduct, but please be respectful and considerate when engaging with the community.

This project adheres to the [Koseka Standards](https://koseka.net/standards/), which provides standardized versioning and contribution rules. So, make sure to read it first before contributing to the project in any way. Additionally, please refer to the [Contribution Guide](CONTRIBUTING.md) for setup instructions and guidance on how to contribute the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as bellow, without any additional terms or conditions.

<h2><img height="20" alt="license" src="./img/license.svg">&nbsp;&nbsp;License</h2>

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
