use self::error::{ Result, ResultExt };
use ::std::slice;

pub mod error;

mod bool;
mod float;
mod integer;
mod marker;
mod none;
mod value;

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

pub unsafe trait BufferImplRead {
	unsafe fn read_next_bytes_ptr(&mut self, count: usize) -> Result<*const u8>;
	// fn peek_next_byte(&mut self) -> Option<u8>;

	#[inline]
	unsafe fn read_next_bytes_const_ptr<const N: usize>(&mut self) -> Result<*const u8> {
		self.read_next_bytes_ptr(N)
	}

	#[inline]
	fn read_next_bytes(&mut self, count: usize) -> Result<&[u8]> {
		unsafe {
			self.read_next_bytes_ptr(count)
				.map(|ptr| ::std::slice::from_raw_parts(ptr, count))
		}
	}

	#[inline]
	fn read_next_bytes_const<const N: usize>(&mut self) -> Result<&[u8; N]> {
		unsafe {
			self.read_next_bytes_const_ptr::<N>()
				.map(|ptr| &*(ptr as *const [u8; N]))
		}
	}

	#[inline]
	fn read_next_byte(&mut self) -> Result<u8> {
		unsafe {
			self.read_next_bytes_const_ptr::<1>()
				.map(|ptr| *ptr)
		}
	}
}

unsafe impl<'h> BufferImplRead for &'h [u8] {
	unsafe fn read_next_bytes_ptr(&mut self, count: usize) -> Result<*const u8> {
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

pub trait Serialise {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B);
}

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

pub trait Deserialise<'h>: Sized {
	fn deserialise<B: BufferImplRead>(input: &mut B) -> Result<Self>;
}

pub trait DeserialiseOwned: for<'h> Deserialise<'h> {}

pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
	T::deserialise(&mut bytes)
}
