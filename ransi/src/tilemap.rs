use viewport::Viewport;
use macroquad::prelude::*;
use crate::Glyph;

// Holds data about a tile
#[derive(Clone, Default)]
pub struct Tile {
	pub glyph: Glyph,
	// Tags for the Tile.
	// For example, a wall would have a "solid" tag
	pub tags: Vec<&'static str>,
}

impl Tile {
	pub fn new(glyph: Glyph, tags: Vec<&'static str>) -> Self {
		Self {
			glyph,
			tags,
		}
	}

	pub fn has_tag(&self, tag: &'static str) -> bool {
		self.tags.contains(&tag)
	}
}

pub struct Tilemap {
	// A sort of palette for the tiles
	pub tilemap: Vec<Tile>,
	// A tile (in this case a usize) is just an index to the tilemap of which tile to use
	pub tiles: Vec<Vec<Vec<usize>>>,

	// Min and max Z levels to render
	pub min_z_render: usize,
	pub max_z_render: usize,
}

impl Tilemap {
	pub fn new(tilemap: Vec<Tile>, tiles: Vec<Vec<Vec<usize>>>, min_z_render: usize, max_z_render: usize) -> Self {
		Self {
			tilemap,
			tiles,

			// By default, render all z levels
			min_z_render,
			max_z_render,
		}
	}

	// Return the Tile from the tilemap at the x, y, and z position
	pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Tile {
		self.tilemap[self.tiles[z][y][x]].clone()
	}

	pub fn render(&self, font_texture: Texture2D, viewport: Option<&Viewport>) {
		// Loop through the tiles and render them
		for z in self.min_z_render..usize::min(self.max_z_render, self.tiles.len()) {
			for y in 0..self.tiles[z].len() {
				for x in 0..self.tiles[z][y].len() {
					self.tilemap[self.tiles[z][y][x] as usize].glyph.render(font_texture, vec2(x as f32, y as f32), viewport);
				}
			}
		}
	}
}