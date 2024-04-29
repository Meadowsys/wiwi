use super::{ buffer::*, core::*, error::*, integer::*, len_int::* };
use ::std::{ borrow::Cow, ptr, str };

pub const MARKER_STRING_8: u8 = 0xa8;
pub const MARKER_STRING_XL: u8 = 0xa9;

impl Serialise for str {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		match self.len() {
			len @ ..=0xff => {
				output.write_byte(MARKER_STRING_8);
				output.write_byte(len as _);
			}
			len => {
				output.write_byte(MARKER_STRING_XL);
				serialise_len_int(len as _, output);
			}
		}

		output.write_bytes(self.as_bytes());
	}
}

impl Serialise for String {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		(**self).serialise(output, options);
	}
}

impl<'h> Serialise for Cow<'h, str> {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		(**self).serialise(output, options);
	}
}

// fn deserialise_rest_of_str<'h, B: BufferRead<'h>>(len: u64, input: &mut B) -> Result<Self> {}

impl<'h> Deserialise<'h> for &'h str {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		let len = deserialise_str_len(input.read_byte()?, input)?;
		deserialise_rest_of_str(len, input)
	}
}

impl<'h> Deserialise<'h> for String {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		<&str>::deserialise(input).map(Into::into)
	}
}

impl<'h> Deserialise<'h> for Cow<'h, str> {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		Ok(Self::Borrowed(<&str>::deserialise(input)?))
	}
}

pub fn deserialise_str_len<'h, B: BufferRead<'h>>(marker: u8, input: &mut B) -> Result<usize> {
	match marker {
		MARKER_STRING_8 => { input.read_byte().map(|len| len as _) }
		MARKER_STRING_XL => { deserialise_len_int(input) }
		_ => { err("expected string") }
	}
}

pub fn deserialise_rest_of_str<'h, B: BufferRead<'h>>(
	len: usize,
	input: &mut B
) -> Result<&'h str> {
	str::from_utf8(input.read_bytes(len)?)
		.convert_err()
}
