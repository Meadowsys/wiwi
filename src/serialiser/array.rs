use super::{ buffer::*, core::*, error::*, integer::*, len_int::* };

mod single_type;

pub const MARKER_ARRAY_8: u8 = 0xaa;
pub const MARKER_ARRAY_XL: u8 = 0xab;

pub fn serialise_array<T, B>(items: &[T], output: &mut B, options: &Options)
where
	T: Serialise,
	B: BufferWrite
{
	match items.len() {
		len @ ..=0xff => {
			output.write_byte(MARKER_ARRAY_8);
			output.write_byte(len as _);
		}
		len => {
			output.write_byte(MARKER_ARRAY_XL);
			serialise_len_int(len as _, output);
		}
	}

	for item in items {
		item.serialise(output, options);
	}
}

pub fn deserialise_array<'h, T, B>(input: &mut B) -> Result<Vec<T>>
where
	T: Deserialise<'h>,
	B: BufferRead<'h>
{
	let len = deserialise_array_len(input.read_byte()?, input)?;
	deserialise_rest_of_array(len, input)
}

pub fn deserialise_array_len<'h, B: BufferRead<'h>>(marker: u8, input: &mut B) -> Result<usize> {
	match marker {
		MARKER_ARRAY_8 => { input.read_byte().map(|len| len as _) }
		MARKER_ARRAY_XL => { deserialise_len_int(input) }
		_ => { err("expected array") }
	}
}

pub fn deserialise_rest_of_array<'h, T, B>(len: usize, input: &mut B) -> Result<Vec<T>>
where
	T: Deserialise<'h>,
	B: BufferRead<'h>
{
	(0..len).map(|_| T::deserialise(input)).collect()
}
