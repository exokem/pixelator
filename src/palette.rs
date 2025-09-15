use std::{collections::HashMap, str::FromStr};

use image::Rgba;
use ordered_float::OrderedFloat;

use crate::rgb::{LabaColor, RgbaColor};
pub struct PaletteColor {
	pub rgba: RgbaColor,
	pub laba: LabaColor,
}

pub struct Palette {
	pub colors: Vec<PaletteColor>
}

impl FromStr for Palette {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let colors = s.split(";")
			.map(|hex| {
				let rgba = RgbaColor::from_str(hex)?;
				let laba = LabaColor::from(rgba);

				Ok(PaletteColor{rgba, laba})
			}).collect::<Result<Vec<PaletteColor>, String>>()?;

		return Ok(Palette {
			colors: colors
		});
	}
}

impl Palette {
	pub fn nearest_match(&self, color: RgbaColor) -> &PaletteColor {
		let laba = LabaColor::from(color);

		let color = self.colors.iter()
			.min_by_key(|c| OrderedFloat(c.laba.delta_e_94(laba)))
			.unwrap();

		color
	}

	#[allow(dead_code)]
	pub fn nearest_match_map(&self, colors: Vec<RgbaColor>) -> HashMap<RgbaColor, &PaletteColor> {
		colors.into_iter().map(|rgba| {
			(rgba, self.nearest_match(rgba))
		}).collect()
	}

	pub fn nearest_match_map_raw(&self, colors: Vec<RgbaColor>) -> HashMap<Rgba<u8>, &PaletteColor> {
		colors.into_iter().map(|rgba| {
			(rgba.raw, self.nearest_match(rgba))
		}).collect()
	}
}