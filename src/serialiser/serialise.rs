use super::buffer::*;

pub trait Serialise {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &SerialiseOptions);
}

pub fn serialise<T: ?Sized + Serialise>(item: &T) -> Vec<u8> {
	serialise_with_options(item, &SerialiseOptions::default())
}

pub fn serialise_with_options<T: ?Sized + Serialise>(item: &T, options: &SerialiseOptions) -> Vec<u8> {
	let mut vec = Vec::with_capacity(options.capacity);
	item.serialise(&mut vec, options);
	vec
}

#[derive(Clone, Debug)]
pub struct SerialiseOptions {
	pub capacity: usize
}

impl Default for SerialiseOptions {
	fn default() -> Self {
		SerialiseOptions {
			capacity: 128
		}
	}
}
