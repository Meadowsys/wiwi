mod marker;
mod number;

/// Trait for generic buffer impl. Unsafe trait to assert that implementers have implemented it correctly
// Might make it easier to do that unsafe impl later? :p
pub unsafe trait BufferImpl {
	// TODO: preallocation related methods
	// unsafe fn preallocate_at_once(amount: usize);
	// some kind of builder device that ultimately calls preallocate_at_once
	// unsafe fn preallocate_amount_tracker

	fn write_slice(&mut self, bytes: &[u8]);
	fn write_byte(&mut self, byte: u8);

	// TODO: unsafe write methods
	// unsafe fn write_ptr(&mut self, bytes: *const u8, len: usize) {
	// 	self.write_slice(slice::from_raw_parts(bytes, len));
	// }
	// unsafe fn write_ptr_const<const N: usize>(&mut self, bytes: *const u8) {
	// 	self.write_slice(slice::from_raw_parts(bytes, N));
	// }
	// unsafe fn write_bytes_const<N>
}

unsafe impl BufferImpl for Vec<u8> {
	fn write_slice(&mut self, bytes: &[u8]) {
		self.extend(bytes);
	}

	fn write_byte(&mut self, byte: u8) {
		self.push(byte);
	}
}

pub trait Serialise {

}

pub trait Deserialise {}

// pub fn serialise<T: Serialise>(item: &T) -> Vec<u8> {}

// pub fn deserialise<T: Deserialise>(bytes: &[u8]) -> T {}

// fn serialise_none<B: BufferImpl>(buf: &mut B) {
// 	buf.write_byte(consts::MARKER_NONE);
// }
