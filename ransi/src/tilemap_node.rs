// See glyph_node.rs for more explanation, as this uses alot of the same things :)

// TODO: min and max layers (for rendering)
// TODO: screen space culling

use crate::constants::colors;
use crate::Glyph;
use crate::draw_glyph;

use node_system::*;

use macroquad::prelude::*;

use std::any::Any;

// Stores data about a tile in the tileset
#[derive(Clone)]
pub struct Tile {
	pub tags: Vec<String>,
	pub glyph: Glyph,
}

impl Tile {
	pub fn new() -> Self {
		Self {
			tags: vec![],
			glyph: Glyph::new('?', colors::BLACK, colors::DARK_PINK),
		}
	}

	pub fn has_tag(&self, tag: String) -> bool {
		self.tags.contains(&tag)
	}

	pub fn add_tag(&mut self, tag: String) {
		if !self.has_tag(tag.clone()) {
			self.tags.push(tag);
		}
	}

	// Removes all instances of the tag
	pub fn remove_tag(&mut self, tag: String) {
		if self.has_tag(tag.clone()) {
			self.tags = self.tags.clone()
				.into_iter()
				.filter(|x| *x != tag)
				.collect::<Vec<_>>();
		}
	}
}

pub struct TilemapNode {
	type_id: &'static str,
	path: String,
	position: Vec3,

	// Used for X, Y, Z tiles
	pub tileset: Vec<Tile>,
	pub tiles: Vec<Vec<Vec<u16>>>,
	pub font: Texture2D,
}

impl TilemapNode {
	pub fn new(path: String, position: Vec3, tileset: Vec<Tile>, tiles: Vec<Vec<Vec<u16>>>, font: Texture2D) -> Self {
		Self {
			type_id: "tilemap",
			path,
			position,

			tileset,
			tiles,
			font,
		}
	}

	// Get's the corresponding Tile from the tileset, that is at the x, y, and z position
	pub fn get_tile(&self, x: usize, y: usize, z: usize) -> &Tile {
		&self.tileset[self.tiles[z][y][x] as usize]
	}

	pub fn get_tile_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Tile {
		&mut self.tileset[self.tiles[z][y][x] as usize]
	}
}

node!(TilemapNode);

pub fn tilemap_render_system(context: &Context, camera: &Camera2D) {
	for node in context.tree.get_nodes_by_type_id("glyph").iter().map(|node| node.downcast_ref::<TilemapNode>().unwrap()) {
		let position = context.tree.get_node_position(node.get_path());
		for z in 0..node.tiles.len() {
			for y in 0..node.tiles[z].len() {
				for x in 0..node.tiles[z][y].len() {
					draw_glyph(position.truncate() + vec2(x as f32, y as f32), node.font, node.tileset[node.tiles[z][y][x] as usize].glyph, camera);
				}
			}
		}
	}
}