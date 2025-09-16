use std::{path::PathBuf};

use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

use crate::{palette::Palette, rgb::RgbaColor};

pub struct Image {
	pub path: PathBuf,
	pub raw: DynamicImage
}

impl Image {
	#[allow(dead_code)]
	pub fn height(&self) -> u32 {
		self.raw.height()
	}

	#[allow(dead_code)]
	pub fn width(&self) -> u32 {
		self.raw.width()
	}

	pub fn name(&self) -> Result<&str, String> {
		self.path.file_stem().ok_or("Image missing file name")?
			.to_str().ok_or("Invalid file name".to_string())
	}

	pub fn collect_unique_colors(&self) -> Vec<RgbaColor> {
		let mut vec: Vec<Rgba<u8>> = self.raw.pixels()
			.map(|(_, _, px)| px)
			.collect();

		vec.sort_by_key(|px| px.0);
		vec.dedup();

		vec.iter().map(|c| RgbaColor { raw: *c }).collect()
	}

	pub fn apply_palette(&mut self, palette: &Palette) {
		let colors = self.collect_unique_colors();

		let map = palette.nearest_match_map_raw(colors);

		for x in 0..self.raw.width() {
			for y in 0..self.raw.height() {
				let px = self.raw.get_pixel(x, y);
				self.raw.put_pixel(x, y, map.get(&px).unwrap().rgba.raw);
			}
		}
	}

	pub fn load(path: &PathBuf) -> Result<Self, String> {
		let img = ImageReader::open(path)
			.map_err(|err| err.to_string())?
			.decode().map_err(|err| err.to_string())?;

		return Ok(Image {
			path: path.to_path_buf(),
			raw: img
		})
	}

	pub fn save_as(&self, name: &str) -> Result<(), String> {
		let extension = self.path.extension()
			.ok_or("Image missing extension")?
			.to_str().ok_or("Invalid image extension")?;
		// let name = self.path.file_stem().ok_or("Image missing file name")?;

		self.raw.save(self.path.with_file_name(format!("{name}.{extension}")))
			.map_err(|e| e.to_string())?;

		Ok(())
	}
}