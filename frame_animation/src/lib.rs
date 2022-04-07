// A simple library for frame based animation

#[derive(Copy, Clone)]
pub struct AnimationFrame {
	pub index: usize,
	pub duration: f32,
}

impl AnimationFrame {
	pub fn new(index: usize, duration: f32) -> Self {
		Self {
			index,
			duration,
		}
	}
}

#[derive(Clone)]
pub struct Animation {
	pub name: &'static str,
	pub frames: Vec<AnimationFrame>,
	// Wether the Animation can be interrupted by change_animation or not
	pub dont_interrupt: bool,
}

impl Animation {
	pub fn new(name: &'static str, frames: Vec<AnimationFrame>, dont_interrupt: bool) -> Self {
		Self {
			name,
			frames,
			dont_interrupt,
		}
	}
}

impl Default for Animation {
	fn default() -> Self {
		Self {
			name: "animation",
			frames: vec![],
			dont_interrupt: false,
		}
	}
}

#[derive(Clone, Default)]
pub struct Animator {
	pub animations: Vec<Animation>,
	pub current_animation_index: usize,
	pub current_frame_index: usize,
	pub timer: f32,
	pub dont_interrupt: bool,
}

impl Animator {
	pub fn new(animations: Vec<Animation>) -> Self {
		Self {
			animations,
			current_animation_index: 0,
			current_frame_index: 0,
			timer: 0.0,
			dont_interrupt: false,
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		self.timer -= delta_time;
		if self.timer <= 0.0 {
			self.timer = self.animations[self.current_animation_index].frames[self.current_frame_index].duration;
			self.current_frame_index += 1;
			// If the animation frame is out of bounds
			if self.current_frame_index >= self.animations[self.current_animation_index].frames.len() {
				if self.dont_interrupt {
					// If animation can't be interrupted, it can't loop either (This is here because if it's not, it looks *really* weird)
					self.dont_interrupt = false;
					self.current_frame_index -= 1;
				} else {
					// If it's a regular animation, just loop it
					self.current_frame_index = 0;
				}
			}
		}
	}

	pub fn change_animation(&mut self, name: &'static str) {
		// Return if the animation is already being played, or the current animation can't be interrupted
		if self.dont_interrupt
		|| self.animations[self.current_animation_index].name == name {
			return;
		}

		for (i, animation) in self.animations.iter().enumerate() {
			// Find the animation to switch to
			if animation.name == name {
				// Set the animation index, and dont_interrupt
				self.current_animation_index = i;
				self.dont_interrupt = animation.dont_interrupt;

				// Reset timers
				self.current_frame_index = 0;
				self.timer = 0.0;

				// Return because we found the animaiton
				return;
			}
		}
	}

	// Returns the current frame index
	pub fn get_frame(&self) -> f32 {
		self.animations[self.current_animation_index].frames[self.current_frame_index].index as f32
	}

	// A pseudo-code example of using this:
	/*
	draw_texture(texture, source: Rect {
		x: animator.get_frame() * WIDTH,
		y: 0.0,
		w: WIDTH,
		h: HEIGHT,
	});
	*/
}