// TODO: this should be removed
#![allow(
	dead_code,
	missing_docs
)]

use std::{ ptr, slice };
use std::marker::PhantomData;

mod error;
mod marker;

mod bool;

pub use error::{ Error, Result };

#[inline]
pub fn serialise<T: Serialise>(item: &T) -> Vec<u8> {
	// SAFETY: `calc_needed_capacity` has no invariants, it's unsafe to make
	// implementor assert they've followed preconditions relied on for soundness
	let capacity = unsafe { item.calc_needed_capacity() };
	let mut out = OutputVecBuffer::with_capacity(capacity);
	item.serialise(&mut out);
	out.into_vec()
}

#[inline]
pub fn deserialise<'h, T: Deserialise<'h>>(bytes: &[u8]) -> Result<T, T::Error> {
	let mut input = InputSliceBuffer::new(bytes);
	Ok(use_ok!(
		T::deserialise(&mut input),
		val => {
			if input.is_empty() {
				val
			} else {
				return error::expected(error::expected::DESC_EXPECTED_EOF)
					.found(error::found::DESC_FOUND_TRAILING_BYTES)
					.wrap_in_foreign_err()
			}
		}
	))
}

pub trait Serialise: Sized {
	/// Calculate the amount of bytes this item will take up when serialised
	///
	/// # Safety
	///
	/// Unsafe code is allowed to rely on this for soundness. If you cannot
	/// precisely determine amount needed, you should (and must for soundness
	/// reasons) err on the larger side.
	unsafe fn calc_needed_capacity(&self) -> usize;

	// Serialise `self` into the provided output buffer
	fn serialise<O: Output>(&self, buf: &mut O);
}

pub trait Deserialise<'h>: Sized {
	type Error: std::error::Error + From<Error>;

	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<Self, Self::Error>;
}

pub trait Output {
	/// Reserve the given amount of bytes in the buffer
	fn reserve(&mut self, bytes: usize);

	/// Write all the bytes in the byte slice in
	///
	/// # Safety
	///
	/// You must have reserved at least the amount of space as `bytes.len()`
	/// before calling this.
	unsafe fn write_bytes(&mut self, bytes: &[u8]);

	/// Write the byte in
	///
	/// # Safety
	///
	/// You must have reserved at least 1 before calling this.
	unsafe fn write_byte(&mut self, byte: u8);
}

pub trait Input<'h> {
	fn read_bytes_ptr(&mut self, bytes: usize) -> Option<*const u8>;

	#[inline]
	fn read_bytes_ptr_const<const BYTES: usize>(&mut self) -> Option<*const u8> {
		self.read_bytes_ptr(BYTES)
	}

	#[inline]
	fn read_bytes(&mut self, bytes: usize) -> Option<&'h [u8]> {
		Some(use_some!(
			self.read_bytes_ptr(bytes),
			// SAFETY: if this returned `Some` then the ptr is valid for `bytes` reads
			// so is safe to create slice here
			ptr => unsafe { slice::from_raw_parts(ptr, bytes) }
		))
	}

	#[inline]
	fn read_bytes_const<const BYTES: usize>(&mut self) -> Option<&'h [u8; BYTES]> {
		Some(use_some!(
			self.read_bytes(BYTES),
			// SAFETY: `read_bytes` returns either None or Some(&[u8]) with length
			// equal to what we asked for (`N`). So, pointer returned is valid for
			// `N` reads
			bytes => unsafe { &*bytes.as_ptr().cast() }
		))
	}

	#[inline]
	fn read_byte(&mut self) -> Option<u8> {
		Some(use_some!(
			self.read_bytes_ptr_const::<1>(),
			// SAFETY: ptr returned by `read_bytes_ptr_const` is
			// guaranteed to be readable for at least 1 byte
			byte => unsafe { *byte }
		))
	}
}

pub struct OutputVecBuffer {
	vec: Vec<u8>,
	ptr: *const u8
}

impl OutputVecBuffer {
	#[inline]
	fn with_capacity(capacity: usize) -> Self {
		let mut vec = Vec::with_capacity(capacity);
		let ptr = vec.as_mut_ptr();
		Self { vec, ptr }
	}

	#[inline]
	fn into_vec(self) -> Vec<u8> {
		self.vec
	}
}

impl Output for OutputVecBuffer {
	#[inline]
	fn reserve(&mut self, bytes: usize) {
		self.vec.reserve(bytes);

		let len = self.vec.len();
		let ptr = self.vec.as_mut_ptr();
		self.ptr = unsafe { ptr.add(len).cast_const() };
	}

	#[inline]
	unsafe fn write_bytes(&mut self, bytes: &[u8]) {
		let bytes_len = bytes.len();

		ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.cast_mut(), bytes_len);
		self.ptr = self.ptr.add(bytes_len);

		let vec_len = self.vec.len();
		self.vec.set_len(vec_len + bytes_len);
	}

	#[inline]
	unsafe fn write_byte(&mut self, byte: u8) {
		ptr::write(self.ptr.cast_mut(), byte);

		let vec_len = self.vec.len();
		self.vec.set_len(vec_len + 1);
	}
}

pub struct InputSliceBuffer<'h> {
	ptr: *const u8,
	remaining: usize,
	_marker: PhantomData<&'h [u8]>
}

impl<'h> InputSliceBuffer<'h> {
	#[inline]
	fn new(bytes: &[u8]) -> Self {
		Self {
			ptr: bytes.as_ptr(),
			remaining: bytes.len(),
			_marker: PhantomData
		}
	}

	fn is_empty(&self) -> bool {
		self.remaining == 0
	}
}

impl<'h> Input<'h> for InputSliceBuffer<'h> {
	fn read_bytes_ptr(&mut self, bytes: usize) -> Option<*const u8> {
		if self.remaining < bytes {
			None
		} else {
			let ptr = self.ptr;
			self.remaining -= bytes;
			self.ptr = unsafe { self.ptr.add(bytes) };
			Some(ptr)
		}
	}
}

macro_rules! use_ok {
	($result:expr) => {
		match $result {
			Ok(val) => { val }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, $val:ident => $op:expr) => {
		match $result {
			Ok($val) => { $op }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, $val:ident => $op:expr, $err:ident => $err_op:expr) => {
		match $result {
			Ok($val) => { $op }
			Err($err) => { return $err_op }
		}
	};
}
use use_ok;

macro_rules! use_some {
	($option:expr) => {
		match $option {
			Some(val) => { val }
			None => { return None }
		}
	};

	($option:expr, $val:ident => $op:expr) => {
		match $option {
			Some($val) => { $op }
			None => { return None }
		}
	};

	($option:expr, $val:ident => $op:expr, none => $none_op:expr) => {
		match $option {
			Some($val) => { $op }
			None => { return $none_op }
		}
	};
}
use use_some;

macro_rules! consts {
	{
		@impl(const)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub const $name: $($type)+ = $value;
			)*
		}
	};

	{
		@impl(static)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub static $name: $($type)+ = $value;
			)*
		}
	};

	{
		const type u8
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts! {
			@impl(const)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $value, u8))*
		}
	};

	{
		static type &'static str
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts! {
			@impl(static)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $value, &'static str))*
		}
	};
}
use consts;
