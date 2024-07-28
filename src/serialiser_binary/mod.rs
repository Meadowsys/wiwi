// TODO: remove this
#![allow(
	dead_code,
	missing_docs,
	unsafe_op_in_unsafe_fn,
	clippy::missing_safety_doc,
	clippy::undocumented_unsafe_blocks
)]

use std::{ ptr, slice };
use std::marker::PhantomData;

pub mod error;
mod marker;

mod bool;
mod cow;
mod number;
mod string;

pub use self::bool::BoolSerialiser;
pub use self::number::{
	U8Serialiser,
	U16Serialiser,
	U32Serialiser,
	U64Serialiser,
	U128Serialiser,
	USizeSerialiser,
	I8Serialiser,
	I16Serialiser,
	I32Serialiser,
	I64Serialiser,
	I128Serialiser,
	ISizeSerialiser
};
pub use self::error::{ Error, Result };

#[inline]
pub fn serialise<T>(item: &T) -> Vec<u8>
where
	T: Serialise + ?Sized
{
	let mut vec = Vec::new();
	let mut buf = OutputVecBuffer::new(&mut vec);
	serialise_into(item, &mut buf);
	vec
}

#[inline]
pub fn serialise_into<T, O>(item: &T, buf: &mut O)
where
	T: Serialise + ?Sized,
	O: Output
{
	let serialiser = item.build_serialiser();
	let capacity = unsafe { serialiser.needed_capacity() };

	buf.reserve(capacity);
	unsafe { serialiser.serialise(buf) }
}

#[inline]
pub fn deserialise<'h, T>(bytes: &[u8]) -> Result<T, T::Error>
where
	T: Deserialise<'h>
{
	let mut input = InputSliceBuffer::new(bytes);
	Ok(use_ok!(
		T::deserialise(&mut input),
		val => {
			if input.is_empty() {
				val
			} else {
				return error::expected(error::expected::DESC_EXPECTED_EOF)
					.found(error::found::DESC_FOUND_TRAILING_BYTES)
					.wrap_foreign()
			}
		}
	))
}

pub trait Serialise {
	type Serialiser<'h>: Serialiser<'h> where Self: 'h;

	/// Gather all data required for serialisation, and store them in this
	/// serialiser struct
	fn build_serialiser(&self) -> Self::Serialiser<'_>;
}

pub trait Serialiser<'h> {
	/// Get the amount of bytes this item will take up when serialised
	///
	/// # Safety
	///
	/// Unsafe code is allowed to rely on this for soundness. If you cannot
	/// precisely determine amount needed, you should (and must for soundness
	/// reasons) provide the upper bound.
	unsafe fn needed_capacity(&self) -> usize;

	/// Serialise `self` into the provided output buffer
	///
	/// # Safety
	///
	/// The provided output `buf` must have at least the amount of free bytes
	/// available to write to as `needed_capacity` returns (ie. calling
	/// `needed_capacity`, then calling `reserve` on `buf` with the value returned
	/// from `needed_capacity`, satisfies this precondition).
	unsafe fn serialise<O: Output>(&self, buf: &mut O);
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

impl Output for Vec<u8> {
	#[inline]
	fn reserve(&mut self, bytes: usize) {
		self.reserve(bytes);
	}

	#[inline]
	unsafe fn write_bytes(&mut self, bytes: &[u8]) {
		let len = self.len();
		let ptr = self.as_mut_ptr().add(len);

		ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
		self.set_len(len + bytes.len())
	}

	#[inline]
	unsafe fn write_byte(&mut self, byte: u8) {
		let len = self.len();
		let ptr = self.as_mut_ptr().add(len);

		ptr::write(ptr, byte);
		self.set_len(len + 1);
	}
}

