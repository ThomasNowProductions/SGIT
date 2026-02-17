# SupGIT

SupGIT (Simple Git) is a lightning-fast Rust wrapper around the Git CLI that exposes streamlined workflows for initializing repositories, staging files, creating commits, and pushing in a few simple commands.

## Building

```sh
cd supgit
cargo build --release
```

Copy `target/release/supgit` into your `PATH`, or run it via `cargo run --bin supgit -- <command>`.

## Usage

```
supgit <command> [options]
```

### Simplified commands

- `supgit init` — run `git init`
- `supgit stage [path ...]` — add files (defaults to `.`)
- `supgit unstage [path ...]` — drop files from the staging area (`git restore --staged`)
- `supgit commit -m "message" [--all | --unstaged | --staged] [--push] [--amend]` — create commits with helpers to stage tracked/unstaged changes and optionally push immediately
- `supgit status [--short]` — show `git status` (`-sb` with `--short`)
- `supgit log [--short]` — compact or detailed log
- `supgit diff [path] [--staged]` — diff working tree (or staged snapshot)
- `supgit branch` — list local branches
- `supgit push [remote] [branch]` — push with the same defaults as `git push`, but allow overriding remote/branch if you need to force a specific ref
- `supgit pull [remote] [branch]` — pull with optional remote/branch

When using `--push`, SupGIT now runs `git push` without hard-coding `origin`, so your repository’s configured upstream and `push.default` still take precedence. `--all` stages tracked and untracked files before committing, `--unstaged` stages tracked-but-uncommitted changes, and the plain commit command commits only what you already staged.

`supgit status` accepts `--short` to show the compact `git status -sb` view, and `supgit push` respects the default `git push` behavior (add `remote`/`branch` only if you explicitly pass them).

Set `--explain` on any `supgit` invocation (even without a subcommand) to print a friendly “noob explanation” of each command and its common options instead of running the command you normally would.

## Local installation

Use the provided scripts to install or remove the binary:

```
./install.sh       # builds SupGIT and copies it to $HOME/.local/bin (or $SupGIT_INSTALL_DIR)
./uninstall.sh     # deletes the installed binary from the same location
```

Set the `SupGIT_INSTALL_DIR` environment variable before running `install.sh`/`uninstall.sh` if you prefer installing somewhere else in your PATH. Re-running `install.sh` rebuilds and updates the binary.
