use std::path::{Path, PathBuf};
use std::process::Command;

//use anyhow::*;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;

pub struct Repository {
	path: PathBuf,
}

impl Repository {
	pub fn new(path: &Path) -> Self {
		Self {
			path: path.to_owned(),
		}
	}

	#[allow(dead_code)]
	pub fn open(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	pub fn get_dirty(&mut self) -> anyhow::Result<Vec<String>> {
		// :HACK: This is super hacky, and very incomplete
		let mut changed_files = Vec::new();
		// Note: do not trim!
		let changes = self.git_cmd(&["status", "--branch", "--porcelain"])?;
		// let changes = changes.trim();
		for c in changes.split('\n') {
			// let c = c.trim();
			if c.len() >= 4 {
				//				eprintln!("Change: {}", &c);
				match &c[0..2] {
					"##" => println!(":TODO: branch status ignored {}", &c),
					" M" => changed_files.push(c[2..].to_string()),
					"??" => println!("Untracked file (not considered dirty): {}", &c[3..]),
					o => eprintln!("Unhandled status prefix {:?}", &o),
				}
			}
		}
		Ok(changed_files)
	}

	pub fn commit(&mut self, files: &[String], message: &str) -> anyhow::Result<()> {
		for f in files.iter() {
			//let p = Path::new(&cwd).join(&f);
			self.git_cmd(&["add", f])?;
		}
		self.git_cmd(&["status"])?;
		self.git_cmd(&["commit", "-m", message])?;
		self.git_cmd(&["status"])?;
		Ok(())
	}

	pub fn check_ignore(&mut self, file: &str) -> anyhow::Result<bool> {
		let r = self.git_cmd(&["check-ignore", file])?;
		Ok(!r.is_empty())
	}

	pub fn tag(&mut self, tag: &str, msg: &str) -> anyhow::Result<()> {
		self.git_cmd(&["tag", tag, "-m", msg])?;
		Ok(())
	}

	pub fn fetch(&mut self) -> anyhow::Result<usize> {
		self.git_cmd(&["fetch"])?;
		Ok(0)
	}

	pub fn rebase(&mut self) -> anyhow::Result<()> {
		self.git_cmd(&["rebase"])?;
		Ok(())
	}

	pub fn push(&mut self) -> anyhow::Result<usize> {
		self.git_cmd(&["push"])?;
		Ok(0)
	}

	pub fn push_tag(&mut self, tag: &str) -> anyhow::Result<usize> {
		self.git_cmd(&["push", "origin", tag])?;
		Ok(0)
	}

	fn git_cmd(&self, args: &[&str]) -> anyhow::Result<String> {
		let path = match self.path.clone().into_os_string().into_string() {
			Ok(p) => p,
			Err(e) => bail!("Can not convert path to string {:?}", &e),
		};

		tracing::debug!("git -C {} {}", &path, args.join(" "));
		let args: Vec<&str> = args.iter().map(|s| s.trim()).collect();
		let output = Command::new("git")
			.arg("-C")
			.arg(&path)
			.args(&args)
			.output()
			.with_context(|| format!("error running git `{args:?}`"))?;
		//		trace!("git output = {:?}", output);
		let stdout = Self::string_from_bytes(output.stdout)?;
		if output.status.success() {
			tracing::debug!("{}", stdout);
			Ok(stdout)
		} else {
			let mut error = "error while running git:\n".to_string();
			if !stdout.is_empty() {
				error.push_str("- stdout: ");
				error.push_str(&stdout);
			}
			let stderr = Self::string_from_bytes(output.stderr)?;
			if !stderr.is_empty() {
				error.push_str("- stderr: ");
				error.push_str(&stderr);
			}
			Err(anyhow!(error))
		}
	}
	fn string_from_bytes(bytes: Vec<u8>) -> anyhow::Result<String> {
		let stdout = String::from_utf8(bytes).context("cannot extract stderr")?;
		//		let stdout = stdout.trim();
		Ok(stdout)
	}
}
