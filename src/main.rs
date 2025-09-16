use std::{path::PathBuf};

use crate::{cli::{parse_cli, Commands}, image::{Image, SamplingFilter}, palette::Palette};

mod rgb;
mod image;
mod palette;
mod cli;

// "#ffffdd;#fcfd86;#fcc210;#e7fbfc;#d5fcf8;#b0d1d7;#c1c3ae;#666757;#8a8e7e;#369cb5;#1e9ab2;#17617a;#136a82;#273a48;#20292e;#101e27;#0b1419;#aa8656;#584936;#755e41;#432e19"

fn main() {
	let cli = parse_cli();

	let _ = match &cli.command {
		Commands::Reduce { palette, files } => {
			reduce(palette, files)
		}
		Commands::Scale  { 
			files, 
			sampling_filter, 
			divisor ,
			width
		} => {
			scale(files, *sampling_filter, *divisor, *width)
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

fn scale(files: &Vec<PathBuf>, filter: Option<SamplingFilter>, divisor: u32, width_opt: Option<u32>) -> Result<String, String> {

	for file in files {
		let mut image = Image::load(file)?;

		let mut width = image.width();

		if !width_opt.is_none() {
			width = width_opt.unwrap();
		}

		let height = width / (image.width() / image.height());

		image.resize(width / divisor, height / divisor, filter.unwrap_or(SamplingFilter::Nearest))?;

		let name = image.name()?;
		let saved_name = format!("{name}_resized");

		image.save_as(&saved_name)?;

		// TODO: print info
	}

	Ok(format!("Successfully resized {} images", files.len()))
}