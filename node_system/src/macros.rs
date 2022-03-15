// This is a helper macro for implementing everything a Node needs
// (You have to have the type_id, path, and position properties on your struct)
#[macro_export]
macro_rules! node {
	($struct_name: ident) => {
		impl Node for $struct_name {
			fn get_type_id(&self) -> &'static str { self.type_id }
			fn get_path(&self) -> String { self.path.clone() }
			fn get_position(&self) -> Vec2 { self.position }
		}

		impl AsAny for $struct_name {
			fn as_any(&self) -> &dyn Any { self as &dyn Any }
			fn as_any_mut(&mut self) -> &mut dyn Any { self as &mut dyn Any }
		}

		impl FullNode for $struct_name {}
	}
}

// This is a helper macro for implementing everything a resource container needs
#[macro_export]
macro_rules! resources {
	($struct_name: ident) => {
		impl AsAny for $struct_name {
			fn as_any(&self) -> &dyn Any { self as &dyn Any }
			fn as_any_mut(&mut self) -> &mut dyn Any { self as &mut dyn Any }
		}

		impl ResourceContainer for $struct_name {}
	}
}