
use anyhow::*;

use crate::cargo::Cargo;
use crate::manifest::Manifest;
use crate::repository::Repository;

#[derive(Debug)]
enum BumpLevel {
	Patch,
	Minor,
	Major
}

#[derive(Debug)]
pub struct Release {
	bump_level:			BumpLevel,
	pre_release_suffix:	String,
	allow_dirty:		bool,
}


impl Release {
	pub fn new() -> Self {
		Self {
			bump_level:			BumpLevel::Patch,
			pre_release_suffix: "alpha".to_string(),
			allow_dirty:		false,
		}
	}

	pub fn set_pre_release_suffix( &mut self, pre_release_suffix: &str ) {
		self.pre_release_suffix = pre_release_suffix.to_string();
	}

	pub fn set_bump_level(&mut self, bump_level: &str ) -> anyhow::Result<()> {

		self.bump_level = match bump_level {
			"patch" => BumpLevel::Patch,
			"minor" => BumpLevel::Minor,
			"major" => BumpLevel::Major,
			o => bail!( "Invalid bump level: {}", &o ),
		};

		Ok(())
	}

	pub fn set_allow_dirty( &mut self, allow_dirty: bool ) {
		self.allow_dirty = allow_dirty;
	}

	pub fn run( &self ) -> anyhow::Result<()> {

		if false { // use repository in sub folder for testing
			let root = std::path::Path::new(&std::env::current_dir()?).join("./automatic-octo-guacamole/");
			std::env::set_current_dir(&root)?;
		} else {

		}

		println!("Working in {:?}", std::env::current_dir()?);
		// check if git is clean
		let mut repo = Repository::new( "." );
//		let mut repo = Repository::new( "./automatic-octo-guacamole/" );

		repo.open()?;
		if !self.allow_dirty { // skip dirty
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

		if true { // skip for faster iteration
		// load the Cargo.toml
		let mut manifest = Manifest::new( "Cargo.toml" );
		manifest.load()?;

		let old_version = manifest.get_pretty_version()?;
		println!("Current version: {}", &old_version);
		manifest.set_version_suffix( &self.pre_release_suffix )?;

		manifest.save()?;

		let mut cargo = Cargo::new( "." );
		cargo.open()?;
		cargo.update_workspace()?;

//		panic!("STOP");

		let release_version = manifest.get_pretty_version()?;
		println!("Release version: {}", &release_version);

		// dbg!(&doc);

		let dirty = repo.get_dirty();

		if dirty.len() > 0 {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
		}

		// :TODO: update Cargo.lock

		let mut files = Vec::new();
		files.push( "Cargo.toml".to_owned() );
		files.push( "Cargo.lock".to_owned() );
		let msg = format!( ": Bump version for {} release - {}", &self.pre_release_suffix ,&release_version );
		println!("Commit");
		repo.commit( &files, &msg )?;

		println!("Fetch");
		if repo.fetch()? > 0 {
			bail!("Fetch was not empty. Please resolve manually!")
		};
		println!("Rebase");
		repo.rebase()?;
		println!("Push");
		repo.push()?;

		let tag_msg = format!( ". Tag {}", &release_version );
		println!("Tag");
		repo.tag( &release_version, &tag_msg )?;
		println!("Push Tag");
		repo.push_tag( &release_version )?;


		// post release
		let mut repo = Repository::new( "." );
//		let mut repo = Repository::new( "./automatic-octo-guacamole/" );

		repo.open()?;

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


		// :TODO: update Cargo.lock
		let msg = format!( ": Bump version back to dev release, and bump patch level - {}", &new_version );
		repo.commit( &files, &msg )?;

		} else {
			println!( "Skipping everything up to fetch/rebase/push!" );
		}

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
		println!("Push");
		repo.push()?;

		let dirty = repo.get_dirty();

		if dirty.len() > 0 {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
		}

		Ok(())
	}
}
