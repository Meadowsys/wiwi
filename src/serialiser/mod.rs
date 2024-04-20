use self::error::*;
use ::std::slice;

pub mod error;

mod array;
mod bool;
mod float;
mod integer;
mod marker;
mod none;
mod object;
mod string;
mod value;

pub use self::value::{ Key, Value };

pub trait Serialise {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B);
}

pub trait Deserialise<'h>: Sized {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self>;
}

pub trait DeserialiseOwned: for<'h> Deserialise<'h> {}

impl<T> DeserialiseOwned for T
where
	T: for<'h> Deserialise<'h>
{}

pub fn serialise<T: ?Sized + Serialise>(item: &T) -> Vec<u8> {
	let mut vec = Vec::new();
	item.serialise(&mut vec);
	vec
}

pub fn serialise_with_capacity<T: ?Sized + Serialise>(item: &T, capacity: usize) -> Vec<u8> {
	let mut vec = Vec::with_capacity(capacity);
	item.serialise(&mut vec);
	vec
}

pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	T::deserialise(&mut bytes)
}

/// Trait for generic buffer impl. Unsafe trait to assert that implementers have implemented it correctly
// Might make it easier to do that unsafe impl later? :p
pub unsafe trait BufferImplWrite {
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

unsafe impl BufferImplWrite for Vec<u8> {
	fn write_slice(&mut self, bytes: &[u8]) {
		self.extend(bytes);
	}

	fn write_byte(&mut self, byte: u8) {
		self.push(byte);
	}
}

pub unsafe trait BufferImplRead<'h> {
	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8>;
	// fn peek_next_byte(&mut self) -> Option<u8>;

	#[inline]
	unsafe fn read_bytes_const_ptr<const N: usize>(&mut self) -> Result<*const u8> {
		self.read_bytes_ptr(N)
	}

	#[inline]
	fn read_bytes(&mut self, count: usize) -> Result<&'h [u8]> {
		unsafe {
			self.read_bytes_ptr(count)
				.map(|ptr| ::std::slice::from_raw_parts(ptr, count))
		}
	}

	#[inline]
	fn read_bytes_const<const N: usize>(&mut self) -> Result<&'h [u8; N]> {
		unsafe {
			self.read_bytes_const_ptr::<N>()
				.map(|ptr| &*(ptr as *const [u8; N]))
		}
	}

	#[inline]
	fn read_byte(&mut self) -> Result<u8> {
		unsafe {
			self.read_bytes_const_ptr::<1>()
				.map(|ptr| *ptr)
		}
	}
}

unsafe impl<'h> BufferImplRead<'h> for &'h [u8] {
	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8> {
		(self.len() >= count).then(#[inline] || {
			let self_ptr = *self as *const [u8] as *const u8;
			*self = slice::from_raw_parts(
				self_ptr.add(count),
				self.len() - count
			);
			self_ptr
		}).err_eof()
	}

	// fn peek_next_byte(&mut self) -> Option<u8> {
	// 	(!self.is_empty()).then(#[inline] || unsafe {
	// 		*(*self as *const [u8] as *const u8)
	// 	})
	// }
}

// helper things

struct SerialiseLength3VariantsParams<'h, B> {
	pub marker_8: u8,
	pub marker_16: u8,
	pub marker_xl: u8,
	pub len: usize,
	pub output: &'h mut B
}

#[inline]
fn serialise_length_3_variants<B: BufferImplWrite>(
	params: SerialiseLength3VariantsParams<B>
) {
	use self::{ integer::*, marker::* };

	const U8_MAX: u64 = u8::MAX as u64;
	const U16_MAX: u64 = u16::MAX as u64;

	let SerialiseLength3VariantsParams {
		marker_8,
		marker_16,
		marker_xl,
		len,
		output
	} = params;

	match len as u64 {
		len @ ..=U8_MAX => {
			output.write_byte(marker_8);
			unsafe { serialise_rest_of_u64(len, MARKER_U8, output) }
		}
		len @ ..=U16_MAX => {
			output.write_byte(marker_16);
			unsafe { serialise_rest_of_u64(len, MARKER_U16, output) }
		}
		len => {
			output.write_byte(marker_xl);
			len.serialise(output);
		}
	}
}

// struct DeserialiseLength3VariantsParams<'h, B> {
// 	pub marker: u8,
// 	pub marker_8: u8,
// 	pub marker_16: u8,
// 	pub marker_xl: u8,
// 	pub input: &'h mut B,
// 	pub err_msg: &'static str
// }
//
// #[inline]
// fn deserialise_length_3_variants<B: BufferImplRead>(
// 	params: DeserialiseLength3VariantsParams<B>
// ) -> Result<usize> {
// 	use self::{ integer::*, marker::* };
//
// 	let DeserialiseLength3VariantsParams {
// 		marker,
// 		marker_8,
// 		marker_16,
// 		marker_xl,
// 		input,
// 		err_msg
// 	} = params;
//
// 	Ok(match input.read_byte()? {
// 		marker if marker == marker_8 => unsafe { deserialise_rest_of_u64(MARKER_U8, input)? as usize }
// 		marker if marker == marker_16 => unsafe { deserialise_rest_of_u64(MARKER_U16, input)? as usize }
// 		marker if marker == marker_xl => { u64::deserialise(input)? as usize }
// 		_ => { return err(err_msg) }
// 	})
// }

enum MarkerType {
	M8,
	M16,
	MXL
}

#[inline]
fn deserialise_rest_of_length_3_variants<'h, B: BufferImplRead<'h>>(
	marker_type: MarkerType,
	input: &mut B
) -> Result<usize> {
	use self::{ integer::*, marker::* };

	Ok(match marker_type {
		MarkerType::M8 => unsafe { deserialise_rest_of_u64(MARKER_U8, input)? as usize }
		MarkerType::M16 => unsafe { deserialise_rest_of_u64(MARKER_U16, input)? as usize }
		MarkerType::MXL => { u64::deserialise(input)? as usize }
	})
}
