use super::{ buffer::*, core::*, error::*, marker::* };

impl Serialise for bool {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		let byte = BoolMarker::from_bool(*self).to_marker();
		output.write_byte(byte);
	}
}

impl<'h> Deserialise<'h> for bool {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<bool> {
		Marker::from_byte(input.read_byte()?)?
			.expect_bool()
			.err("expected bool")
			.map(|b| b.to_bool())
	}
}