pub trait Input<'h> {
	fn read_bytes_ptr(&mut self, bytes: usize) -> Result<*const u8, error::ErrorFound>;

	#[inline]
	fn read_bytes(&mut self, bytes: usize) -> Result<&'h [u8], error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes_ptr(bytes),
			// SAFETY: if this returned `Some` then the ptr is valid for `bytes` reads
			// so is safe to create slice here
			ptr => unsafe { slice::from_raw_parts(ptr, bytes) },
			#err err => err.wrap()
		))
	}

	#[inline]
	fn read_byte(&mut self) -> Result<u8, error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes_ptr(1),
			// SAFETY: ptr returned by `read_bytes_ptr` is
			// guaranteed to be readable for at least 1 byte
			byte => unsafe { *byte },
			#err err => err.wrap()
		))
	}

	#[inline]
	fn read_bytes_const<const BYTES: usize>(&mut self) -> Result<&'h [u8; BYTES], error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes(BYTES),
			bytes => unsafe { &*bytes.as_ptr().cast::<[u8; BYTES]>() },
			#err err => err.wrap()
		))
	}
}

pub struct OutputVecBuffer<'h> {
	vec: &'h mut Vec<u8>,
	ptr: *const u8
}

impl<'h> OutputVecBuffer<'h> {
	#[inline]
	pub fn new(vec: &'h mut Vec<u8>) -> Self {
		let ptr = vec.as_mut_ptr();
		Self { vec, ptr }
	}
}

impl<'h> Output for OutputVecBuffer<'h> {
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
		self.ptr = self.ptr.add(1);
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
	fn read_bytes_ptr(&mut self, bytes: usize) -> Result<*const u8, error::ErrorFound> {
		if self.remaining < bytes {
			error::found_eof().wrap()
		} else {
			let ptr = self.ptr;
			self.remaining -= bytes;
			self.ptr = unsafe { self.ptr.add(bytes) };
			Ok(ptr)
		}
	}
}

macro_rules! use_marker {
	($buf:ident) => {
		use_ok!(
			$buf.read_byte(),
			#err err => err.expected(
				$crate::serialiser_binary::error::expected::DESC_EXPECTED_MARKER
			).wrap()
		)
	}
}
use use_marker;

macro_rules! use_ok {
	($result:expr) => {
		match $result {
			Ok(val) => { val }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, #err $err:ident => $err_op:expr) => {
		match $result {
			Ok(val) => { val }
			Err($err) => { return $err_op }
		}
	};

	($result:expr, $val:ident => $op:expr) => {
		match $result {
			Ok($val) => { $op }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, $val:ident => $op:expr, #err $err:ident => $err_op:expr) => {
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

	($option:expr, #none => $none_op:expr) => {
		match $option {
			Some(val) => { val }
			None => { return $none_op }
		}
	};

	($option:expr, $val:ident => $op:expr) => {
		match $option {
			Some($val) => { $op }
			None => { return None }
		}
	};

	($option:expr, $val:ident => $op:expr, #none => $none_op:expr) => {
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
			$name:ident, $val:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub const $name: $($type)+ = $val;
			)*
		}
	};

	{
		@impl(static)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $val:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub static $name: $($type)+ = $val;
			)*
		}
	};

	{
		const type u8
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $val:expr)*
	} => {
		consts! {
			@impl(const)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $val, u8))*
		}
	};

	{
		static type &'static str
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $val:expr)*
	} => {
		consts! {
			@impl(static)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $val, &'static str))*
		}
	};
}
use consts;

#[allow(unused_imports)]
mod internal_prelude {
	pub(super) use crate::num_traits::*;
	pub(super) use super::{
		Deserialise,
		Input,
		Output,
		Serialise,
		Serialiser,
		use_marker,
		use_ok,
		use_some
	};
	pub(super) use super::error::{
		Error,
		ErrorExpected,
		ErrorFound,
		Result,
		expected,
		found,
		found_eof,
		found_something_else
	};
	pub(super) use super::error::expected::*;
	pub(super) use super::error::found::*;
	pub(super) use super::marker::*;
	pub(super) use super::marker::markers::*;
}
