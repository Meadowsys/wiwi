use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::{ error::*, integer::*, marker::*, value::Value };

pub fn serialise_value_array<B: BufferImplWrite>(arr: &[Value], output: &mut B) {
	const U8_MAX: u64 = u8::MAX as u64;
	const U16_MAX: u64 = u16::MAX as u64;
	const U24_MAX: u64 = (u16::MAX as u64) << 8 | u8::MAX as u64;

	match arr.len() as u64 {
		len @ ..=U8_MAX => {
			output.write_byte(MARKER_HETEROGENOUS_ARRAY_8);
			output.write_byte(len as u8);
		}
		len @ ..=U16_MAX => {
			output.write_byte(MARKER_HETEROGENOUS_ARRAY_16);
			unsafe { serialise_rest_of_u64(len, MARKER_U16, output) }
		}
		len @ ..=U24_MAX => {
			output.write_byte(MARKER_HETEROGENOUS_ARRAY_24);
			unsafe { serialise_rest_of_u64(len, MARKER_U24, output) }
		}
		len => {
			output.write_byte(MARKER_HETEROGENOUS_ARRAY_XL);
			len.serialise(output);
		}
	}

	// TODO: this is not good for auto vectorisation
	arr.iter().for_each(|item| item.serialise(output));
}

#[inline]
pub fn deserialise_rest_of_heterogenous_array_8<'h, V: Deserialise<'h>, B: BufferImplRead>(input: &mut B) -> Result<Vec<V>> {
	let len = unsafe { deserialise_rest_of_u64(MARKER_U8, input)? as usize };
	(0..len).map(|_| V::deserialise(input)).collect()
}

#[inline]
pub fn deserialise_rest_of_heterogenous_array_16<'h, V: Deserialise<'h>, B: BufferImplRead>(input: &mut B) -> Result<Vec<V>> {
	let len = unsafe { deserialise_rest_of_u64(MARKER_U16, input)? as usize };
	(0..len).map(|_| V::deserialise(input)).collect()
}

#[inline]
pub fn deserialise_rest_of_heterogenous_array_24<'h, V: Deserialise<'h>, B: BufferImplRead>(input: &mut B) -> Result<Vec<V>> {
	let len = unsafe { deserialise_rest_of_u64(MARKER_U24, input)? as usize };
	(0..len).map(|_| V::deserialise(input)).collect()
}

#[inline]
pub fn deserialise_rest_of_heterogenous_array_xl<'h, V: Deserialise<'h>, B: BufferImplRead>(input: &mut B) -> Result<Vec<V>> {
	let len = u64::deserialise(input)?;
	(0..len).map(|_| V::deserialise(input)).collect()
}
