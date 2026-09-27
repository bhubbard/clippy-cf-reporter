# Clippy CF Reporter (`clippy-cf-reporter`)

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://img.shields.io/badge/CI-passing-brightgreen.svg)](https://github.com/bhubbard/clippy-cf-reporter/actions)
[![Live Demo](https://img.shields.io/badge/Live%20Demo-code.brandonhubbard.com-brightgreen?logo=github)](https://code.brandonhubbard.com/clippy-cf-reporter/)

> Translates `worker-rs` and Rust Clippy diagnostics into Code Climate JSON, SARIF v2.1.0, and GitHub Actions PR annotations.

> 🎮 **Live Interactive Visualizer & Demo:** [clippy-cf-reporter on code.brandonhubbard.com](https://code.brandonhubbard.com/clippy-cf-reporter/)

---

## ⚡ Overview

`clippy-cf-reporter` is a high-performance Rust CLI tool and library engineered for Cloudflare Workers, Cloudflare Pages, and Astro developers. Built with `cargo clippy`, `serde`, `serde_json`, `clap`, it provides instant feedback, CI compliance enforcement, and seamless developer workflows.

---

## ✨ Features

- **Multi-Format Ingestion:** Ingests `cargo clippy --message-format=json` compiler output via stdin or file.
- **Code Climate JSON:** Emits GitLab CI and Code Climate compatible issue payloads with fingerprint hashing.
- **SARIF v2.1.0:** Generates OASIS SARIF v2.1.0 reports for native GitHub Code Scanning integration.
- **GitHub Workflow Annotations:** Formats workflow commands (`::warning file=...::...` and `::error file=...::...`) for real-time PR review comments.
- **Wasm & Cargo Filtering:** Automatically filters out third-party dependency noise in `.cargo/registry` and `wasm-bindgen` generated wrappers.

---

## 📦 Installation

### From Source (Cargo)
```bash
cargo install --git https://github.com/bhubbard/clippy-cf-reporter
```

### Pre-built Binary (Release)
Clone the repository and build in release mode:
```bash
git clone https://github.com/bhubbard/clippy-cf-reporter.git
cd clippy-cf-reporter
cargo build --release
./target/release/clippy-cf-reporter --help
```

---

## 🚀 Usage

```bash
# Pipe cargo clippy directly into GitHub annotations in CI
cargo clippy --message-format=json | clippy-cf-reporter --format github

# Generate SARIF report for GitHub Code Scanning
cargo clippy --message-format=json | clippy-cf-reporter --format sarif --output clippy-results.sarif

# Generate Code Climate report for GitLab CI
cargo clippy --message-format=json | clippy-cf-reporter --format codeclimate --output codeclimate.json
```

---

## 🛠️ Configuration & CI Integration

### GitHub Actions Workflow Example
```yaml
name: clippy-cf-reporter Audit

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install clippy-cf-reporter
        run: cargo install --git https://github.com/bhubbard/clippy-cf-reporter
      - name: Run Audit
        run: clippy-cf-reporter --check
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 Brandon Hubbard.
