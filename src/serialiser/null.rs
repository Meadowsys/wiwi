use super::buffer::*;

pub const MARKER_NULL: u8 = 0xa7;

pub fn serialise_null<B: BufferWrite>(output: &mut B) {
	output.write_byte(MARKER_NULL);
}

#[inline]
pub fn marker_is_valid_null(marker: u8) -> bool {
	marker == MARKER_NULL
}
