use std::{str::FromStr};

use ordered_float::OrderedFloat;

use crate::rgb::{LabaColor, RgbaColor};

#[derive(Clone)]
pub struct PaletteColor {
	pub rgba: RgbaColor,
	pub laba: LabaColor,
}

impl From<RgbaColor> for PaletteColor {
	fn from(value: RgbaColor) -> Self {
		let laba = LabaColor::from(value);

		PaletteColor { rgba: value, laba: laba }
	}
}

impl FromStr for PaletteColor {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let rgba = RgbaColor::from_str(s)?;

		Ok(PaletteColor::from(rgba))
	}
}

#[derive(Clone)]
pub struct Palette {
	pub colors: Vec<PaletteColor>
}

impl From<&Vec<RgbaColor>> for Palette {
	fn from(value: &Vec<RgbaColor>) -> Self {
		let colors = value.into_iter().map(|rgba| {
				PaletteColor{rgba: *rgba, laba: LabaColor::from(*rgba)}
			}).collect();

		Palette { colors }
	}
}

impl FromStr for Palette {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let colors = s.split(";")
			.map(|hex| PaletteColor::from_str(hex))
			.collect::<Result<Vec<PaletteColor>, String>>()?;

		Ok(Palette { colors })
	}
}

impl TryFrom<&Vec<&str>> for Palette {
	type Error = String;
	
	fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
		let colors = value.iter()
			.map(|hex| PaletteColor::from_str(hex))
			.collect::<Result<Vec<PaletteColor>, String>>()?;

		Ok(Palette { colors })
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
}