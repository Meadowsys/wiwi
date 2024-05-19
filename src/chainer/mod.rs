// dunno why I felt the need to document it, but...
// mod layout:
// - use statements (of course)
// - struct
// - impl block: constructor fns
// - impl block: conversion fns
// - impl block: chaining fns
// - impl blocks: specific types (eg. ArrayChain<MaybeUninit<T>>)
// - impl IntoChainer, From, AsRef/AsMut, others

mod array;
pub use array::ArrayChain;

mod array_mut;
pub use array_mut::ArrayMutChain;

mod array_ref;
pub use array_ref::ArrayRefChain;

mod slice_boxed;
pub use slice_boxed::SliceBoxedChain;

mod slice_mut;
pub use slice_mut::SliceMutChain;

mod slice_ref;
pub use slice_ref::SliceRefChain;

mod vec;
pub use vec::VecChain;

/// Trait providing [`into_chainer`], to convert any
/// supported type into one with a chaining API.
///
/// Every type that implements this trait has a "preferred" chain type, declared
/// by the [`IntoChainer::Chain`] associated type, and calling [`into_chainer`]
/// will convert it to that chain type.
///
/// [`into_chainer`]: IntoChainer::into_chainer
pub trait IntoChainer {
	/// The preferred chainer of this type.
	type Chain;

	/// Converts `self` into its chain type.
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
	fn into_chainer(self) -> Self::Chain;
}
