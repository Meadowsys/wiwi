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

use std::mem::MaybeUninit;

// TODO: make this some kind of package wide trait? or feature? I dunno
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

pub trait IntoChainer {
	type Chain;
	fn into_chainer(self) -> Self::Chain;
}
