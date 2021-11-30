# TODO

## In Progress



## TODO


- [ ] Improve error handling
- [ ] Cleanup status reporting
- [ ] Improve crate/bin documentation
- [ ] Add command line options
	- [ ] Specify pre-release level (alpha/beta/[none])
	- [ ] Specify bump (patch/minor/major)
	
## DONE

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
