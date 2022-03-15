use macroquad::prelude::Color;

#[derive(Copy, Clone)]
pub struct Glyph {
	pub glyph: char,
	pub fg_color: Color,
	pub bg_color: Color,
}

impl Glyph {
	pub fn new(glyph: char, fg_color: Color, bg_color: Color) -> Self {
		Self {
			glyph,
			fg_color,
			bg_color,
		}
	}
}