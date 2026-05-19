# degit (Rust Port)

A Rust port of the popular `degit` CLI tool originally created by Rich Harris. It makes copies of git repositories without downloading the entire git history – effectively letting you use existing repositories as project templates.

## Features

- **Blazing Fast**: Directly downloads tarballs instead of cloning the entire git repository.
- **Provider Support**: Works out of the box with GitHub, GitLab, and Bitbucket.
- **Offline Caching**: Reuses previously downloaded templates via local disk caching to speed up subsequent scaffolding operations.
- **Branch/Tag/Commit Specific**: Target specific branches, tags, or even exact git commits.
- **Git Fallback**: Supports falling back to a standard `git clone` mechanism via the `--mode git` configuration.

## Installation

You need to have [Rust](https://rustup.rs/) installed. To compile and install the CLI tool globally:

```bash
git clone git@github.com:srttk degit-rs
cd degit-rs
cargo install --path .
```

## Usage

The syntax mostly matches the original Node.js version of `degit`:

```bash
degit <source> <destination>
```

### Examples

Download the default branch of `Rich-Harris/degit` into a folder named `my-project`:

```bash
degit Rich-Harris/degit my-project
```

Target a specific branch, tag, or commit using the `#` symbol:

```bash
# Target a branch
degit Rich-Harris/degit#dev my-project

# Target a release tag
degit Rich-Harris/degit#v1.0.0 my-project

# Target a commit hash
degit Rich-Harris/degit#a1b2c3d my-project
```

Use a different provider (GitHub is the default):

```bash
degit gitlab:user/repo my-project
degit bitbucket:user/repo my-project
```

## CLI Options

| Flag / Option   | Description |
|-----------------|-------------|
| `--force`       | Allow overwriting of existing files within the destination directory. |
| `--cache`       | Prefer using the locally cached tarball download if available. |
| `--clear-cache` | Clear the global degit cache directory on your system. |
| `--verbose`     | Output more detailed logs about the fetching/extraction operation. |
| `--mode`        | Specify the fetching strategy. Default is `tar`. Use `git` to invoke `git clone` explicitly. |

## How it Works

The application operates in two distinct phases:

1. **Source Parsing**: Parses a URI string (e.g., `gitlab:user/repo#dev`) identifying the provider, repository owner, repository name, and the explicit ref (defaults to `HEAD`).
2. **Execution Strategy**:
    - **Tar Mode (Default)**: Leverages `reqwest` to perform an HTTP fetch of the `.tar.gz` archive from the provider. Leverages standard OS cache directories to store archives to prevent redundant network fetches. Extraction is handled natively via `tar` and `flate2`, actively cleaning up the arbitrary top-level wrapper directory commonly injected by GitHub/GitLab.
    - **Git Mode Fallback**: Drops down to a native OS `std::process::Command` to invoke `git clone`. If a specific `ref` was requested, an ensuing `git checkout` is performed. It cleanly eradicates the initialized `.git` inner directory after resolution to provide an unlinked source template.

## Development & Testing

All unit tests assessing string parsing and validation are available via Cargo. Ensure your environment covers end-to-end extraction behaviors appropriately.

```bash
cargo test
```
