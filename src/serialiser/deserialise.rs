use super::{ buffer::*, error::* };

pub trait Deserialise<'h>: Sized {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self>;
}

pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	let value = T::deserialise(&mut bytes)?;
	if !bytes.is_empty() { return err_str("trailing bytes found") }
	Ok(value)
}

/// Deserialise the next item from the provided slice, ignoring excess
///
/// Return the value as well as the remaining slice that wasn't consumed.
pub fn deserialise_one<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<(T, &'h [u8])> {
	let res = T::deserialise(&mut bytes);
	res.map(|val| (val, bytes))
}
