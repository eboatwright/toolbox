// use std::any::Any;
use macroquad::prelude::*;
use crate::Tree;
use crate::ResourceContainer;

// A work-around to having a 2d or 3d camera
pub struct CameraHolder {
	pub camera2d: Option<Camera2D>,
	pub camera3d: Option<Camera3D>,
}

impl CameraHolder {
	pub fn camera2d(camera2d: Camera2D) -> Self {
		Self {
			camera2d: Some(camera2d),
			camera3d: None,
		}
	}

	pub fn camera3d(camera3d: Camera3D) -> Self {
		Self {
			camera2d: None,
			camera3d: Some(camera3d),
		}
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