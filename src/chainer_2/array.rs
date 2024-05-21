use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn new_uninit() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::uninit()
				.assume_init()
				.into()
		}
	}

	pub fn new_zeroed() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::zeroed()
				.assume_init()
				.into()
		}
	}
}

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { inner: value }
	}
}
