// See glyph.rs for more details :)

use viewport::Viewport;
use crate::Glyph;
use macroquad::prelude::*;

#[derive(Clone, Default)]
pub struct Text {
	pub string: String,
	pub fg_color: Color,
	pub bg_color: Color,
}

impl Text {
	pub fn new(string: String, fg_color: Color, bg_color: Color) -> Self {
		Self {
			string,
			fg_color,
			bg_color,
		}
	}

	pub fn render(&self, font_texture: Texture2D, position: Vec2, viewport: Option<&Viewport>) {
		let mut offset = Vec2::ZERO;
		for c in self.string.chars() {
			if c == '\n' {
				// If the char is a new line, then reset the x offset and increase the y offset
				offset.x = 0.0;
				offset.y += 1.0;
				continue;
			}
			// Create a new glyph and render it
			Glyph::new(c, self.fg_color, self.bg_color).render(font_texture, position + offset, viewport);
			offset.x += 1.0;
		}
	}
}