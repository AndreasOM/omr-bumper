use std::path::Path;

//use anyhow::*;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;

use std::process::Command;

pub struct Repository {
	path: String,
	repo: Option<git2::Repository>,
}

impl Repository {
	pub fn new(path: &str) -> Self {
		Self {
			path: path.to_owned(),
			repo: None,
		}
	}

	pub fn open(&mut self) -> anyhow::Result<()> {
		let repo = match git2::Repository::discover(&self.path) {
			Ok(repo) => repo,
			Err(e) => bail!("failed to open: {}", e),
		};

		//		dbg!(&repo.state());

		self.repo = Some(repo);

		Ok(())
	}

	pub fn get_dirty(&mut self) -> Vec<String> {
		Vec::new() // :TODO:

		/*
		match &self.repo {
			Some(repo) => {
				let mut dirty = Vec::new();
				let mut check_s = Status::empty();
				check_s.insert(Status::INDEX_NEW);
				check_s.insert(Status::INDEX_MODIFIED);
				check_s.insert(Status::WT_NEW);
				check_s.insert(Status::WT_MODIFIED);

				let mut skip_s = Status::empty();
				skip_s.insert(Status::IGNORED);
				skip_s.insert(Status::WT_NEW);

				for se in repo.statuses(None).unwrap().iter() {
					let s = se.status();
					//					println!("Maybe dirty {:?} {}", s, se.path().unwrap_or( "" ) );
					if s.intersects(check_s) {
						dirty.push(se.path().unwrap_or("").to_owned());
					} else {
						if !s.intersects(skip_s) {
							println!("Not dirty {:?} {}", s, se.path().unwrap_or(""));
						}
					}
				}
				dirty
			},
			None => Vec::new(),
		}
		*/
	}

	pub fn commit(&mut self, files: &Vec<String>, message: &str) -> anyhow::Result<()> {
		for f in files.iter() {
			//let p = Path::new(&cwd).join(&f);
			self.git_cmd(&["add",f]);
		}
		self.git_cmd(&["status"]);
//		self.git_cmd(&["commit", "-m", &format!("\"{}\"",message), &files.join(" ")]);
		self.git_cmd(&["commit", "-m", &message]);
		self.git_cmd(&["status"]);

/*
		match &self.repo {
			Some(repo) => {
				/*
				let name = "Somebody";
				let email = "some@body.org";
				let author = Signature::now( &name, &email )?;
				let commiter = Signature::now( &name, &email )?;
				*/

				let sig = repo.signature()?;

				let mut index = match repo.index() {
					Err(e) => bail!("No index for repository {}", &e),
					Ok(index) => index,
				};
				let cwd = match std::env::current_dir() {
					Ok(wd) => wd,
					Err(e) => bail!("No current working directory found: {}", &e),
				};
				//				dbg!(&repo.path());
				//				dbg!(&repo.workdir());
				let rwd = match repo.workdir() {
					Some(wd) => wd,
					None => bail!("No workdir for repository"),
				};
				//				dbg!(&cwd);
				//				dbg!(&rwd);
				for f in files.iter() {
					let p = Path::new(&cwd).join(&f);
					//					dbg!(&p);
					// filenames are relative to working directory
					// make them relative to repository root
					let p = match p.strip_prefix(&rwd) {
						Ok(p) => p,
						Err(e) => bail!("Error stripping {:?} from {:?}: {}", &rwd, &p, &e),
					};
					//					dbg!(&p);
					index.add_path(&p)?;
				}

				index.write()?;

				let mut index = match repo.index() {
					Err(e) => bail!("No index for repository {}", &e),
					Ok(index) => index,
				};
				/*
				pub fn write_tree(&mut self) -> Result<Oid, Error>
				*/
				let oid = match index.write_tree() {
					Ok(oid) => oid,
					Err(e) => bail!("Error writing tree for repository: {}", &e),
				};
				index.write()?;

				//				dbg!(&oid);

				let tree = match repo.find_tree(oid) {
					Ok(tree) => tree,
					Err(e) => bail!("Error findind tree for OID {}: {}", &oid, &e),
				};

				//				dbg!(&tree);

				let parent = match repo.revparse_ext("HEAD") {
					Ok((object, _)) => object,
					Err(e) => bail!("Error finding HEAD {}", &e),
				};

				let parent = match parent.as_commit() {
					Some(commit) => commit,
					None => bail!("Parent is not a commit"),
				};

				//				dbg!(&parent);
				/*
				pub fn commit(
					&self,
					update_ref: Option<&str>,
					author: &Signature<'_>,
					committer: &Signature<'_>,
					message: &str,
					tree: &Tree<'_>,
					parents: &[&Commit<'_>]
				) -> Result<Oid, Error>
								*/
				/*
				pub fn find_tree(&self, oid: Oid) -> Result<Tree<'_>, Error>

				pub fn revparse_ext(
					&self,
					spec: &str
				) -> Result<(Object<'_>, Option<Reference<'_>>), Error>
								*/
				repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[parent])?;
				Ok(())
			},
			None => bail!("No repo open for commit"),
		}
		*/

		Ok(())

	}

	pub fn tag(&mut self, tag: &str, msg: &str) -> anyhow::Result<()> {
		self.git_cmd(&["tag",tag,"-m", msg]);
		Ok(())
		/*
		match &self.repo {
			Some(repo) => {
				let rv = repo.revparse("HEAD")?;
				let ho = match rv.from() {
					Some(ho) => ho,
					None => bail!("No HEAD found!"),
				};
				println!("Tagging {} with {}", &ho.id(), &tag);
				//				dbg!(&ho);

				let sig = repo.signature()?;

				let _tag_oid = repo.tag(tag, &ho, &sig, msg, true)?;
				//				dbg!(&tag_oid);
				Ok(())
				/*
				pub fn tag(
					&self,
					name: &str,
					target: &Object<'_>,
					tagger: &Signature<'_>,
					message: &str,
					force: bool
				) -> Result<Oid, Error>
				*/
			},
			None => bail!("No repo open for tag"),
		}
		*/
	}
