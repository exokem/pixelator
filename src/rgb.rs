use std::{ops::Range, str::FromStr};

use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct XyzaColor {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub alpha: u8,
}

pub trait ToXyza {
	fn to_xyza(&self) -> XyzaColor;
}

#[derive(Debug, Clone, Copy)]
pub struct LabaColor {
	pub l: f32,
	pub a: f32,
	pub b: f32,
	pub alpha: u8,
}

impl LabaColor {
	pub fn delta_e_94(&self, y: LabaColor) -> f32 {
		const WL: f32 = 1.0;
		const WC: f32 = 1.0;
		const WH: f32 = 1.0;

		let xC1 = f32::sqrt( f32::powi( self.a, 2) + f32::powi( self.b, 2) );
		let xC2 = f32::sqrt( f32::powi( y.a, 2) + f32::powi( y.b, 2) );
		let mut xDL = y.l - self.l;
		let mut xDC = xC2 - xC1;
		let xDE = f32::sqrt( ( ( self.l - y.l ) * ( self.l - y.l ) )
				+ ( ( self.a - y.a ) * ( self.a - y.a ) )
				+ ( ( self.b - y.b ) * ( self.b - y.b ) ) );

		let mut xDH = ( xDE * xDE ) - ( xDL * xDL ) - ( xDC * xDC );
		if xDH > 0f32 {
			xDH = f32::sqrt( xDH );
		} else {
			xDH = 0f32;
		}

		let xSC = 1f32 + ( 0.045f32 * xC1 );
		let xSH = 1f32 + ( 0.015f32 * xC1 );

		xDL /= WL;
		xDC /= WC * xSC;
		xDH /= WH * xSH;

		return f32::sqrt( f32::powi(xDL, 2) + f32::powi(xDC, 2) + f32::powi(xDH, 2) );
	}
}

pub trait ToLaba {
	fn to_laba(&self) -> LabaColor;
}

pub struct LinearRgbaColor {
	r: f32,
	g: f32,
	b: f32,
	alpha: u8,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct RgbaColor {
	pub raw: Rgba<u8>,
}

const XYZ_LINEAR_GAMMA_THRESHOLD: f32 = 0.04045f32;
const XYZ_LINEAR_GAMMA_SLOPE: f32 = 12.92f32;
const XYZ_GAMMA_EXPONENT: f32 = 2.4;
const XYZ_SCALE_FACTOR: f32 = 1.055;
const XYZ_CONTINUOUS_OFFSET: f32 = 0.055;

fn linearize_rgb_value(value: u8) -> f32 {
	let mut normalized = value as f32 / 255f32;

	if XYZ_LINEAR_GAMMA_THRESHOLD < normalized {
		normalized = f32::powf((normalized + XYZ_CONTINUOUS_OFFSET) / XYZ_SCALE_FACTOR, XYZ_GAMMA_EXPONENT)
	} else {
		normalized = normalized / XYZ_LINEAR_GAMMA_SLOPE;
	}

	return normalized * 100f32;
}

impl RgbaColor {
	pub fn red(&self) -> &u8 {
		&self.raw.0[0]
	}

	pub fn green(&self) -> &u8 {
		&self.raw.0[1]
	}

	pub fn blue(&self) -> &u8 {
		&self.raw.0[2]
	}

	pub fn alpha(&self) -> &u8 {
		&self.raw.0[3]
	}

	pub fn linearize(&self) -> LinearRgbaColor {
		return LinearRgbaColor { 
			r: linearize_rgb_value(*self.red()), 
			g: linearize_rgb_value(*self.green()), 
			b: linearize_rgb_value(*self.blue()), 
			alpha: *self.alpha()
		}
	}
}

impl FromStr for RgbaColor {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim();

		if s.is_empty() {
			return Err("Color cannot be empty".to_string());
		}

		if s.get(0..1) == Some("#") {
			if s.len() != 7 && s.len() != 9 {
				return Err("RGB hex must be exactly 7 or 9 characters, e.g. #000000, #00000000".to_string());
			}
		} else {
			return Err("Invalid RGB color".to_string());
		}

		fn hex_to_u8(s: &str, range: Range<usize>) -> Result<u8, String> {
			let hex = s.get(range).ok_or("Out of bounds")?;

			return u8::from_str_radix(
				hex, 16
			).map_err(|e| e.to_string());
		}

