use super::{ buffer::*, error::* };

pub trait Serialise {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options);
}

pub trait Deserialise<'h>: Sized {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self>;
}

pub fn serialise<T: ?Sized + Serialise>(item: &T) -> Vec<u8> {
	serialise_with_options(item, &Options::default())
}

pub fn serialise_with_options<T: ?Sized + Serialise>(item: &T, options: &Options) -> Vec<u8> {
	let mut vec = Vec::with_capacity(options.capacity);
	item.serialise(&mut vec, options);
	vec
}

pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	let value = T::deserialise(&mut bytes);
	if !bytes.is_empty() { return err("trailing bytes found") }
	value
}

pub fn deserialise_lax<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	T::deserialise(&mut bytes)
}

#[derive(Clone, Debug)]
pub struct Options {
	pub capacity: usize
}

impl Default for Options {
	fn default() -> Self {
		Options {
			capacity: 128
		}
	}
}
