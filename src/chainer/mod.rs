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

mod slice_boxed;
pub use slice_boxed::SliceBoxedChain;

mod slice_mut;
pub use slice_mut::SliceMutChain;

mod slice_ref;
pub use slice_ref::SliceRefChain;

mod vec;
pub use vec::VecChain;

pub trait IntoChainer {
	type Chain;
	fn into_chainer(self) -> Self::Chain;
}
