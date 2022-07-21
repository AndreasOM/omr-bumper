use std::path::{Path, PathBuf};

use anyhow::*;

use crate::cargo::Cargo;
use crate::manifest::Manifest;
use crate::Repository;

#[derive(Debug)]
enum BumpLevel {
	Patch,
	Minor,
	Major,
}

#[derive(Debug)]
pub struct Release {
	bump_level:         BumpLevel,
	pre_release_suffix: String,
	allow_dirty:        bool,
	skip_git:           bool,
	skip_push:          bool,
	skip_tag:           bool,
	path:               PathBuf,
}

impl Release {
	pub fn new() -> Self {
		Self {
			bump_level:         BumpLevel::Patch,
			pre_release_suffix: "alpha".to_string(),
			allow_dirty:        false,
			skip_git:           false,
			skip_push:          false,
			skip_tag:           false,
			path:               Path::new(".").to_path_buf(),
		}
	}

	pub fn set_pre_release_suffix(&mut self, pre_release_suffix: &str) {
		self.pre_release_suffix = pre_release_suffix.to_string();
	}

	pub fn set_bump_level(&mut self, bump_level: &str) -> anyhow::Result<()> {
		self.bump_level = match bump_level {
			"patch" => BumpLevel::Patch,
			"minor" => BumpLevel::Minor,
			"major" => BumpLevel::Major,
			o => bail!("Invalid bump level: {}", &o),
		};

		Ok(())
	}

	pub fn set_allow_dirty(&mut self, allow_dirty: bool) {
		self.allow_dirty = allow_dirty;
	}

	pub fn set_skip_git(&mut self, skip_git: bool) {
		self.skip_git = skip_git;
	}

	pub fn set_skip_push(&mut self, skip_push: bool) {
		self.skip_push = skip_push;
	}

	pub fn set_skip_tag(&mut self, skip_tag: bool) {
		self.skip_tag = skip_tag;
	}

	pub fn set_path(&mut self, path: &str) {
		self.path = Path::new(path).to_path_buf();
	}

	pub fn run(&self) -> anyhow::Result<()> {
		/* Note: use `--path [path]` instead
		if false { // use repository in sub folder for testing
			let root = std::path::Path::new(&std::env::current_dir()?).join("./automatic-octo-guacamole/");
			std::env::set_current_dir(&root)?;
		} else {

		}
		*/

		std::env::set_current_dir(&self.path)?;
		println!("Working in {:?}", std::env::current_dir()?);

		// check if git is clean
		let mut repo = Repository::new(".");
		//		let mut repo = Repository::new( "./automatic-octo-guacamole/" );

		if !self.skip_git {
			repo.open()?;
			if !self.allow_dirty {
				// skip dirty
				println!("Checking if repository is clean...");

				let dirty = repo.get_dirty();

				if dirty.len() > 0 {
					println!("Dirty files:");
					for d in dirty.iter() {
						println!("{}", d);
					}
					bail!("Repository is dirty");
				}
				println!("Repositiory is clean (enough)");
			} else {
				println!("Skipping check if repository is clean!");
			}
		}

		// load the Cargo.toml
		let mut manifest = Manifest::new("Cargo.toml");
		manifest.load()?;

		let old_version = manifest.get_pretty_version()?;
		println!("Current version: {}", &old_version);
		manifest.set_version_suffix(&self.pre_release_suffix)?;

		manifest.save()?;

		let mut cargo = Cargo::new(".");
		cargo.open()?;
		cargo.update_workspace()?;

		//		panic!("STOP");

		let release_version = manifest.get_pretty_version()?;
		println!("Release version: {}", &release_version);

		// dbg!(&doc);

		if !self.skip_git {
			let dirty = repo.get_dirty();

			if dirty.len() > 0 {
				println!("Dirty files:");
				for d in dirty.iter() {
					println!("{}", d);
				}
			}

			let mut files = Vec::new();
			files.push("Cargo.toml".to_owned());
			files.push("Cargo.lock".to_owned());

			let msg = format!(
				": Bump version for {} release - {}",
				&self.pre_release_suffix, &release_version
			);
			println!("Commit");
			repo.commit(&files, &msg)?;

			println!("Fetch");
			if repo.fetch()? > 0 {
				bail!("Fetch was not empty. Please resolve manually!")
			};
			println!("Rebase");
			repo.rebase()?;

			if !self.skip_push {
				println!("Push");
				repo.push()?;
			} else {
				println!("Skipping push to origin as requested.");
			}

			if !self.skip_tag {
				let tag_msg = format!(". Tag {}", &release_version);
				println!("Tag");
				repo.tag(&release_version, &tag_msg)?;
				if !self.skip_push {
					println!("Push Tag");
					repo.push_tag(&release_version)?;
				} else {
					println!("Skipping push (of tag) to origin as requested.");
				}
			} else {
				println!("Skipping tagging as requested.");
			}
		}

		// post release
		let mut repo = Repository::new(".");
		//		let mut repo = Repository::new( "./automatic-octo-guacamole/" );

		if !self.skip_git {
			repo.open()?;
		}

		println!("---- Post Release ----");

		match self.bump_level {
			BumpLevel::Patch => manifest.bump_patch_version()?,
			BumpLevel::Minor => manifest.bump_minor_version()?,
			BumpLevel::Major => manifest.bump_major_version()?,
		};

		manifest.set_version_suffix("dev")?;

		manifest.save()?;
		cargo.update_workspace()?;

		let new_version = manifest.get_pretty_version()?;
		println!("New development version: {}", &new_version);

		if !self.skip_git {
			let mut files = Vec::new();
			files.push("Cargo.toml".to_owned());
			files.push("Cargo.lock".to_owned());

			let msg = format!(
				": Bump version back to dev release, and bump patch level - {}",
				&new_version
			);
			repo.commit(&files, &msg)?;

			let dirty = repo.get_dirty();

			if dirty.len() > 0 {
				println!("Dirty files before fetch:");
				for d in dirty.iter() {
					println!("{}", d);
				}
			}

			println!("Fetch");
			if repo.fetch()? > 0 {
				bail!("Fetch was not empty. Please resolve manually!")
			};
			println!("Rebase");
			repo.rebase()?;

			if !self.skip_push {
				println!("Push");
				repo.push()?;
			} else {
				println!("Skipping push to origin as requested.");
			}

			let dirty = repo.get_dirty();

			if dirty.len() > 0 {
				println!("Dirty files:");
				for d in dirty.iter() {
					println!("{}", d);
				}
			}
		}

		Ok(())
	}
}
