use crate::GLYPHS;

use node_system::*;

use macroquad::prelude::*;

use std::any::Any;

pub struct GlyphNode {
	// All of the mandatory properties (required by the node system)
	type_id: &'static str,
	path: String,
	position: Vec2,

	// All of the rendering properties
	pub font: Texture2D,
	pub glyph: char,
	pub fg_color: Color,
	pub bg_color: Color,
}

impl GlyphNode {
	pub fn new(path: String, position: Vec2, font: Texture2D, glyph: char, fg_color: Color, bg_color: Color) -> Self {
		Self {
			type_id: "glyph",
			path,
			position,

			font,
			glyph,
			fg_color,
			bg_color,
		}
	}
}

node!(GlyphNode);

pub fn glyph_render_system(context: &Context) {
	// Iterate through each glyph node, and downcast them all to GlyphNode
	for node in context.tree.get_nodes_by_type_id("glyph").iter().map(|node| node.downcast_ref::<GlyphNode>().unwrap()) {
		// Get the *global* position of the node
		let position = context.tree.get_node_position(node.get_path());

		// Render the background
		draw_rectangle(
			position.x * 10.0,
			position.y * 17.0,
			10.0,
			17.0,
			node.bg_color,
		);

		// Render the foreground
		draw_texture_ex(
			node.font,
			position.x * 10.0,
			position.y * 17.0,
			node.fg_color,
			DrawTextureParams {
				dest_size: Some(vec2(9.0, 16.0)),
				source: Some(Rect {
					x: GLYPHS.iter()
						.position(|&x| x == node.glyph)
						.unwrap() as f32 * 9.0,
					y: 0.0,
					w: 9.0,
					h: 16.0,
				}),
				..Default::default()
			},
		);
	}
}