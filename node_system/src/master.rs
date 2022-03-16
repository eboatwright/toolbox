use crate::Context;

// A *very* simple function for handling *most* of the game loop
pub struct Master {
	pub render_order: Vec<&'static str>,
	pub init_systems: Vec<fn(&mut Context)>,
	pub update_systems: Vec<fn(&mut Context)>,
	pub render_systems: Vec<fn(&Context, &str)>,
	pub context: Context,
}

impl Master {
	pub fn init(&mut self) {
		self.init_systems
			.iter()
			.for_each(|init| init(&mut self.context));
	}

	pub fn update(&mut self) {
		self.update_systems
			.iter()
			.for_each(|update| update(&mut self.context));
	}

	pub fn render(&self) {
		self.render_order.iter().for_each(|layer| {
			self.render_systems
				.iter()
				.for_each(|render| render(&self.context, layer));
		});
	}
}