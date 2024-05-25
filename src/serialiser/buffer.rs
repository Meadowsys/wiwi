use super::error::*;
use std::{ ptr, slice };
use std::future::Future;

pub trait BufferWrite {
	fn reserve(&mut self, additional: usize);

	unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]);

	#[inline]
	fn write_bytes(&mut self, bytes: &[u8]) {
		self.reserve(bytes.len());
		unsafe { self.write_bytes_unchecked(bytes) }
	}

	#[inline]
	fn write_byte(&mut self, byte: u8) {
		self.write_bytes(slice::from_ref(&byte));
	}

	unsafe fn with_ptr<F>(&mut self, f: F)
	where
		F: FnOnce(*mut u8) -> usize;
}

pub trait BufferRead<'h> {
	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8>;

	#[inline]
	unsafe fn read_bytes_ptr_const<const N: usize>(&mut self) -> Result<*const u8> {
		self.read_bytes_ptr(N)
	}

	#[inline]
	fn read_bytes(&mut self, count: usize) -> Result<&'h [u8]> {
		unsafe {
			self.read_bytes_ptr(count)
				.map(|ptr| slice::from_raw_parts(ptr, count))
		}
	}

	#[inline]
	fn read_bytes_const<const N: usize>(&mut self) -> Result<&'h [u8; N]> {
		unsafe {
			self.read_bytes_ptr_const::<N>()
				.map(|ptr| &*(ptr as *const [u8; N]))
		}
	}

	#[inline]
	fn read_byte(&mut self) -> Result<u8> {
		unsafe {
			self.read_bytes_ptr_const::<1>()
				.map(|ptr| *ptr)
		}
	}
}

impl BufferWrite for Vec<u8> {
	#[inline]
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional);
	}

	#[inline]
	unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]) {
		debug_assert!(self.capacity() >= self.len() + bytes.len());

		self.with_ptr(|ptr| {
			let len = bytes.len();
			let bytes_ptr = bytes as *const [u8] as *const u8;
			ptr::copy_nonoverlapping(bytes_ptr, ptr, len);
			len
		});
	}

	#[inline]
	unsafe fn with_ptr<F>(&mut self, f: F)
	where
		F: FnOnce(*mut u8) -> usize
	{
		let len = self.len();
		let ptr = self.as_mut_ptr().add(len);
		let count = f(ptr);
		self.set_len(len + count);
	}
}

impl<'h> BufferRead<'h> for &'h [u8] {
	#[inline]
	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8> {
		let len = self.len();
		(len >= count).then(|| {
			let ptr = *self as *const [u8] as *const u8;
			*self = slice::from_raw_parts(ptr.add(count), len - count);
			ptr
		}).err_eof()
	}
}
