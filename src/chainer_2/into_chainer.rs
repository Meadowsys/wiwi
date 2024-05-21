use super::{
	VecChain,
	SliceBoxChain,
	SliceRefChain,
	SliceMutChain,
	ArrayChain,
	ArrayRefChain,
	ArrayMutChain
};

/// Trait providing [`into_chainer`], to convert any
/// supported type into one with a chaining API.
///
/// Every type that implements this trait has a "preferred" chain type, declared
/// by the [`IntoChainer::Chain`] associated type, and calling [`into_chainer`]
/// will convert it to that chain type.
///
/// [`into_chainer`]: IntoChainer::into_chainer
pub trait IntoChainer: Sized {
	/// The preferred chainer of this type
	type Chain: From<Self>;

	/// Converts `self` into its associated chain type
	///
	/// # Examples
	///
	/// ```
	/// # // bruh
	/// # use wiwi::chainer::{ IntoChainer, VecChain };
	/// let vec: Vec<String> = Vec::new();
	/// let chain: VecChain<String> = vec.into_chainer();
	/// // ...
	/// // do funny things with chaining API c:
	/// ```
	#[inline]
	fn into_chainer(self) -> Self::Chain {
		self.into()
	}
}

impl<T> IntoChainer for Vec<T> {
	type Chain = VecChain<T>;
}

impl<T> IntoChainer for Box<[T]> {
	type Chain = SliceBoxChain<T>;
}

impl<'h, T> IntoChainer for &'h [T] {
	type Chain = &'h SliceRefChain<T>;
}

impl<'h, T> IntoChainer for &'h mut [T] {
	type Chain = &'h mut SliceMutChain<T>;
}

impl<T, const N: usize> IntoChainer for [T; N] {
	type Chain = ArrayChain<T, N>;
}

impl<'h, T, const N: usize> IntoChainer for &'h [T; N] {
	type Chain = &'h ArrayRefChain<T, N>;
}

impl<'h, T, const N: usize> IntoChainer for &'h mut [T; N] {
	type Chain = &'h mut ArrayMutChain<T, N>;
}
