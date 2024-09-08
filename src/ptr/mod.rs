use crate::prelude::*;

#[inline(always)]
pub fn coerce_ptr<T: ?Sized>(ptr: &T) -> *const T {
	ptr
}

#[inline(always)]
pub fn coerce_mut_ptr<T: ?Sized>(ptr: &mut T) -> *mut T {
	ptr
}
