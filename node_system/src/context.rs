// use std::any::Any;
use macroquad::prelude::*;
use crate::Tree;
use crate::ResourceContainer;

// A work-around to having a 2d or 3d camera
pub struct CameraHolder {
	pub two_d: Camera2D,
	pub three_d: Camera3D,
}

impl CameraHolder {
	pub fn camera2d(camera: Camera2D) -> Self {
		Self {
			two_d: camera,
			three_d: Camera3D::default(),
		}
	}

	pub fn camera3d(camera: Camera3D) -> Self {
		Self {
			two_d: Camera2D::default(),
			three_d: camera,
		}
	}

	pub fn get_2d(&self) -> &Camera2D {
		&self.two_d
	}

	pub fn get_3d(&self) -> &Camera3D {
		&self.three_d
	}
}

// Pretty self explanatory
pub struct Context {
	pub tree: Tree,
	pub resources: Box<dyn ResourceContainer>,
	pub camera_holder: CameraHolder,
}

impl Context {
	// Just a helper function for converting the resources into the specified type
	pub fn get_resources<T: 'static + ResourceContainer>(&self) -> &T {
		self.resources.as_any().downcast_ref::<T>().unwrap()
	}
}