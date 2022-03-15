use crate::AsAny;
use macroquad::prelude::Vec2;

pub trait Node {
	fn get_type_id(&self) -> &'static str;
	fn get_path(&self) -> String;
	fn get_position(&self) -> Vec2;
}

pub trait FullNode: Node + AsAny {}