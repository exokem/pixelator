use std::path::PathBuf;

use clap::{command, CommandFactory, Parser, Subcommand};

use crate::{image::SamplingFilter, palette::Palette};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,

	/// Specify a directory for bulk processing
	// #[arg(
	// 	short, long,
	// 	value_name = "DIRECTORY"
	// )]
	// directory: Option<PathBuf>,

	/// Specify a directory for output files
	#[arg(
		short, long,
		value_name = "DIRECTORY"
	)]
	pub output: Option<PathBuf>
}

impl Cli {
	#[allow(dead_code)]
	pub fn exit(message: &str) {
		println!("{}", message);
		let _ = Cli::command().print_help();
		std::process::exit(0);
	}
}

pub fn parse_cli() -> Cli {
	Cli::parse()
}

#[derive(Subcommand)]
pub enum Commands {
	/// Reduce colors in the target resource(s)
	Reduce {
		// TODO: color weighting???

		/// Provide a color palette for image color reduction
		#[arg(
			short, long,
			value_parser = clap::value_parser!(Palette),
			value_name = "PALETTE"
		)]
		palette: Palette,

		/// Specify individual files for processing
		#[arg(
			short, long,
			value_name = "FILES",
			num_args = 0..,
		)]
		files: Vec<PathBuf>,
	},

	/// Resize the target resource(s)
	Scale {
		// /// Specify a floating point scale factor 
		// #[arg(
		// 	short, long,
		// 	value_name = "FACTOR",
		// 	default_value_t = 1.0f32,
		// )]
		// factor: f32,

		/// Specify a scale factor based on a power of two
		#[arg(
			short, long,
			value_name = "DIVISOR",
			default_value_t = 1,
		)]
		divisor: u32,

		/// Specify the output width of the scaled image (aspect ratio is preserved)
		#[arg(
			short, long,
			value_name = "WIDTH"
		)]
		width: Option<u32>,

		/// Specify a sampling filter when scaling
		#[arg(
			short, long,
			value_name = "SCALE_METHOD",
		)]
		sampling_filter: Option<SamplingFilter>,

		/// Specify individual files for processing
		#[arg(
			short, long,
			value_name = "FILES",
			num_args = 0..,
		)]
		files: Vec<PathBuf>,
	}
}