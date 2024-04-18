use super::{ BufferImplWrite, marker::MARKER_NONE };

pub fn serialise_none<B: BufferImplWrite>(output: &mut B) {
	output.write_byte(MARKER_NONE);
}
