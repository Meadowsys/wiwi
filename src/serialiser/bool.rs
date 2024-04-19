use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::error::*;
use super::marker::{ MARKER_BOOL_FALSE, MARKER_BOOL_TRUE };

impl Serialise for bool {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
		match *self {
			true => { output.write_byte(MARKER_BOOL_TRUE) }
			false => { output.write_byte(MARKER_BOOL_FALSE) }
		}
	}
}

impl<'h> Deserialise<'h> for bool {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<bool> {
		retrieve_bool_from_marker(input.read_byte()?)
			.err("expected bool")
	}
}

pub fn retrieve_bool_from_marker(marker: u8) -> Option<bool> {
	match marker {
		MARKER_BOOL_TRUE => { Some(true) }
		MARKER_BOOL_FALSE => { Some(false) }
		_ => { None }
	}
}
