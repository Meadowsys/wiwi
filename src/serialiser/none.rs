use super::{ BufferImplWrite, marker::MARKER_NONE };

pub fn serialise_none<B: BufferImplWrite>(output: &mut B) {
	output.write_byte(MARKER_NONE);
}

#[inline]
pub fn marker_is_valid_none(marker: u8) -> bool {
	marker == MARKER_NONE
}
