# AGENTS.md - SupGIT Project Guidelines

## Project Overview

SupGIT (Simple Git) is a Rust CLI wrapper around Git that provides simplified workflows for common Git operations. It is a single-binary project with a modular code structure.

## Build/Lint/Test Commands

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the binary directly
cargo run -- <command>

# Check for compilation errors (faster than build)
cargo check

# Run all tests
cargo test

# Run a specific test by name
cargo test <test_name>

# Run a specific test file (for integration tests)
cargo test --test <test_file>

# Run tests with output shown
cargo test -- --nocapture

# Run a single test with pattern matching
cargo test <pattern>

# Lint with clippy
cargo clippy

# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check
```

## Code Style Guidelines

### Imports

- Group imports logically: standard library first, then external crates, then local modules
- Use `use` statements at the top of the file
- Import specific items rather than glob imports (`use anyhow::{bail, Context, Result}` not `use anyhow::*`)
- Rename imports to avoid conflicts (e.g., `use std::process::Command as StdCommand`)

```rust
use std::process::Command as StdCommand;

use anyhow::{bail, Context, Result};
use dialoguer::{Confirm, Input, Select};

use crate::git::run_git_silent;
use crate::status::get_repo_root;
```

### Formatting

- Use 4 spaces for indentation (Rust standard)
- Maximum line length: 100 characters
- Place opening braces on the same line
- Use `cargo fmt` to auto-format before committing

### Types

- Use `Result<T>` as the return type for fallible functions (aliased from `anyhow::Result`)
- Use `Option<T>` for optional values
- Prefer `String` for owned strings, `&str` for borrowed
- Use `Vec<T>` for collections

### Naming Conventions

- **Functions**: snake_case (`run_git`, `get_staged_files`, `stage_targets`)
- **Structs/Enums**: PascalCase (`Cli`, `SupgitCommand`)
- **Constants**: SCREAMING_SNAKE_CASE (`NOT_IN_REPO_HINT`, `NO_STAGED_HINT`)
- **Variables**: snake_case
- **Enum variants**: PascalCase (`SupgitCommand::Init`, `SupgitCommand::Stage`)

### Error Handling

- Use `anyhow` crate for error handling
- Use `bail!` macro for early returns with an error message
- Use `.context()` or `.with_context()` to add context to operations that may fail

```rust
pub fn check_in_repo() -> Result<()> {
    StdCommand::new("git")
        .args(["rev-parse", "--git-dir"])
        .status()
        .context("failed to execute git - is git installed?")?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("{}", NOT_IN_REPO_HINT))
}
```

### CLI Structure

- Use `clap` derive macros (`#[derive(Parser, Subcommand)]`)
- Document commands and arguments with doc comments (`///`)
- Use `#[arg(long)]` for long flags, `#[arg(short, long)]` for both
- Use `#[command(subcommand)]` for subcommands

```rust
#[derive(Parser)]
#[command(name = "supgit", about = "Description", version)]
pub struct Cli {
    #[arg(long, global = true)]
    pub explain: bool,

    #[command(subcommand)]
    pub command: Option<SupgitCommand>,
}
```

### Functions

- Keep functions focused on a single responsibility
- Use `-> Result<()>` for functions that may fail
- Pattern match on `is_interactive` to handle CLI vs interactive modes
- Extract helper functions for repeated logic

### Pattern Matching

- Use `match` for enum dispatch
- Handle all variants explicitly
- Use `_ => Ok(())` for no-op default cases when appropriate

### Command Execution

- Use `std::process::Command` for running Git commands
- Check `output.status.success()` before returning `Ok(())`
- Use `.output()` when you need to capture stdout/stderr

```rust
let output = StdCommand::new("git")
    .args(["status", "--porcelain"])
    .output()
    .context("running git status --porcelain")?;
```

### String Handling

- Use `String::from_utf8_lossy()` for converting command output
- Use `.trim()` to clean up output strings
- Use `.lines()` for iterating over multi-line output
- Use `.as_str()` to convert `String` to `&str` when needed

## Architecture Notes

- Modular architecture under `src/`:
  - `main.rs` - Entry point, command dispatch
  - `cli.rs` - CLI definitions (Cli struct, SupgitCommand enum)
  - `git.rs` - Git command execution helpers (`run_git`, `run_git_silent`, etc.)
  - `status.rs` - Repository status utilities (`get_staged_files`, `get_branches`, etc.)
  - `commands/` - Individual command implementations
- Entry point is `fn main()` which calls `run()` and handles errors
- All Git operations use helpers from `git.rs` module
- Interactive prompts use the `dialoguer` crate
- User-facing errors are printed via `eprintln!` with error chain

## Adding New Commands

1. Add a new variant to `SupgitCommand` enum in `src/cli.rs` with doc comment
2. Add any required arguments as fields with `#[arg]` attributes
3. Create a new file in `src/commands/` for the implementation
4. Export the function from `src/commands/mod.rs`
5. Add a match arm in `main.rs` to call the new command function
6. Update `print_explanations()` in `main.rs` to document the new command

## Dependencies

- `anyhow`: Error handling
- `clap`: CLI argument parsing (derive feature enabled)
- `dialoguer`: Interactive prompts (Select, Input, Confirm, MultiSelect)