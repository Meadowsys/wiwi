use std::mem::MaybeUninit;

pub trait ToMaybeUninit: Sized {
	unsafe fn to_maybeuninit(&self) -> &MaybeUninit<Self>;
	unsafe fn to_maybeuninit_mut(&mut self) -> &mut MaybeUninit<Self>;

	unsafe fn to_maybeuninit_drop(&mut self) -> &mut MaybeUninit<Self>;
}

impl<T> ToMaybeUninit for T {
	#[inline]
	unsafe fn to_maybeuninit(&self) -> &MaybeUninit<Self> {
		&*(self as *const T as *const MaybeUninit<T>)
	}

	#[inline]
	unsafe fn to_maybeuninit_mut(&mut self) -> &mut MaybeUninit<Self> {
		&mut *(self as *mut T as *mut MaybeUninit<T>)
	}

	#[inline]
	unsafe fn to_maybeuninit_drop(&mut self) -> &mut MaybeUninit<Self> {
		let maybeuninit_ref = self.to_maybeuninit_mut();

		let value = maybeuninit_ref.assume_init_read();
		drop(value);

		maybeuninit_ref
	}
}
