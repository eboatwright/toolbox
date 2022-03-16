use macroquad::prelude::*;

// This holds information about the screen
pub struct Viewport {
	pub width: f32,
	pub height: f32,
	pub render_target: RenderTarget,
	// This is for calculating mouse position
	pub(crate) aspect_diff: f32,
	pub camera: Camera2D,
}

impl Viewport {
	pub fn new(width: f32, height: f32) -> Viewport {
		let render_target = render_target(width as u32, height as u32);
		let viewport = Viewport {
			width,
			height,
			render_target,
			aspect_diff: 1.0,
			camera: Camera2D {
		        // I don't know why this is like this, but it's just Macroquad's scaling. Yoinked from TanTanDev
		        zoom: vec2(1.0 / width * 2.0, 1.0 / height * 2.0),
		        render_target: Some(render_target),
		        ..Default::default()
		    },
		};

		// This just sets it here, so I don't have to :)
		set_camera(&viewport.camera);

		viewport
	}

	pub fn render(&mut self) {
		// Go to screen space
		set_default_camera();

		// Calculate aspect differences (Also yoinked from TanTanDev and modified)
	    let game_diff = vec2(
	        screen_width() / self.width,
	        screen_height() / self.height,
	    );
	    let aspect_diff = game_diff.x.min(game_diff.y);
	    self.aspect_diff = aspect_diff;

	    let scaled_game_size = vec2(
	        self.width * aspect_diff,
	        self.height * aspect_diff,
	    );

	    let padding = vec2(
	        (screen_width() - scaled_game_size.x) * 0.5,
	        (screen_height() - scaled_game_size.y) * 0.5,
	    );

	    clear_background(BLACK);

	    draw_texture_ex(
	        self.render_target.texture,
	        padding.x.round(),
	        padding.y.round(),
	        WHITE,
	        DrawTextureParams {
	            dest_size: Some(scaled_game_size.floor()),
	            ..Default::default()
	        },
	    );

	    set_camera(&self.camera);
	}

	pub fn mouse_position(&self) -> Vec2 {
		let mut mouse_pos = mouse_position();

		// Calculate the mouse position with screen scaling into account
		mouse_pos.0 = (mouse_pos.0 - screen_width() * 0.5) / self.aspect_diff + self.camera.target.x;
		mouse_pos.1 = (mouse_pos.1 - screen_height() * 0.5) / self.aspect_diff + self.camera.target.y;

		Vec2::from(mouse_pos)
	}

	pub fn top(&self) -> f32 { self.camera.target.y - self.height * 0.5 }
	pub fn bottom(&self) -> f32 { self.camera.target.y + self.height * 0.5 }
	pub fn left(&self) -> f32 { self.camera.target.x - self.width * 0.5 }
	pub fn right(&self) -> f32 { self.camera.target.x + self.width * 0.5 }
}