		let red = hex_to_u8(s, 1..3)?;
		let green = hex_to_u8(s, 3..5)?;
		let blue = hex_to_u8(s, 5..7)?;
		let alpha = hex_to_u8(s, 7..9).unwrap_or(255);

		return Ok(RgbaColor {
			raw: Rgba([
				red, green, blue, alpha
			])
		});
	}
}

impl ToXyza for RgbaColor {
	fn to_xyza(&self) -> XyzaColor {
		let linear = &self.linearize();

		return XyzaColor { 
			x: (linear.r * 0.4124f32) + (linear.g * 0.3576f32) + (linear.b * 0.1805f32), 
			y: (linear.r * 0.2126f32) + (linear.g * 0.7152f32) + (linear.b * 0.0722f32), 
			z: (linear.r * 0.0193f32) + (linear.g * 0.1192f32) + (linear.b * 0.9505f32), 
			alpha: linear.alpha, 
		}
	}
}

// https://en.wikipedia.org/wiki/Illuminant_D65
const D65_X: f32 =  95.047f32;
const D65_Y: f32 = 100f32;
const D65_Z: f32 = 108.883f32;

const L_SCALE: f32 = 116f32;
const A_SCALE: f32 = 500f32;
const B_SCALE: f32 = 200f32;

const LAB_LINEAR_THRESHOLD: f32 = 0.008856451679f32;
const LAB_LINEAR_ADDEND: f32 = 4f32 / 29f32;

impl ToLaba for RgbaColor {
	fn to_laba(&self) -> LabaColor {
		let xyza = &self.to_xyza();

		fn lab_f(t: f32) -> f32 {
			if LAB_LINEAR_THRESHOLD < t {
				return f32::powf(t, 1f32 / 3f32);
			} else {
				return (7.787f32 * t) + LAB_LINEAR_ADDEND;
			}
		}

		let vx = lab_f(xyza.x / D65_X);
		let vy = lab_f(xyza.y / D65_Y);
		let vz = lab_f(xyza.z / D65_Z);

		return LabaColor {
			l: (L_SCALE * vy) - 16f32,
			a: A_SCALE * (vx - vy),
			b: B_SCALE * (vy - vz),
			alpha: xyza.alpha,
		};
	}
}


// fn rgba_to_xyza(rgba: Rgba<u8>) -> XyzaColor {
// 	fn xyzmed(c: u8) -> f32 {
// 		let mut f = c as f32 / 255f32;

// 		if 0.04045f32 < f {
// 			f = f32::powf((f + 0.055f32) / 1.055f32, 2.4f32)
// 		} else {
// 			f = f / 12.92f32;
// 		}

// 		return f * 100f32;
// 	}

// 	let [r, g, b, a] = rgba.0;

// 	let fr = xyzmed(r);
// 	let fg = xyzmed(g);
// 	let fb = xyzmed(b);

// 	return XyzaColor { 
// 		x: (fr * 0.4124f32) + (fg * 0.3576f32) + (fb * 0.1805f32), 
// 		y: (fr * 0.2126f32) + (fg * 0.7152f32) + (fb * 0.0722f32), 
// 		z: (fr * 0.0193f32) + (fg * 0.1192f32) + (fb * 0.9505f32), 
// 		alpha: a, 
// 	}
// }

// https://www.easyrgb.com/en/math.php
fn xyza_to_cie(xyza: XyzaColor) -> LabaColor {

	const RX: f32 = 95.047f32;
	const RY: f32 = 100.000f32;
	const RZ: f32 = 108.883f32;

	fn cielmed(c: f32, rc: f32) -> f32 {
		let c = c / rc;

		if 0.008856f32 < c {
			return f32::powf(c, 1f32 / 3f32);
		} else {
			return (7.787f32 * c) + (16f32 / 116f32);
		}
	}

	let vx = cielmed(xyza.x, RX);
	let vy = cielmed(xyza.y, RY);
	let vz = cielmed(xyza.z, RZ);

	return LabaColor {
		l: (116f32 * vy) - 16f32,
		a: 500f32 * (vx - vy),
		b: 200f32 * (vy - vz),
		alpha: xyza.alpha,
	};
}