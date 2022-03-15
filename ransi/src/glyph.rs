use crate::GLYPHS;
use macroquad::prelude::*;

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

// A function so I don't have to repeat myself between rendering nodes
pub fn draw_glyph(position: Vec2, font: Texture2D, glyph: Glyph, camera: &Camera2D) {
	// Make sure the tile position is within the bounds of the camera
	let mut screen_size = vec2(screen_width(), screen_height());
	if let Some(target) = camera.render_target {
		screen_size = vec2(target.texture.width(), target.texture.height());
	}
	if (position.y + 1.0) * 17.0 <= camera.target.y - (screen_size.y * 0.5)
	|| position.y * 17.0 >= camera.target.y + (screen_size.y * 0.5)
	|| (position.x + 1.0) * 10.0 <= camera.target.x - (screen_size.x * 0.5)
	|| position.x * 10.0 >= camera.target.x + (screen_size.x * 0.5) {
		return;
	}

	// Render the background
	draw_rectangle(
		position.x * 10.0,
		position.y * 17.0,
		10.0,
		17.0,
		glyph.bg_color,
	);

	// Render the foreground
	draw_texture_ex(
		font,
		position.x * 10.0,
		position.y * 17.0,
		glyph.fg_color,
		DrawTextureParams {
			dest_size: Some(vec2(9.0, 16.0)),
			source: Some(Rect {
				x: GLYPHS.iter()
					.position(|&x| x == glyph.glyph)
					.unwrap() as f32 * 9.0,
				y: 0.0,
				w: 9.0,
				h: 16.0,
			}),
			..Default::default()
		},
	);
}