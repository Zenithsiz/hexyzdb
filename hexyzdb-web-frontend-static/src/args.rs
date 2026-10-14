//! Command line arguments

use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
	/// Directory to output to
	#[clap(long)]
	pub output_dir: PathBuf,
}
