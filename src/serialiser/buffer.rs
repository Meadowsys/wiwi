use super::error::*;
use ::std::{ ptr, slice };
use ::std::future::Future;

pub trait BufferWrite {
	fn reserve(&mut self, additional: usize);

	unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]);

	fn write_bytes(&mut self, bytes: &[u8]) {
		self.reserve(bytes.len());
		unsafe { self.write_bytes_unchecked(bytes) }
	}

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

	fn read_bytes_const<const N: usize>(&mut self) -> Result<&'h [u8; N]> {
		unsafe {
			self.read_bytes_ptr_const::<N>()
				.map(|ptr| &*(ptr as *const [u8; N]))
		}
	}

	fn read_byte(&mut self) -> Result<u8> {
		unsafe {
			self.read_bytes_ptr_const::<1>()
				.map(|ptr| *ptr)
		}
	}
}

// pub trait BufferWriteAsync {
// 	fn write_bytes(&mut self, bytes: &[u8]) -> impl Future<Output = ()>;
// }

// pub trait BufferReadAsync<'h> {}

impl BufferWrite for Vec<u8> {
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional);
	}

	unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]) {
		debug_assert!(self.capacity() >= self.len() + bytes.len());

		ptr::copy_nonoverlapping(
			bytes as *const [u8] as *const u8,
			self.as_mut_ptr().add(self.len()),
			bytes.len()
		);
		self.set_len(self.len() + bytes.len());
	}

	unsafe fn with_ptr<F>(&mut self, f: F)
	where
		F: FnOnce(*mut u8) -> usize
	{
		let ptr = self.as_mut_ptr().add(self.len());
		let count = f(ptr);
		self.set_len(self.len() + count);
	}
}

impl<'h> BufferRead<'h> for &'h [u8] {
	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8> {
		(self.len() >= count).then(|| {
			let ptr = *self as *const [u8] as *const u8;
			*self = slice::from_raw_parts(ptr.add(count), self.len() - count);
			ptr
		}).err_eof()
	}
}
