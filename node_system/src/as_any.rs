use std::any::Any;

// Nodes and Resources have to implement this
pub trait AsAny {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}