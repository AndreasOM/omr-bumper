# TODO

## In Progress


## TODO

- [ ] Skip dirty repository check if `--allow-dirty`
- [ ] Skip all git if `--skip-git`
- [ ] Skip pushing if `--skip-push`
- [ ] Skip tagggin if `--skip-tag`


- [ ] Improve error handling
- [ ] Cleanup status reporting
- [ ] Improve crate/bin documentation

- [ ] Fix usage of hardcode ssh/git key
- [ ] Remove dead code, and comments

## DONE

- [x] Fix all warnings
- [x] Add command line options
	- [x] Specify pre-release level (alpha/beta/[none])
	- [x] Specify bump (patch/minor/major)

- [x] Prepare release
	- [x] Check workspace/git is clean
	- [x] Remove -dev from version, and replace by *alpha*/beta/[none]
	- [x] Update Cargo.lock `cargo update --workspace --verbose`
	- [x] Commmit Cargo.toml
		- [x] and other files as needed (e.g. Cargo.lock)
	- [x] Push to git (fetch, rebase, push)
		- [x] fetch
		- [x] rebase
		- [x] push
	- [x] Tag the release
	- [x] Push the tag

- [x] Prepare to continue with development
	- [x] Bump the local version patch/minor/major
	- [x] Remove *alpha*/beta/[none] from version, and replace by dev
	- [x] Update Cargo.lock `cargo update --workspace --verbose`
	- [x] Commit Cargo.toml
		- [x] and other files as needed
	- [x] Push to git (fetch, rebase, push)

- [x] Commit message includes version
- [x] Improve output during execution
- [x] Preserve formatting of version field in manifest
- [x] Find git root in folder hierarchy


## Released
