//! Does all the necessary preparation to have github build a release.
//!
//! - Prepare release
//!		- Checks workspace/git is clean
//!		- Removes -dev from version, and replaces by alpha/beta/[none]
//!		- Update Cargo.lock `cargo update --workspace --dry-run --verbose`
//!		- Commmits Cargo.toml (and other files as needed)
//!		- Pushes to git
//!		- Tags the release
//!		- Pushes the tag
//! - Prepare to continue with development
//!		- Bumps the local version patch/minor/major
//!		- Commits Cargo.toml
//!		- Pushes to git

use anyhow::bail;
use clap::Parser;
use omr_bumper::Release;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "omr-bumper")]
#[command(author = "Andreas N. <andreas@omni-mad.com")]
#[command(version = VERSION)]
#[command(about = "Bump version, and push to git with tag", long_about = None)]
struct Cli {
	#[arg(short = 'r', long)]
	pre_release_suffix: Option<String>,
	#[arg(short = 'b', long)]
	bump_level:         Option<String>,
	#[arg(long)]
	allow_dirty:        bool,
	#[arg(long)]
	skip_git:           bool,
	#[arg(long)]
	skip_push:          bool,
	#[arg(long)]
	skip_tag:           bool,
	#[arg(long)]
	skip_all:           bool,
	#[arg(long)]
	path:               Option<String>,
}

pub fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	let cli = Cli::parse();
	let pre_release_suffix = cli
		.pre_release_suffix
		.unwrap_or(String::from("alpha"))
		.to_string();
	let bump_level = cli.bump_level.unwrap_or(String::from("patch")).to_string();
	let allow_dirty = cli.allow_dirty;
	let skip_git = cli.skip_git;
	let skip_push = cli.skip_push;
	let skip_tag = cli.skip_tag;
	let skip_all = cli.skip_all;
	let path = cli.path.unwrap_or(String::from(".")).to_string();

	if ![
		"patch".to_string(),
		"minor".to_string(),
		"major".to_string(),
	]
	.contains(&bump_level)
	{
		println!(
			"Error: Invalid bump level {} should be patch/minor/major",
			&bump_level
		);
		bail!("Usage Error");
	};

	println!("Pre Release Suffix : {}", pre_release_suffix);
	println!("Bump Level         : {}", bump_level);
	//	println!( "Allow Dirty        : {}", allow_dirty?"yes":"no" );
	println!(
		"Allow Dirty        : {}",
		if allow_dirty { "yes" } else { "no" }
	);
	println!(
		"Skip Git           : {}",
		if skip_git { "yes" } else { "no" }
	);
	println!(
		"Skip Push          : {}",
		if skip_push { "yes" } else { "no" }
	);
	println!(
		"Skip Tag           : {}",
		if skip_tag { "yes" } else { "no" }
	);
	println!(
		"Skip All           : {}",
		if skip_all { "yes" } else { "no" }
	);
	println!("Path               : {}", path);

	let mut release = Release::new();

	release.set_pre_release_suffix(&pre_release_suffix);
	release.set_bump_level(&bump_level)?;
	release.set_allow_dirty(allow_dirty);
	release.set_skip_git(skip_git);
	release.set_skip_push(skip_push);
	release.set_skip_tag(skip_tag);
	release.set_skip_all(skip_all);
	release.set_path(&path);

	//	dbg!( &release );

	match release.run() {
		Ok(_) => {},
		Err(e) => {
			println!("Error: {}", e);
		},
	};
	Ok(())
}
