use std::mem::MaybeUninit;

pub trait ToMaybeUninit: Sized {
	unsafe fn to_maybeuninit(&self) -> &MaybeUninit<Self>;
	unsafe fn to_maybeuninit_mut(&mut self) -> &mut MaybeUninit<Self>;

	/// # Safety
	///
	/// We can safely assume we have exclusive access to the value at this
	/// location, since we have a mutable (exclusive) reference to it. HOWEVER,
	/// that reference is to a location of an actual value which likely has
	/// initialisation variations, that will be dropped in this function.
	/// The value behind this reference is effectively uninitialised, and you
	/// _MUST_ write to the reference returned, otherwise you will cause a
	/// use-after-free / double drop.
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
