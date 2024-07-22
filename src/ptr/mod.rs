use std::ops::{ Deref, DerefMut };

#[inline(always)]
pub fn coerce_ptr<T: ?Sized>(thing: &T) -> *const T {
	thing
}

#[inline(always)]
pub fn coerce_ptr_mut<T: ?Sized>(thing: &mut T) -> *mut T {
	thing
}

#[inline(always)]
pub fn coerce_slice_ptr<T>(thing: &[T]) -> *const T {
	coerce_ptr(thing).cast()
}

#[inline(always)]
pub fn coerce_slice_ptr_mut<T>(thing: &mut [T]) -> *mut T {
	coerce_ptr_mut(thing).cast()
}

#[inline(always)]
pub unsafe fn reborrow<'h, T: ?Sized>(thing: *const T) -> &'h T {
	unsafe { &*thing }
}

#[inline(always)]
pub unsafe fn reborrow_mut<'h, T: ?Sized>(thing: *mut T) -> &'h mut T {
	unsafe { &mut *thing }
}

#[inline(always)]
pub unsafe fn deref_ptr<T: Deref>(thing: *const T) -> *const T::Target {
	unsafe { &**thing }
}

#[inline(always)]
pub unsafe fn deref_ptr_mut<T: DerefMut>(thing: *mut T) -> *mut T::Target {
	unsafe { &mut **thing }
}
