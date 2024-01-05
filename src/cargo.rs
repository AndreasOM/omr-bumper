use std::path::{Path, PathBuf};

use anyhow::bail;
use cargo::core::Workspace;
use cargo::ops::{self, UpdateOptions};
use cargo::util::config::Config;
use path_absolutize::*;

pub struct Cargo /*<'a>*/ {
	path: PathBuf,
	cfg:  Option<Config>,
	//	ws: Option< Workspace<'a> >,
}

impl Cargo /*<'a>*/ {
	pub fn new(path: &Path) -> Self {
		Self {
			path: path.to_owned(),
			cfg:  None,
			//			ws: None,
		}
	}

	pub fn open(&mut self) -> anyhow::Result<()> {
		let cfg = Config::default()?;
		self.cfg = Some(cfg);
		if let Some(cfg) = &self.cfg {
			let p = std::path::Path::new(&self.path).join("Cargo.toml");
			let p = p.absolutize()?;
			let _ws = Workspace::new(&p, cfg)?;
			//			self.ws = Some( ws );
			//			dbg!(&ws);
			Ok(())
		} else {
			bail!("No Config");
		}
	}
	/*
	WorkspaceRootConfig::
	pub fn new(
		root_dir: &Path,
		members: &Option<Vec<String>>,
		default_members: &Option<Vec<String>>,
		exclude: &Option<Vec<String>>,
		custom_metadata: &Option<Value>
	) -> WorkspaceRootConfig
	*/

	/*
	Workspace::
	pub fn new(
		manifest_path: &Path,
		config: &'cfg Config
	) -> CargoResult<Workspace<'cfg>>
	*/
	// Config::default()

	pub fn update_workspace(&mut self) -> anyhow::Result<()> {
		//		let cfg = WorkspaceRootConfig::new(".")
		//		ops::update_lockfile(&ws, &update_opts)?;
		if let Some(cfg) = &self.cfg {
			let p = std::path::Path::new(&self.path).join("Cargo.toml");
			let p = p.absolutize()?;
			let ws = Workspace::new(&p, cfg)?;

			let update_opts = UpdateOptions {
				// aggressive: false,
				precise:   None,
				recursive: true,
				to_update: Vec::new(),
				dry_run:   false,
				workspace: true,
				config:    cfg,
			};
			//			dbg!(&ws);
			ops::update_lockfile(&ws, &update_opts)?;
			println!("Updated Cargo.lock for {}", &p.display());
			Ok(())
		} else {
			bail!("No Config");
		}

		/*
		if let Some( ws ) = &self.ws {

			Ok(())
		} else {
			bail!( "No Workspace when updating" );
		}
		*/
	}
}
