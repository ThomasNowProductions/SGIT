  ---
name: supgit
description: |
  SupGIT is a Rust CLI wrapper around Git that provides simplified workflows for common Git operations.
  Use this skill whenever you need to perform Git operations (commit, push, pull, branch, stage, status, etc.)
  and the user wants a simpler, more streamlined experience. This skill triggers for any Git-related task
  where SupGIT could simplify the workflow.
---

# SupGIT Skill

SupGIT is a globally installed CLI tool that wraps Git with simplified workflows. This skill allows you to use SupGIT commands to accomplish Git tasks more efficiently.

## When to Use SupGIT

Use this skill when:
- The user asks to perform Git operations (commit, push, pull, branch, stage, etc.)
- You need to check repository status
- The user wants a simpler Git experience
- Working with a Git repository and you want more streamlined commands

## Running SupGIT Commands

Since SupGIT is globally installed, use:

```bash
supgit <command>
```

## Available Commands

### Init
Initialize a new Git repository with sensible defaults.

```bash
supgit init
```

### Stage
Stage files for commit. Supports multiple targeting modes:

```bash
# Stage specific files
supgit stage path/to/file.rs

# Stage all modified files (including untracked)
supgit stage --all

# Stage only tracked (modified) files
supgit stage --tracked
```

### Unstage
Unstage files from the index:

```bash
# Unstage specific files
supgit unstage path/to/file.rs

# Unstage all staged files
supgit unstage --all
```

### Status
Check repository status. Use `--short` for concise output:

```bash
supgit status
supgit status --short
```

### Commit
Create commits with various options:

```bash
# Commit with message (will prompt if not provided)
supgit commit -m "Your commit message"

# Stage all tracked files and commit
supgit commit --all -m "Message"

# Commit only staged files (default behavior)
supgit commit --staged -m "Message"

# Commit and push in one command
supgit commit --push -m "Message"

# Amend the last commit
supgit commit --amend -m "New message"

# Skip hooks
supgit commit --no-verify -m "Message"
```

### Log
View commit history:

```bash
supgit log
supgit log --short  # Concise format
```

### Diff
View changes:

```bash
supgit diff              # unstaged changes
supgit diff --staged    # staged changes
supgit diff path/to/file # specific file
```

### Reset
Reset changes with flexible targeting:

```bash
# Reset staged changes
supgit reset --staged

# Reset all staged and unstaged
supgit reset --all

# Reset tracked files
supgit reset --tracked

# Reset untracked files
supgit reset --untracked
```

### Branch
Manage branches:

```bash
# Create a new branch
supgit branch -c feature/my-feature

# Delete a branch
supgit branch -d old-feature
```

### Push
Push to remote:

```bash
supgit push                    # push current branch
supgit push origin main       # push to specific remote/branch
```

### Pull
Pull from remote:

```bash
supgit pull                   # pull current branch
supgit pull origin main       # pull from specific remote/branch
```

### Sync
Pull + push in one command:

```bash
supgit sync                   # sync current branch
supgit sync origin main       # sync specific remote/branch
```

### Clone
Clone a repository:

```bash
supgit clone https://github.com/user/repo
supgit clone https://github.com/user/repo my-directory
```

### Update
Update SupGIT to the latest version:

```bash
supgit update
```

### Alias / Unalias
Manage shell aliases:

```bash
# Add supgit alias to shell
supgit alias

# Add supgit + sg aliases
supgit alias --sg

# Preview without applying
supgit alias --dry-run

# Remove aliases
supgit unalias
supgit unalias --sg
```

## Global Options

- `--explain`: Add explanations to the output, showing what each command does behind the scenes

## Error Handling

If a command fails:
1. Read the error message carefully - SupGIT provides helpful hints
2. Check if you're in a Git repository (`git status` or `supgit status`)
3. For staged/unstaged issues, use `supgit status` to see what's happening
4. Some commands require interactive input - handle prompts appropriately

## Best Practices

1. Always check status before committing: `supgit status`
2. Use `supgit status --short` for quick overview in scripts
3. Use `--explain` when you want to understand what SupGIT is doing internally
4. Prefer `supgit commit --all` to stage and commit in one step when you've reviewed all changes
5. Use `supgit sync` instead of separate pull/push for quick updates
