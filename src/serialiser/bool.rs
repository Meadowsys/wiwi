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
	fn deserialise<B: BufferImplRead>(output: &mut B) -> Result<bool> {
		match output.read_next_byte()? {
			MARKER_BOOL_TRUE => { Ok(true) }
			MARKER_BOOL_FALSE => { Ok(false) }
			_ => { err("expected bool") }
		}
	}
}
