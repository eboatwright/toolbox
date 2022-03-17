use macroquad::prelude::*;
use crate::GLYPHS;

// Holds data about a glyph for rendering
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

	// Render the glyph with the texture, at the position
	pub fn render(&self, font_texture: Texture2D, position: Vec2) {
		// Draw the background
		draw_rectangle(
			position.x * 9.0,
			position.y * 16.0,
			9.0,
			16.0,
			self.bg_color,
		);

		// Draw the foreground
		draw_texture_ex(
			font_texture,
			// Multiply the positions so that they render in tile coordinates
			position.x * 9.0,
			position.y * 16.0,
			self.fg_color,
			DrawTextureParams {
				dest_size: Some(vec2(9.0, 16.0)),
				source: Some(Rect {
					// Loop through the glyphs, and find the index of the one we need to render
					x: GLYPHS
						.iter()
						.position(|&c| c == self.glyph)
						.unwrap() as f32 * 9.0, // Then multiply it by 9 so that we get the source X position of the glyph
					y: 0.0,
					w: 9.0,
					h: 16.0,
				}),
				..Default::default()
			},
		);
	}
}