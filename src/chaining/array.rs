use std::{ mem::MaybeUninit, ptr };
use super::{ IntoChainer, ToMaybeUninit as _ };

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn new_uninit(len: usize) -> ArrayChain<MaybeUninit<T>, N> {
		unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init().into() }
	}

	pub fn new_zeroed(len: usize) -> ArrayChain<MaybeUninit<T>, N> {
		unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::zeroed().assume_init().into() }
	}

	// from_fn, from_mut, from_ref, try_from_fn
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(N);
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	pub fn is_empty_uninit(mut self, out: &mut MaybeUninit<bool>) -> Self {
		let mut len = MaybeUninit::uninit();
		self = self.len_uninit(&mut len);
		out.write(unsafe { len.assume_init() == 0 });
		self
	}

	pub fn with_first<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}
	/*
	map, try map
	as_slice
	as_mut_slice
	each_ref
	each_mut
	split_array_ref
	split_array_mut
	rsplit_array_ref
	rsplit_array_mut
	*/
}

impl<const N: usize> ArrayChain<u8, N> {
	// as_ascii/unchecked
}

impl<T, const N: usize> ArrayChain<MaybeUninit<T>, N> {
	pub unsafe fn assume_init(self) -> ArrayChain<T, N> {
		// TODO: this is subpar (its copying), but I can't find a better way to do it?
		// all ways to do it seem to be unstable (transmute is too dumb, transmute_unchecked
		// is unstable and likely won't ever be stable, MaybeUninit::array_assume_init
		// is unstable (it uses transmute_unchecked internally))
		ptr::read(&self.inner as *const [MaybeUninit<T>; N] as *const [T; N]).into()
	}
	// transpose
}

// o.0
// impl<T, const N: usize, const N2: usize> ArrayChain<[T; N2], N> {
// 	pub fn flatten(self) -> ArrayChain<T, { N * N2 }> {}
// }

impl<T, const N: usize> IntoChainer for [T; N] {
	type Chain = ArrayChain<T, N>;
	fn into_chainer(self) -> ArrayChain<T, N> {
		self.into()
	}
}

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	fn from(inner: [T; N]) -> Self {
		Self { inner }
	}
}

/*
AsRef/AsMut for itself, [T; N], [T], other chain types
arraylike?????? (and for boxed version of )

asref/mut to self, slice, slice chainers
borrow/mut
clone, debug, default
from array for cow slice?, &/mut [T; N] for Vec

TODO: next: btreemap thing
*/
