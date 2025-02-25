use std::path::{Path, PathBuf};

use anyhow::bail;
use cargo::core::Workspace;
use cargo::ops::{self, UpdateOptions};
use cargo::util::context::GlobalContext;
use path_absolutize::*;

pub struct Cargo /*<'a>*/ {
	path: PathBuf,
	cfg:  Option<GlobalContext>,
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
		let cfg = GlobalContext::default()?;
		self.cfg = Some(cfg);
		if let Some(cfg) = &self.cfg {
			let manifest_path = std::path::Path::new(&self.path).join("Cargo.toml");
			let p = manifest_path.absolutize()?;

			// Check if Cargo.toml exists
			if !manifest_path.exists() {
				bail!("Cargo.toml not found at: {}", manifest_path.display());
			}

			// Check if Cargo.lock exists
			let lock_path = std::path::Path::new(&self.path).join("Cargo.lock");
			if !lock_path.exists() {
				bail!("Cargo.lock not found at: {}. Run 'cargo build' in the repository to generate it.", lock_path.display());
			}

			// Try to open and parse Cargo.lock to verify it's valid
			match std::fs::read_to_string(&lock_path) {
				Ok(contents) => {
					// Basic check if it looks like a valid Cargo.lock
					if !contents.contains("[package]")
						&& !contents.contains("version =")
						&& !contents.trim().is_empty()
					{
						bail!("Cargo.lock at {} appears to be malformed. Run 'cargo build' to regenerate it.", lock_path.display());
					}
				},
				Err(e) => {
					bail!(
						"Failed to read Cargo.lock at {}: {}",
						lock_path.display(),
						e
					);
				},
			}

			// Try to create a workspace - this will validate both Cargo.toml and Cargo.lock
			match Workspace::new(&p, cfg) {
				Ok(_ws) => {
					// Success!
					Ok(())
				},
				Err(e) => {
					bail!("Failed to create Cargo workspace: {}. Make sure both Cargo.toml and Cargo.lock are valid.", e);
				},
			}
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
			let manifest_path = std::path::Path::new(&self.path).join("Cargo.toml");
			let p = manifest_path.absolutize()?;

			// Check if Cargo.toml exists
			if !manifest_path.exists() {
				bail!("Cargo.toml not found at: {}", manifest_path.display());
			}

			// Check if Cargo.lock exists
			let lock_path = std::path::Path::new(&self.path).join("Cargo.lock");
			if !lock_path.exists() {
				bail!("Cargo.lock not found at: {}. Run 'cargo build' in the repository to generate it.", lock_path.display());
			}

			// Create workspace
			let ws = match Workspace::new(&p, cfg) {
				Ok(ws) => ws,
				Err(e) => bail!("Failed to create Cargo workspace: {}. Make sure both Cargo.toml and Cargo.lock are valid.", e)
			};

			let update_opts = UpdateOptions {
				// aggressive: false,
				precise:   None,
				recursive: true,
				to_update: Vec::new(),
				dry_run:   false,
				workspace: true,
				gctx:      cfg,
			};

			// Update lockfile
			match ops::update_lockfile(&ws, &update_opts) {
				Ok(_) => {
					println!("Updated Cargo.lock for {}", &p.display());
					Ok(())
				},
				Err(e) => bail!("Failed to update Cargo.lock: {}", e),
			}
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
