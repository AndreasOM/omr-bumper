use clap::{
	Arg,
	App,
};

use anyhow::bail;

use omr_bumper::Release;

pub fn main() -> anyhow::Result<()> {

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let matches = App::new( "omr-bumper" )
					.version( VERSION )
					.author( "Andreas N. <andreas@omni-mad.com" )
					.about( "Bump version, and push to git with tag" )
					.arg( Arg::with_name( "pre-release-suffix" )
							.long( "pre-release-suffix" )
							.short( "r" )
							.value_name( "PRE_RELEASE_SUFFIX" )
							.help( "Set the pre release suffix" )
							.takes_value( true )
					)
					.arg( Arg::with_name( "bump-level" )
							.long( "bump-level" )
							.short( "b" )
							.value_name( "BUMP_LEVEL" )
							.help( "Set the bump level [patch/minor/major]" )
							.takes_value( true )
					)
					.arg( Arg::with_name( "allow-dirty")
						.long( "allow-dirty" )
						.help( "Allow running on dirty repository" )
						.takes_value( false )
					)
					.arg( Arg::with_name( "skip-git")
						.long( "skip-git" )
						.help( "Skip ALL git steps" )
						.takes_value( false )
					)
					.get_matches();


//	dbg!( &matches );

	let pre_release_suffix	= matches.value_of( "pre-release-suffix" ).unwrap_or( "alpha" ).to_string();
	let bump_level			= matches.value_of( "bump-level" ).unwrap_or( "patch" ).to_string();
	let allow_dirty			= matches.is_present( "allow-dirty" );
	let skip_git			= matches.is_present( "skip-git" );

	if ![ "patch".to_string(), "minor".to_string(), "major".to_string() ].contains( &bump_level ) {
		println!( "Error: Invalid bump level {} should be patch/minor/major", &bump_level );
		bail!("Usage Error");
	};

	println!( "Pre Release Suffix : {}", pre_release_suffix );
	println!( "Bump Level         : {}", bump_level );
//	println!( "Allow Dirty        : {}", allow_dirty?"yes":"no" );
	println!( "Allow Dirty        : {}", if allow_dirty { "yes" } else { "no" } );
	println!( "Skip Git           : {}", if skip_git { "yes" } else { "no" } );


	let mut release = Release::new();

	release.set_pre_release_suffix( &pre_release_suffix );
	release.set_bump_level( &bump_level )?;
	release.set_allow_dirty( allow_dirty );
	release.set_skip_git( skip_git );

//	dbg!( &release );

	match release.run() {
		Ok(_) => {},
		Err(e) => { println!("Error: {}", e);},
	};
	Ok(())
}
