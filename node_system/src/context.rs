use crate::Tree;
use crate::ResourceContainer;

pub struct Context {
	pub tree: Tree,
	pub resources: Box<dyn ResourceContainer>,
}

impl Context {
	pub fn get_resources<T: 'static + ResourceContainer>(&self) -> &T {
		self.resources.as_any().downcast_ref::<T>().unwrap()
	}
}