use std::any::Any;
use crate::*;

use macroquad::prelude::Vec3;

pub struct Tree {
	pub nodes: Vec<Box<dyn FullNode>>,
}

impl Tree {
	// All of these are pretty self explanatory
	pub fn get_node<NodeType: 'static + FullNode>(&self, path: &str) -> Option<&NodeType> {
		self.nodes
			.iter()
			.find(|node| node.get_path() == path)
			.map(|node| node.as_any().downcast_ref::<NodeType>().unwrap())
	}

	pub fn get_node_mut<NodeType: 'static + FullNode>(&mut self, path: &str) -> Option<&mut NodeType> {
		self.nodes
			.iter_mut()
			.find(|node| node.get_path() == path)
			.map(|node| node.as_any_mut().downcast_mut::<NodeType>().unwrap())
	}

	pub fn get_node_raw(&self, path: &str) -> &Box<dyn FullNode> {
		self.nodes
			.iter()
			.find(|node| node.get_path() == path)
			.unwrap()
	}

	pub fn get_node_mut_raw(&mut self, path: &str) -> &mut Box<dyn FullNode> {
		self.nodes
			.iter_mut()
			.find(|node| node.get_path() == path)
			.unwrap()
	}

	pub fn get_nodes_by_type_id(&self, id: &str) -> Vec<&dyn Any> {
		self.nodes
			.iter()
			.filter(|node| node.get_type_id() == id)
			.map(|node| node.as_any())
			.collect()
	}

	pub fn get_nodes_by_type_id_mut(&mut self, id: &str) -> Vec<&mut dyn Any> {
		self.nodes
			.iter_mut()
			.filter(|node| node.get_type_id() == id)
			.map(|node| node.as_any_mut())
			.collect()
	}

	pub fn get_node_parent_path(&self, path: String) -> String {
		let split: Vec<&str> = path.split('/').collect();
		path[0..path.len() - split[split.len() - 1].len() - 1].to_string()
	}

	pub fn get_node_position(&self, path: String) -> Vec3 {
		let mut position = Vec3::ZERO;
		let mut temp = path.clone();
		while temp.len() > 0 {
			let node_position = self.get_node_raw(&temp).get_position();
			position += node_position;

			let split: Vec<&str> = temp.split('/').collect();
			if split.len() <= 1 {
				break;
			}
			temp = self.get_node_parent_path(temp);
		}
		position
	}

	pub fn get_node_parent<NodeType: 'static + FullNode>(&self, path: String) -> Option<&NodeType> {
		let parent_path = self.get_node_parent_path(path);
		self.get_node(&parent_path)
	}

	pub fn get_node_parent_mut<NodeType: 'static + FullNode>(&mut self, path: String) -> Option<&mut NodeType> {
		let parent_path = self.get_node_parent_path(path);
		self.get_node_mut(&parent_path)
	}
}