use crate::Tree;
use crate::ResourceContainer;

// Pretty self explanatory
pub struct Context {
	pub tree: Tree,
	pub resources: Box<dyn ResourceContainer>,
}

impl Context {
	// Just a helper function for converting the resources into the specified type
	pub fn get_resources<T: 'static + ResourceContainer>(&self) -> &T {
		self.resources.as_any().downcast_ref::<T>().unwrap()
	}
}