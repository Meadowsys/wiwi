use std::{ array, mem::MaybeUninit };
use super::SliceChain;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

// TODO: why do I need this?
// impl<T> ArrayChain<T, 1> {
// 	pub fn from_ref(s: &T) -> &Self {
// 		array::from_ref(s).into()
// 	}
//
// 	pub fn from_mut(s: &mut T) -> &mut Self {
// 		array::from_mut(s).into()
// 	}
// }

impl<T, const N: usize> ArrayChain<T, N> {
	#[inline]
	pub fn from_array(array: [T; N]) -> Self {
		Self { inner: array }
	}

	#[inline]
	pub fn from_fn<F>(cb: F) -> Self
	where
		F: FnMut(usize) -> T
	{
		array::from_fn(cb).into()
	}

	#[inline]
	pub fn map<F, U>(self, f: F) -> ArrayChain<U, N>
	where
		F: FnMut(T) -> U
	{
		self.inner.map(f).into()
	}

	// TODO: nightly try_map

	#[inline]
	pub fn as_slice(&self) -> &SliceChain<T> {
		// <&SliceChain<T>>::from(&self.inner as &[T])
		(&self.inner as &[T]).into()
	}

	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut SliceChain<T> {
		(&mut self.inner as &mut [T]).into()
	}

	// TODO: upgrade toolchain then each_ref
	// TODO: upgrade toolchain then each_mut
	// TODO: nightly split_array_ref
	// TODO: nightly split_array_mut
	// TODO: nightly rsplit_array_ref
	// TODO: nightly rsplit_array_mut
}

// TODO: trait impls

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	#[inline]
	fn from(inner: [T; N]) -> Self {
		Self { inner }
	}
}

// TODO: why do I need this?
// impl<T, const N: usize> From<&[T; N]> for &ArrayChain<T, N> {
// 	fn from(value: &[T; N]) -> Self {
// 		unsafe { &*(value as *const [T; N] as *const ArrayChain<T, N>) }
// 	}
// }

// impl<T, const N: usize> From<&mut [T; N]> for &mut ArrayChain<T, N> {
// 	fn from(value: &mut [T; N]) -> Self {
// 		unsafe { &mut *(value as *mut [T; N] as *mut ArrayChain<T, N>) }
// 	}
// }

impl<const N: usize> ArrayChain<u8, N> {
	// TODO: nightly as_ascii
	// TODO: nightly as_ascii_unchecked
}

impl<T, const N: usize> ArrayChain<MaybeUninit<T>, N> {
	// TODO: nightly transpose
}

impl<T, const N: usize> AsRef<ArrayChain<T, N>> for ArrayChain<T, N> {
	#[inline]
	fn as_ref(&self) -> &Self {
		self
	}
}

impl<T, const N: usize> AsMut<ArrayChain<T, N>> for ArrayChain<T, N> {
	#[inline]
	fn as_mut(&mut self) -> &mut Self {
		self
	}
}

impl<T, const N: usize> AsRef<[T; N]> for ArrayChain<T, N> {
	#[inline]
	fn as_ref(&self) -> &[T; N] {
		&self.inner
	}
}

impl<T, const N: usize> AsMut<[T; N]> for ArrayChain<T, N> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; N] {
		&mut self.inner
	}
}

impl<T, const N: usize> AsRef<SliceChain<T>> for ArrayChain<T, N> {
	#[inline]
	fn as_ref(&self) -> &SliceChain<T> {
		self.as_slice()
	}
}

impl<T, const N: usize> AsMut<SliceChain<T>> for ArrayChain<T, N> {
	#[inline]
	fn as_mut(&mut self) -> &mut SliceChain<T> {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> AsRef<[T]> for ArrayChain<T, N> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		&self.inner
	}
}

impl<T, const N: usize> AsMut<[T]> for ArrayChain<T, N> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}

impl<T, const N: usize> ArrayChain<T, N> {
	#[inline]
	pub fn as_array(&self) -> &[T; N] {
		&self.inner
	}

	#[inline]
	pub fn as_mut_array(&mut self) -> &mut [T; N] {
		&mut self.inner
	}

	#[inline]
	pub fn into_inner(self) -> [T; N] {
		self.inner
	}
}
