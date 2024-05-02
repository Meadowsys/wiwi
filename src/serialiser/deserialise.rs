use super::{ buffer::*, error::* };

pub trait Deserialise<'h>: Sized {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self>;
}

pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	let value = T::deserialise(&mut bytes)?;
	if !bytes.is_empty() { return err_str("trailing bytes found") }
	Ok(value)
}

pub fn deserialise_lax<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	T::deserialise(&mut bytes)
}
