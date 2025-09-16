use std::{path::PathBuf};
use clap::{CommandFactory, Parser, Subcommand};

use crate::{image::{Image, SamplingFilter}, palette::Palette};

mod rgb;
mod image;
mod palette;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,

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
	output: Option<PathBuf>
}

impl Cli {
	#[allow(dead_code)]
	pub fn exit(message: &str) {
		println!("{}", message);
		let _ = Cli::command().print_help();
		std::process::exit(0);
	}
}

#[derive(Subcommand)]
enum Commands {
	/// Reduce colors in the target resource(s)
	Reduce {
		// #[arg(
		// 	short, long,
		// 	value_name = "THRESHOLD",
		// 	default_value_t = 50.0,
		// )]
		// threshold: f32,

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

		// /// Specify how pixels are merged when downscaling
		// #[arg(
		// 	short, long,
		// 	value_name = "METHOD",
		// 	value_enum,
		// 	default_value_t = MergeMethod::Average,
		// )]
		// merge: MergeMethod

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

fn main() {
	let cli = Cli::parse();

// "#ffffdd;#fcfd86;#fcc210;#e7fbfc;#d5fcf8;#b0d1d7;#c1c3ae;#666757;#8a8e7e;#369cb5;#1e9ab2;#17617a;#136a82;#273a48;#20292e;#101e27;#0b1419;#aa8656;#584936;#755e41;#432e19"

	let _ = match &cli.command {
		Commands::Reduce { palette, files } => {
			reduce(palette, files)
		}
		Commands::Scale  { files, sampling_filter, divisor } => {
			scale(files, *sampling_filter, *divisor)
		}
	};
}

fn reduce(palette: &Palette, files: &Vec<PathBuf>) -> Result<String, String> {
	for file in files {
		let mut image = Image::load(file)?;

		image.apply_palette(palette);

		let name = image.name()?;

		image.save_as(&format!("{name}_reduced"))?;
	}

	Ok(format!("Successfully applied palette to {} images", files.len()))
}

fn scale(files: &Vec<PathBuf>, filter: Option<SamplingFilter>, divisor: u32) -> Result<String, String> {

	for file in files {
		let mut image = Image::load(file)?;

		image.resize(image.width() / divisor, image.height() / divisor, filter.unwrap_or(SamplingFilter::Nearest))?;

		let name = image.name()?;

		image.save_as(&format!("{name}_resized"))?;
	}

	Ok("".to_string())
}