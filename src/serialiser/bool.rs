use super::{ buffer::*, core::*, error::* };

pub const MARKER_BOOL: u8 = 0xa0;
pub const MARKER_TRUE: u8 = 0xa0;
pub const MARKER_FALSE: u8 = 0xa1;

impl Serialise for bool {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		match *self {
			true => { output.write_byte(MARKER_TRUE) }
			false => { output.write_byte(MARKER_FALSE) }
		}
	}
}

impl<'h> Deserialise<'h> for bool {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<bool> {
		retrieve_bool_from_marker(input.read_byte()?)
			.err("expected bool")
	}
}

pub fn retrieve_bool_from_marker(marker: u8) -> Option<bool> {
	match marker {
		MARKER_TRUE => { Some(true) }
		MARKER_FALSE => { Some(false) }
		_ => { None }
	}
}
