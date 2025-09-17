use std::{collections::HashMap, path::PathBuf, time::Duration};

use console::{style, StyledObject, Term};
use ::image::Rgba;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use crate::{cli::{parse_cli, Commands}, image::{Image, SamplingFilter}, palette::{Palette, PaletteColor}, rgb::RgbaColor};

mod rgb;
mod image;
mod palette;
mod cli;

// "#ffffdd;#fcfd86;#fcc210;#e7fbfc;#d5fcf8;#b0d1d7;#c1c3ae;#666757;#8a8e7e;#369cb5;#1e9ab2;#17617a;#136a82;#273a48;#20292e;#101e27;#0b1419;#aa8656;#584936;#755e41;#432e19"

fn print_heading(s: &str) {

	let term = Term::stdout();

	let (w, _) = term.size();

	let xtra: String = std::iter::repeat("-").take(w as usize - s.len()).collect();
	println!(">-----< {} >{}<", style(s).cyan(), xtra)
}

fn main() {
	let cli = parse_cli();

	println!();

	let _ = match &cli.command {
		Commands::Reduce { palette, files } => {
			print_heading(&format!("{} reducer {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
			reduce(palette, files)
		}
		Commands::Scale  { 
			files, 
			sampling_filter, 
			divisor ,
			width
		} => {
			print_heading(&format!("{} scaler {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
			scale(files, *sampling_filter, *divisor, *width)
		}
	};

	println!();
	print_heading("finished");
	println!();
}

fn format_step(i: u8, total: u8) -> StyledObject<String> {
	style(format!("[{}/{}]", i, total)).bold().dim()
}

fn new_spinner(message: &str, step: u8, total_steps: u8) -> ProgressBar {
	let spinner_style = ProgressStyle::with_template(
		"{prefix:.bold.dim} {spinner} {wide_msg}"
	).unwrap().tick_chars("|/-\\|\r");

	let spinner = ProgressBar::new_spinner()
		.with_message(format!("{} {}", format_step(step, total_steps), message))
		.with_style(spinner_style.clone());
	spinner.enable_steady_tick(Duration::from_millis(100));

	spinner
}

pub fn new_progress_bar(message: &str, length: u64, step: u8, total_steps: u8) -> ProgressBar {
	let progress = ProgressBar::new(length)
		.with_message(message.to_string())
		.with_prefix(format!("{}", format_step(step, total_steps)))
		.with_finish(indicatif::ProgressFinish::Abandon);

	progress.set_style(ProgressStyle::with_template(" {prefix} {bar:40.cyan/blue} {msg} {pos:>7}/{len:7} [{elapsed}]")
		.unwrap()
		.progress_chars(">>-"));

	progress
}

fn load_image(path: &PathBuf, step: u8, total_steps: u8) -> Result<Image, String> {
	let path_str = path.to_str().ok_or("Unable to stringify file name")?;
	let name = style(path_str).cyan();
	let message = &format!("Loading {}", name);

	let spinner = new_spinner(message, step, total_steps);
	let image = Image::load(path)?;
	spinner.finish_with_message(
		format!(
			"{} Loaded {} [{}x{}] ({} pixels)", 
			format_step(step, total_steps), 
			name,
			image.width(),
			image.height(),
			style((image.width() * image.height()).to_string()).cyan()
		)
	);

	Ok(image)
}

fn sort_pixels(image: &Image, step: u8, total_steps: u8) -> Vec<Rgba<u8>> {
	let progress = new_progress_bar("Sorting pixels", image.pixels() as u64, step, total_steps);

	let pixels = image.sort_raw_pixel_colors(Some(|_| progress.inc(1)));

	pixels
}

fn deduplicate_pixel_colors(pixels: &mut Vec<Rgba<u8>>, step: u8, total_steps: u8) -> Vec<RgbaColor> {
	pixels.dedup();
	let progress = new_progress_bar("Deduplicating colors", pixels.len() as u64, step, total_steps);

	pixels.iter()
		.progress_with(progress)
		.map(|px| RgbaColor { raw: *px })
		.collect()
}

fn generate_palette_mapping(colors: Vec<RgbaColor>, palette: &Palette, step: u8, total_steps: u8) -> HashMap<Rgba<u8>, &PaletteColor> {
	let progress = new_progress_bar(&format!("Remapping colors ({} color palette)", palette.colors.len()), colors.len() as u64, step, total_steps);

	colors.into_iter()
		.progress_with(progress)
		.map(|rgba| {
			(rgba.raw, palette.nearest_match(rgba))
		}).collect()
}

fn apply_palette_mapping(image: &mut Image, mapping: HashMap<Rgba<u8>, &PaletteColor>, step: u8, total_steps: u8) {
	let progress = new_progress_bar("Applying color mapping", image.pixels() as u64, step, total_steps);

	for x in 0..image.width() {
		for y in 0..image.height() {
			let px = image.get_pixel_raw(x, y);
			image.put_pixel_raw(x, y, mapping.get(&px).unwrap().rgba.raw);
			progress.inc(1);
		}
	}
}

fn save_image_as(image: &Image, name: &str, step: u8, total_steps: u8) -> Result<(), String> {
	let formatted_name = format!("{}", style(format!("{name}.png")).cyan());

	let spinner = new_spinner(&format!("Saving image as {}", formatted_name), step, total_steps);
	image.save_as(&format!("{name}"))?;
	spinner.finish_with_message(format!("{} Saved image as {}", format_step(step, total_steps), formatted_name));

	Ok(())
}

fn reduce(palette: &Palette, files: &Vec<PathBuf>) -> Result<String, String> {
	for file in files {
		const STEPS: u8 = 6;

		let mut image = load_image(file, 1, STEPS)?;

		let mut sorted_pixels = sort_pixels(&image, 2, STEPS);

		let unique_colors = deduplicate_pixel_colors(&mut sorted_pixels, 3, STEPS);

		let mapped_colors = generate_palette_mapping(unique_colors, palette, 4, STEPS);

		apply_palette_mapping(&mut image, mapped_colors, 5, STEPS);

		let name = image.name()?;

		save_image_as(&image, &format!("{name}_reduced"), 6, STEPS)?;

		// let test_spinner = ProgressBar::new_spinner()
		// 	.with_message("2222")
		// 	.with_style(spinner_style.clone());
		// test_spinner.enable_steady_tick(Duration::from_millis(100));


		// sleep(Duration::from_millis(2000));
		// test_spinner.finish();


		// let unique_spinner = ProgressBar::new_spinner()
		// 	.with_message("Collecting unique colors");
		// multi.add(unique_spinner.clone());

		// // println!("{}", style("Collecting unique colors").bold());
		// let unique_colors = image.collect_unique_colors();
		// unique_spinner.finish_with_message(format!("Collected {} unique colors", unique_colors.len()));

		// image.apply_palette(palette);


		// image.save_as(&format!("{name}_reduced"))?;
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