/*
	fn credentials_cb(
		url: &str,
		username_from_url: Option<&str>,
		_allowed_types: git2::CredentialType,
	) -> Result<Cred, git2::Error> {
		//		dbg!(&username_from_url);
		dbg!(&url);
		Cred::ssh_key(
			username_from_url.unwrap(),
			None,
			std::path::Path::new(&format!(
				"{}/.ssh/id_ed25519",
				std::env::var("HOME").unwrap()
			)),
			//			std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
			None,
		)
	}
*/
	pub fn fetch(&mut self) -> anyhow::Result<usize> {
		self.git_cmd(&["fetch"]);
		/*
		match &self.repo {
			Some(repo) => {
				let remote_name = "origin";
				let mut remote = match repo.find_remote(&remote_name) {
					Ok(remote) => remote,
					Err(e) => bail!("Couldn't find remote({}): {}", &remote_name, &e),
				};

				//				dbg!(&remote.name(), &remote.url());

				let mut cbs = RemoteCallbacks::new();
				cbs.credentials(|url, username_from_url, allowed_types| {
					Repository::credentials_cb(url, username_from_url, allowed_types)
				});
				cbs.transfer_progress(|progress| {
					println!("Transfer progress: {}", progress.received_bytes());
					println!(
						"{}/{} objects",
						progress.received_objects(),
						progress.total_objects()
					);
					//					dbg!(&progress.received_bytes());
					true
				});
				let mut opts = FetchOptions::new();
				opts.remote_callbacks(cbs);
				dbg!(&remote.url());
				remote.fetch(&["main"], Some(&mut opts), None)?;
				let stats = remote.stats();
				println!("Fetched {} bytes.", stats.received_bytes());
				println!("Fetched {} objects.", stats.received_objects());
				Ok(stats.total_objects())
			},
			None => bail!("No repo open for fetch"),
		}
		*/
		Ok(0)
	}

	pub fn rebase(&mut self) -> anyhow::Result<()> {
		self.git_cmd(&["rebase"]);
		/*
		match &self.repo {
			Some(repo) => {
				//				let head = repo.head()?; // wrong, this is local HEAD, we want origin/HEAD
				//				let upstream = repo.reference_to_annotated_commit( &head )?;
				let rv = match repo.revparse("origin/HEAD") {
					Ok(rv) => rv,
					Err(e) => bail!("Error: {:?}", &e),
				};

				let oho = match rv.from() {
					Some(oho) => oho,
					None => bail!("No origin/HEAD found!"),
				};
				//				dbg!(&oho);

				let upstream = match repo.find_annotated_commit(oho.id()) {
					Ok(commit) => commit,
					Err(e) => bail!("No commit for origin/HEAD! {}", &e),
				};
				/*
				pub fn revparse(&self, spec: &str) -> Result<Revspec<'_>, Error>
				*/
				println!("Rebasing on upstream {} {}", upstream.id(), ""); //upstream.refname().unwrap_or("") );
														   //				let upstream = None; // AnnotatedCommit
				let mut rebase = repo.rebase(None, Some(&upstream), None, None)?;
				println!("{}", rebase.len());
				while let Some(ro) = rebase.next() {
					//				};
					//				for ro in &mut rebase {
					match ro {
						Ok(_ro) => {
							//							dbg!( &ro );
							// commit
							/*
							pub fn commit(
								&mut self,
								author: Option<&Signature<'_>>,
								committer: &Signature<'_>,
								message: Option<&str>
							) -> Result<Oid, Error>
							*/
							let sig = repo.signature()?;

							match rebase.commit(None, &sig, None) {
								Ok(_r) => {
									//									dbg!( &r );
								},
								Err(e) => {
									dbg!(&e);
									// :TODO: decide when to bail
									//									bail!( "Rebase Error {:?}", &e );
								},
							}
						},
						Err(e) => {
							dbg!(&e);
						},
					}
				}
				//				rebase.abort()?;
				rebase.finish(None)?;
				//				dbg!(&rebase);
			},
			None => bail!("No repo open for rebase"),
		}
		*/
		Ok(())
		/*

		pub fn head(&self) -> Result<Reference<'_>, Error>

		pub fn reference_to_annotated_commit(
			&self,
			reference: &Reference<'_>
		) -> Result<AnnotatedCommit<'_>, Error>


		pub fn rebase(
			&self,
			branch: Option<&AnnotatedCommit<'_>>,
			upstream: Option<&AnnotatedCommit<'_>>,
			onto: Option<&AnnotatedCommit<'_>>,
			opts: Option<&mut RebaseOptions<'_>>
		) -> Result<Rebase<'_>, Error>
				*/
	}

	pub fn push(&mut self) -> anyhow::Result<usize> {
		self.git_cmd(&["push"]);
		/*
		match &self.repo {
			Some(repo) => {
				let remote_name = "origin";
				let mut remote = match repo.find_remote(&remote_name) {
					Ok(remote) => remote,
					Err(e) => bail!("Couldn't find remote({}): {}", &remote_name, &e),
				};

				dbg!(remote.name());

				let mut cbs = RemoteCallbacks::new();
				cbs.credentials(|url, username_from_url, allowed_types| {
					Repository::credentials_cb(url, username_from_url, allowed_types)
				});
				cbs.transfer_progress(|progress| {
					println!("Transfer progress: {}", progress.received_bytes());
					println!(
						"{}/{} objects",
						progress.received_objects(),
						progress.total_objects()
					);
					//					dbg!(&progress.received_bytes());
					true
				});
				let mut opts = PushOptions::new();
				opts.remote_callbacks(cbs);
				remote.push(&["refs/heads/main"], Some(&mut opts))?;

				/*
				// only works for fetch!!!
				let stats = remote.stats();
				println!("Pushed {} bytes.", stats.received_bytes());
				println!("Pushed {} objects.", stats.received_objects());
				Ok(stats.total_objects())
				*/

				Ok(0)

				/*
				pub fn push<Str: AsRef<str> + IntoCString + Clone>(
					&mut self,
					refspecs: &[Str],
					opts: Option<&mut PushOptions<'_>>
				) -> Result<(), Error>
								*/
			},
			None => bail!("No repo open for push"),
		}
		*/
		Ok(0)
	}

	pub fn push_tag(&mut self, tag: &str) -> anyhow::Result<usize> {
		self.git_cmd(&["push","origin", tag]);
		/*
		match &self.repo {
			Some(repo) => {
				let remote_name = "origin";
				let mut remote = match repo.find_remote(&remote_name) {
					Ok(remote) => remote,
					Err(e) => bail!("Couldn't find remote({}): {}", &remote_name, &e),
				};

				dbg!(remote.name());

				let mut cbs = RemoteCallbacks::new();
				cbs.credentials(|_url, username_from_url, _allowed_types| {
					//					dbg!(&username_from_url);
					Cred::ssh_key(
						username_from_url.unwrap(),
						None,
						std::path::Path::new(&format!(
							"{}/.ssh/id_ed25519",
							std::env::var("HOME").unwrap()
						)),
						//						std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
						None,
					)
				});
				cbs.transfer_progress(|progress| {
					println!("Transfer progress: {}", progress.received_bytes());
					println!(
						"{}/{} objects",
						progress.received_objects(),
						progress.total_objects()
					);
					//					dbg!(&progress.received_bytes());
					true
				});
				cbs.push_update_reference(|name, status| {
					println!("Push Update Reference: {} -> {:?}", name, status);
					Ok(())
				});
				let mut opts = PushOptions::new();
				opts.remote_callbacks(cbs);
				let tag_ref = format!("refs/tags/{}", &tag);
				println!("Pushing ref {}", &tag_ref);
				match remote.push(&[&tag_ref], Some(&mut opts)) {
					Ok(()) => {},
					Err(e) => {
						bail!("Error pushing ref to remote: {:?}", &e);
					},
				};
				//git push <corresponding remote> refs/tags/*:refs/tags/*

				/*
				// only works for fetch!!!
				let stats = remote.stats();
				println!("Pushed {} bytes.", stats.received_bytes());
				println!("Pushed {} objects.", stats.received_objects());
				Ok(stats.total_objects())
				*/

				Ok(0)

				/*
				pub fn push<Str: AsRef<str> + IntoCString + Clone>(
					&mut self,
					refspecs: &[Str],
					opts: Option<&mut PushOptions<'_>>
				) -> Result<(), Error>
								*/
			},
			None => bail!("No repo open for push"),
		}
		*/
		*/
		*/
		Ok(0)
	}

	fn git_cmd(&self, args: &[&str]) -> anyhow::Result<String> {
		println!("git -C {} {}", &self.path, args.join(" "));
		let args: Vec<&str> = args.iter().map(|s| s.trim()).collect();
		let output = Command::new("git")
			.arg("-C")
			.arg(&self.path)
			.args(&args)
			.output()
			.with_context(|| {
				format!("error running git `{args:?}`")
			})?;
//		trace!("git output = {:?}", output);
		let stdout = Self::string_from_bytes(output.stdout)?;
		if output.status.success() {
			println!("{}",stdout);
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
    	let stdout = stdout.trim();
    	Ok(stdout.to_string())
	}
}
