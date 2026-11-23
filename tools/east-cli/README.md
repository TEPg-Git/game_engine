# East CLI

A Rust command-line developer toolkit for [East Engine](https://github.com/TEPg-Git/game_engine).

East CLI provides project inspection, structure validation, environment diagnostics, and convenient Cargo build/run commands.

## Requirements

- Rust toolchain (rustc and Cargo)
- A checkout of East Engine

## Build

From this directory:

```bash
cargo build --release
```

The executable is `target/release/east` (or `east.exe` on Windows).

## Usage

Run commands from anywhere inside the East Engine repository; East CLI searches parent directories for the engine root.

```bash
east --help
east info
east check
east doctor
east run
east run --release
east build
east build --release
east clean
```

You can also run it during development with Cargo:

```bash
cargo run --manifest-path tools/east-cli/Cargo.toml -- info
cargo run --manifest-path tools/east-cli/Cargo.toml -- check
cargo run --manifest-path tools/east-cli/Cargo.toml -- doctor
```

## Commands

| Command | Description |
| --- | --- |
| `info` | Print package version, project location, and technology details |
| `check` | Validate required engine source files, shader, and asset directories |
| `doctor` | Check Rust/Cargo availability and key project prerequisites |
| `run` | Launch the engine through Cargo |
| `run --release` | Launch an optimized build |
| `build` | Compile the engine in debug mode |
| `build --release` | Compile an optimized release build |
| `clean` | Remove Cargo-generated build artifacts |

## Design

- Uses Clap derive for typed subcommands and generated help.
- Finds the repository root by walking up from the current working directory.
- Uses process exit codes so commands can be composed in scripts and CI.
- Delegates compilation and execution to Cargo rather than duplicating Rust build logic.

## Tests

```bash
cargo test --manifest-path tools/east-cli/Cargo.toml
```

## Current scope

This is an initial developer-tooling milestone. It does not yet package distributable game builds or inspect runtime GPU capabilities.
