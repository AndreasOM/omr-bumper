# omr-bumper

A tool to automate version bumping and release preparation for Rust projects.

## Installation

### Using cargo-binstall

The easiest way to install omr-bumper is with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall omr-bumper
```

### Using cargo

```bash
cargo install omr-bumper
```

## About

Does all the necessary preparation to have GitHub build a release.

- Prepare release
	- Checks workspace/git is clean
	- Removes -dev from version, and replaces by alpha/beta/[none]
	- Update Cargo.lock `cargo update --workspace --dry-run --verbose`
	- Commits Cargo.toml (and other files as needed)
	- Pushes to git
	- Tags the release
	- Pushes the tag
- Prepare to continue with development
	- Bumps the local version patch/minor/major
	- Commits Cargo.toml
	- Pushes to git

## CI/CD

The project uses GitHub Actions workflows for:
- Publishing to crates.io on tag push
- Building binaries for multiple platforms (Linux, macOS, Windows)
- Creating GitHub releases with change logs
- Generating cargo-binstall metadata

## Note

`"revspec 'origin/HEAD' not found"`

The git interface is pretty complex.
For them moment we use `origin/HEAD` to get the "correct" refspec.
If you didn't create your local repository by cloning this will not exist.
You can create it via `git symbolic-ref refs/remotes/origin/HEAD refs/remotes/origin/master`.
This will be fixed in a future release.

You can verify `origin/HEAD` exists via `git show-ref`.
