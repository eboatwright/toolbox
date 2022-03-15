use crate::AsAny;

pub trait ResourceContainer: AsAny {}

// Here's an example ResourceContainer :D
/*

struct Resources {
	texture: Texture2D,
	sound: Sound,
	font: Font,
}

resources!(Resources);

*/