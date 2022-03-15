use macroquad::prelude::Camera;
use crate::Context;

// A *very* simple function for handling *most* of the game loop
pub struct Master {
	pub init_systems: Vec<fn(&mut Context, &mut dyn Camera)>,
	pub update_systems: Vec<fn(&mut Context, &mut dyn Camera)>,
	pub render_systems: Vec<fn(&Context, &dyn Camera)>,
	pub context: Context,
}

impl Master {
	pub fn init(&mut self, camera: &mut dyn Camera) {
		self.init_systems
			.iter()
			.for_each(|init| init(&mut self.context, camera));
	}

	pub fn update(&mut self, camera: &mut dyn Camera) {
		self.update_systems
			.iter()
			.for_each(|update| update(&mut self.context, camera));
	}

	pub fn render(&self, camera: &dyn Camera) {
		self.render_systems
			.iter()
			.for_each(|render| render(&self.context, camera));
	}
}