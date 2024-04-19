use super::{
	BufferImplRead,
	BufferImplWrite,
	Deserialise,
	Serialise,
	SerialiseLength3VariantsParams,
	deserialise_rest_of_length_3_variants,
	serialise_length_3_variants,
	MarkerType
};
use super::{ error::*, marker::*, integer::* };
use ::std::str;

impl Serialise for str {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
		serialise_length_3_variants(SerialiseLength3VariantsParams {
			marker_8: MARKER_STRING_8,
			marker_16: MARKER_STRING_16,
			marker_xl: MARKER_STRING_XL,
			len: self.len(),
			output
		});

		output.write_slice(self.as_bytes());
	}
}

impl<'h> Deserialise<'h> for &'h str {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		match input.read_byte()? {
			MARKER_STRING_8 => { deserialise_rest_of_str(MarkerType::M8, input) }
			MARKER_STRING_16 => { deserialise_rest_of_str(MarkerType::M16, input) }
			MARKER_STRING_XL => { deserialise_rest_of_str(MarkerType::MXL, input) }
			_ => { err("expected string") }
		}
	}
}

impl<'h> Deserialise<'h> for String {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		<&str>::deserialise(input).map(Into::into)
	}
}

fn deserialise_rest_of_str<'h, B: BufferImplRead<'h>>(
	marker_type: MarkerType,
	input: &mut B
) -> Result<&'h str> {
	let len = deserialise_rest_of_length_3_variants(marker_type, input)?;
	str::from_utf8(input.read_bytes(len)?)
		.convert_err()
}
