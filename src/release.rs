use std::path::{Path, PathBuf};

use anyhow::*;

use crate::cargo::Cargo;
use crate::manifest::Manifest;
use crate::Repository;

#[derive(Debug, Clone)]
enum GitCommitMessage {
	BumpVersionForRelease,
	BumpVersionForDev,
}
#[derive(Debug, Clone)]
enum Step {
	GitEnsureClean, // fails if there are any changes
	GitShowDirty,
	CargoLoadManifest,
	ManifestSetVersionSuffix,
	ManifestSetVersionSuffixDev,
	CargoSaveManifest,
	CargoUpdateWorkspace,
	ManifestPrintVersion,
	GitCommitManifest(GitCommitMessage),
	GitFetch,
	GitRebase,
	GitPush,
	GitTag,
	GitPushTag,
	ManifestBumpLevel,
}

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
	skip_all:           bool,
	path:               PathBuf,
	steps:              Vec<Step>,
}

impl Default for Release {
	fn default() -> Self {
		Release::new()
	}
}

impl Release {
	pub fn new() -> Self {
		Self {
			bump_level:             BumpLevel::Patch,
			pre_release_suffix:     "alpha".to_string(),
			allow_dirty:            false,
			skip_git:               false,
			skip_push:              false,
			skip_tag:               false,
			skip_all:               false,
			path:                   Path::new(".").to_path_buf(),
			#[rustfmt::skip]
			steps:              [ /* :WIP: */
									// pre release
									Step::GitEnsureClean,
									Step::CargoLoadManifest,
									Step::ManifestSetVersionSuffix,
									Step::CargoSaveManifest,
									Step::CargoUpdateWorkspace,
									Step::ManifestPrintVersion,
									Step::GitShowDirty,
									Step::GitCommitManifest(GitCommitMessage::BumpVersionForRelease),
									Step::GitFetch,
									Step::GitRebase,
									Step::GitPush,
									Step::GitTag,
									Step::GitPushTag,
									// post release
									Step::ManifestBumpLevel,
									Step::ManifestSetVersionSuffixDev,
									Step::CargoSaveManifest,
									Step::CargoUpdateWorkspace,
									Step::ManifestPrintVersion,
									Step::GitShowDirty,
									Step::GitCommitManifest(GitCommitMessage::BumpVersionForDev),
									Step::GitFetch,
									Step::GitRebase,
									Step::GitPush,
									Step::GitShowDirty,
								].to_vec(),
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

	pub fn set_skip_all(&mut self, skip_all: bool) {
		self.skip_all = skip_all;
	}

	pub fn set_path(&mut self, path: &str) {
		self.path = Path::new(path).to_path_buf();
	}

	fn step_git_ensure_clean(&self) -> anyhow::Result<bool> {
		let mut repo = Repository::new(&self.path);

		println!("Checking if repository is clean...");

		let dirty = repo.get_dirty()?;

		if !dirty.is_empty() {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
			//bail!("Repository is dirty");
			return Ok(false);
		}
		println!("Repositiory is clean (enough)");
		Ok(true)
	}
	fn step_git_show_dirty(&self) -> anyhow::Result<()> {
		let mut repo = Repository::new(&self.path);

		println!("Checking if repository is clean...");

		let dirty = repo.get_dirty()?;

		if !dirty.is_empty() {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
			//bail!("Repository is dirty");
			return Ok(());
		}
		println!("Repositiory is clean (enough)");
		Ok(())
	}

	pub fn run(&self) -> anyhow::Result<()> {
		let mut manifest = None;
		let mut release_version = "".to_string();
		for s in &self.steps {
			eprintln!("Step: {:?}", &s);
			match s {
				Step::GitEnsureClean => {
					if !self.step_git_ensure_clean()? {
						if !self.allow_dirty {
							println!("Not clean! STOPPING!");
							return Ok(());
						} else {
							println!("Not clean! I hope you know what you do!");
						}
					}
				},
				Step::GitShowDirty => {
					self.step_git_show_dirty()?;
				},
				Step::CargoLoadManifest => {
					let mut new_manifest = Manifest::new(&self.path.join("Cargo.toml"));
					new_manifest.load()?;
					manifest = Some(new_manifest);
					println!("Loaded manifest");
				},
				Step::CargoSaveManifest => {
					if let Some(m) = &mut manifest {
						m.save()?;
					} else {
						bail!("Tried to save manifest without manifest");
					}
				},
				Step::ManifestSetVersionSuffix => {
					if let Some(m) = &mut manifest {
						m.set_version_suffix(&self.pre_release_suffix)?;
					} else {
						bail!("Tried to set suffix without manifest");
					}
				},
				Step::ManifestSetVersionSuffixDev => {
					if let Some(m) = &mut manifest {
						m.set_version_suffix("dev")?;
					} else {
						bail!("Tried to set dev suffix without manifest");
					}
				},
				Step::ManifestBumpLevel => {
					if let Some(m) = &mut manifest {
						match self.bump_level {
							BumpLevel::Patch => m.bump_patch_version()?,
							BumpLevel::Minor => m.bump_minor_version()?,
							BumpLevel::Major => m.bump_major_version()?,
						};
					} else {
						bail!("Tried to bump level without manifest");
					}
				},
				Step::CargoUpdateWorkspace => {
					let mut cargo = Cargo::new(&self.path);
					cargo.open()?;
					cargo.update_workspace()?;
				},
				Step::ManifestPrintVersion => {
					if let Some(m) = &mut manifest {
						release_version = m.get_pretty_version()?;
						println!("Release version: {}", &release_version);
					} else {
						bail!("Tried to print version without manifest");
					}
				},
				Step::GitCommitManifest(m) => {
					/*
					let mut files = Vec::new();
					files.push("Cargo.toml".to_owned());
					files.push("Cargo.lock".to_owned());
					*/
					let files = vec!["Cargo.toml".to_owned(), "Cargo.lock".to_owned()];
					let msg = match m {
						GitCommitMessage::BumpVersionForRelease => {
							format!(
								": Bump version for {} release - {}",
								&self.pre_release_suffix, &release_version
							)
						},
						GitCommitMessage::BumpVersionForDev => {
							format!(
								": Bump version back to dev release, and bump patch level - {}",
								&release_version
							)
						},
						#[allow(unreachable_patterns)]
						o => format!(":TODO: {:?}", &o),
					};

					let mut repo = Repository::new(&self.path);
					repo.commit(&files, &msg)?;
				},
				Step::GitFetch => {
					let mut repo = Repository::new(&self.path);
					if repo.fetch()? > 0 {
						bail!("Fetch was not empty. Please resolve manually!")
					};
				},
				Step::GitRebase => {
					let mut repo = Repository::new(&self.path);
					repo.rebase()?;
				},
				Step::GitPush => {
					let mut repo = Repository::new(&self.path);
					repo.push()?;
				},
				Step::GitTag => {
					let mut repo = Repository::new(&self.path);
					let tag_msg = format!(". Tag {}", &release_version);
					repo.tag(&release_version, &tag_msg)?;
				},
				Step::GitPushTag => {
					let mut repo = Repository::new(&self.path);
					repo.push_tag(&release_version)?;
				},
				#[allow(unreachable_patterns)]
				s => eprintln!("Step {:?} not handled yet", &s),
			}
		}

		Ok(())
	}
}
