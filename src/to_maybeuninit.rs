use std::mem::MaybeUninit;

pub trait ToMaybeUninit: Sized {
	fn to_maybeuninit(&self) -> &MaybeUninit<Self>;
	fn to_maybeuninit_mut(&mut self) -> &mut MaybeUninit<Self>;
}

impl<T> ToMaybeUninit for T {
	#[inline]
	fn to_maybeuninit(&self) -> &MaybeUninit<Self> {
		unsafe { &*(self as *const T as *const MaybeUninit<T>) }
	}

	#[inline]
	fn to_maybeuninit_mut(&mut self) -> &mut MaybeUninit<Self> {
		unsafe { &mut *(self as *mut T as *mut MaybeUninit<T>) }
	}
}
