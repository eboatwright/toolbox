use macroquad::prelude::*;
use crate::GLYPHS;
use viewport::Viewport;

// Holds data about a glyph for rendering
#[derive(Clone, Default)]
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
	pub fn render(&self, font_texture: Texture2D, mut position: Vec2, viewport: Option<&Viewport>) {
		// Convert to pixel position
		position = position * vec2(9.0, 16.0);

		// If a viewport was provided, check and see if the tile to render is in the bounds before rendering
		if let Some(viewport) = viewport {
			if position.y >= viewport.bottom()
			|| position.y + 16.0 <= viewport.top()
			|| position.x >= viewport.right()
			|| position.x + 9.0 <= viewport.left() {
				return;
			}
		}

		// Draw the background
		draw_texture_ex(
			font_texture,
			// Multiply the positions so that they render in tile coordinates
			position.x,
			position.y,
			self.bg_color,
			DrawTextureParams {
				dest_size: Some(vec2(9.0, 16.0)),
				source: Some(Rect {
					x: 1962.0, // The index of █
					y: 0.0,
					w: 9.0,
					h: 16.0,
				}),
				..Default::default()
			},
		);

		// Just ignore if it's a space
		if self.glyph != ' ' {
			// Draw the foreground
			draw_texture_ex(
				font_texture,
				// Multiply the positions so that they render in tile coordinates
				position.x,
				position.y,
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
}