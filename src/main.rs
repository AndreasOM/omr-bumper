

use omr_bumper::Release;
pub fn main() -> anyhow::Result<()> {

	let release = Release::new();

	match release.run() {
		Ok(_) => {},
		Err(e) => { println!("Error: {}", e);},
	};
	Ok(())
}
