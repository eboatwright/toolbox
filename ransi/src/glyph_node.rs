use crate::draw_glyph;
use crate::Glyph;

use node_system::*;

use macroquad::prelude::*;

use std::any::Any;

pub struct GlyphNode {
	// All of the mandatory properties (required by the node system)
	type_id: &'static str,
	path: String,
	position: Vec3,

	// All of the rendering properties
	pub font: Texture2D,
	pub glyph: Glyph,
}

impl GlyphNode {
	pub fn new(path: String, position: Vec3, font: Texture2D, glyph: Glyph) -> Self {
		Self {
			type_id: "glyph",
			path,
			position,

			font,
			glyph,
		}
	}
}

node!(GlyphNode);

pub fn glyph_render_system(context: &Context) {
	// Iterate through each glyph node, and downcast them all to GlyphNode
	for node in context.tree.get_nodes_by_type_id("glyph").iter().map(|node| node.downcast_ref::<GlyphNode>().unwrap()) {
		// Get the *global* position of the node
		let position = context.tree.get_node_position(node.get_path());
		draw_glyph(position.truncate(), node.font, node.glyph, &context.camera_holder.camera2d.unwrap());
	}
}