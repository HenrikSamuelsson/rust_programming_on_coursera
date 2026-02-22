# Python and Rust with Linux Command Line Tools

Course 4 of 5 in the Duke University Rust Programming Specialization on Coursera.

This repository contains my notes, labs, and exercises for the course "Python and Rust with Linux Command Line Tools".

The focus of this course is building robust command-line tools using both Python and Rust, with emphasis on:

- Linux-style CLI design
- Argument parsing and validation
- Error handling and logging
- Modularization
- Packaging and distribution
- Best practices using Cargo and tooling

## Course Overview

Building command-line tools is a foundational DevOps and systems skill. This course explores both scripting (Python) and compiled (Rust) approaches to CLI development.

Key topics include:

- Basic CLI tools in Python and Rust
- Argument parsing frameworks (Click, clap)
- Logging, error handling, and panics
- Modularization with Rust modules (`lib.rs`, `main.rs`)
- Cargo best practices (`check`, `clippy`, `fmt`)
- Packaging and distribution strategies

## Learning Objectives

By completing this course, I aim to:

1. Build functional CLI tools in Python and Rust.
2. Design Linux-style tools with flags, options, and subcommands.
3. Apply Rust development best practices.
4. Improve error handling and robustness in CLI programs.
5. Understand distribution tradeoffs between Python and Rust.

Each section corresponds to course modules and practical exercises.

## Tooling

Development environment:

- Windows host (Charcoal)
- VS Code with:
  - Rust Analyzer
  - Python extension
- Rust toolchain installed via `rustup`
- Cargo for dependency management and builds

Common commands used:

```bash
cargo check
cargo build --release
cargo run -- <args>
cargo clippy
cargo fmt
````

## Related Course Repositories

Official examples by Alfredo Deza:

- Python CLI examples: [https://github.com/alfredodeza/python-cli-example](https://github.com/alfredodeza/python-cli-example)
- Rust CLI examples: [https://github.com/alfredodeza/rust-cli-example](https://github.com/alfredodeza/rust-cli-example)
- Advanced Python CLI: [https://github.com/alfredodeza/advanced-python-cli](https://github.com/alfredodeza/advanced-python-cli)
- Advanced Rust CLI: [https://github.com/alfredodeza/advanced-rust-cli](https://github.com/alfredodeza/advanced-rust-cli)

## Notes

This repository is part of my long-term embedded systems development and tooling track. The goal is not only to complete the course, but to integrate Rust CLI tooling practices into professional workflows.
