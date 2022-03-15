use crate::AsAny;
use macroquad::prelude::Vec2;

pub trait Node {
	fn get_type_id(&self) -> &'static str;
	fn get_path(&self) -> String;
	fn get_position(&self) -> Vec2;
}

pub trait FullNode: Node + AsAny {}

// Here's an example node :)
/*

struct TestNode {
	type_id: &'static str,
	path: String,
	position: Vec2,
}

node!(TestNode);

*/