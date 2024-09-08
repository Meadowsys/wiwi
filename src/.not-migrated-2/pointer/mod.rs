extern crate core;

use crate::memory::Sized;

pub use core::ptr::{
	// addr_eq,
	// addr_of,
	// addr_of_mut,
	copy,
	copy_nonoverlapping,
	// drop_in_place,
	// eq,
	// from_mut,
	// from_ref,
	// hash,
	null,
	null_mut,
	read,
	read_unaligned,
	read_volatile,
	replace,
	// slice_from_raw_parts,
	// slice_from_raw_parts_mut,
	swap,
	swap_nonoverlapping,
	write,
	write_bytes,
	write_unaligned,
	write_volatile,
	NonNull
};

#[inline(always)]
pub fn coerce_ptr<T: ?Sized>(ptr: &T) -> *const T {
	ptr
}

#[inline(always)]
pub fn coerce_mut_ptr<T: ?Sized>(ptr: &mut T) -> *mut T {
	ptr
}
