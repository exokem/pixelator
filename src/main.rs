use std::{path::PathBuf, str::FromStr};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use crate::{image::Image, palette::Palette};

mod rgb;
mod image;
mod palette;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,

	/// Specify a single file for processing
	#[arg(
		short, long,
		value_name = "FILE"
	)]
	file: Option<PathBuf>,

	/// Specify a directory for bulk processing
	#[arg(
		short, long,
		value_name = "DIRECTORY"
	)]
	directory: Option<PathBuf>,

	/// Specify a directory for output files
	#[arg(
		short, long,
		value_name = "DIRECTORY"
	)]
	output: PathBuf
}

#[derive(Subcommand)]
enum Commands {
	/// Reduce colors in the target resource(s)
	Reduce {
		/// Specify the percentage threshold for color replacement
		#[arg(
			short, long,
			value_name = "THRESHOLD",
			default_value_t = 50.0,
		)]
		threshold: f32,

		// #[arg(
		// 	value_parser = clap::value_parser!(RgbColor),
		// 	num_args = 3..,
		// )]
		// colors: Vec<RgbColor>,
	},

	/// Resize the target resource(s)
	Scale {
		/// Specify a floating point scale factor 
		#[arg(
			short, long,
			value_name = "FACTOR",
			default_value_t = 1.0f32,
		)]
		factor: f32,

		/// Specify a scale factor based on a power of two
		#[arg(
			short, long,
			value_name = "POWER",
			default_value_t = 0,
		)]
		power: i16,

		/// Specify how pixels are merged when downscaling
		#[arg(
			short, long,
			value_name = "METHOD",
			value_enum,
			default_value_t = MergeMethod::Average,
		)]
		merge: MergeMethod
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MergeMethod {
	Average
}

fn main() {
	let cli = Cli::parse();

	// Either a file or directory is required
	if cli.file.is_none() && cli.directory.is_none() {
		Cli::command().print_help().unwrap();
		std::process::exit(0);
	}

	let palette = Palette::from_str("#ffffdd;#fcfd86;#fcc210;#17617a;#273a48;#aa8656;#432e19").unwrap();

	// Process file
	if cli.directory.is_none() {
		let path: PathBuf = cli.file.unwrap();
		let mut img = Image::load(path).unwrap();

		img.apply_palette(&palette);
		let _ = img.raw.save("test.png");

	}

	// match &cli.command {
	// 	Commands::Reduce { } => {

	// 	}
	// 	Commands::Scale  { factor, power, merge } => {

	// 	}
	// }
}
