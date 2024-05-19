use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ MaybeUninit, size_of };
use std::slice;
use super::{ IntoChainer, SliceMutChain, SliceRefChain };

// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct SliceBoxedChain<T> {
	inner: Box<[T]>
}

/// Constructor functions
impl<T> SliceBoxedChain<T> {
	pub fn new_uninit(len: usize) -> SliceBoxedChain<MaybeUninit<T>> {
		// TODO: refactor to use chaining vec?
		// with_capacity in vec won't allocate if T is ZST (capacity is lorge)
		let mut vec = Vec::with_capacity(len);

		// SAFETY: MaybeUninit<T> doesn't have any initialisation requirement
		unsafe { vec.set_len(len) }

		vec.into_boxed_slice().into()
	}

	pub fn new_zeroed(len: usize) -> SliceBoxedChain<MaybeUninit<T>> {
		let mut slice = Self::new_uninit(len);

		// SAFETY: MaybeUninit<T> doesn't have any initialisation requirement
		// (so zeroed is fine)
		unsafe { slice.inner.as_mut_ptr().write_bytes(0, len) }

		slice
	}

	// TODO: nightly try_new_uninit_slice
	// TODO: nightly try_new_zeroed_slice
}

// TODO: allocator constructors
// impl<T> SliceBoxedChain<T> {
// 	// TODO: nightly new_uninit_slice_in
// 	// TODO: nightly new_zeroed_slice_in
// }

/// Conversion functions
impl<T> SliceBoxedChain<T> {
	pub fn into_inner(self) -> Box<[T]> {
		self.inner
	}

	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.inner
	}

	pub fn as_ref_slice_chainer(&self) -> &SliceRefChain<T> {
		(*self.inner).into()
	}

	pub fn as_mut_slice_chainer(&mut self) -> &mut SliceMutChain<T> {
		(&mut *self.inner).into()
	}
}

/// Chaining functions
impl<T> SliceBoxedChain<T> {
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	pub fn is_empty_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_empty());
		self
	}

	// TODO: first, first mut, hmmm
	pub fn with_first<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}

	/*
	split_first/mut
	split_last/mut
	last/mut
	first_chunk/mut
	split_first_chunk/mut
	split_last_chunk/mut
	last_chunk/mut
	get/mut
	get_unchecked/mut
	as_ptr/as_mut_ptr
	as_ptr_range/as_mut_ptr_range
	swap/unchecked
	reverse
	iter/mut
	windows
	chunks/mut
	chunks_exact/mut
	as_chunks/unchecked
	as_rchunks
	array_chunks/mut
	as_chunks_unchecked_mut
	as_chunks_mut
	as_rchunks_mut
	array_windows
	rchunks/mut
	rchunks_exact/mut
	chunk_by_mut(non mut?)
	split_at/mut/unchecked
	split_at_checked/split_at_checked_mut
	split/mut
	split_inclusive/mut
	rsplit/mut
	splitn/mut
	rsplitn/mut
	split_once
	rsplit_once
	contains
	starts/ends_with
	strip_prefix/suffix
	binary_search/by/key
	sort_unstable/by/key
	select_nth_unstable/by/key
	partition_dedup/by/key
	rotate_left/right
	fill/with
	clone_from_slice
	copy_from_slice
	copy_within
	clone_within (not in std???)
	swap_with_slice
	align_to/mut
	nightly as_simd/mut?
	is_sorted/by/key
	partition_point
	take/mut
	take_first/mut
	take_last/mut
	get_many_unchecked_mut
	get_many_mut
	get_many/unchecked (not in std??????????????????)
	sort/by/key
	sort_by_cached_key
	to_vec/in
	into_vec/in(in is not std? not even nightly??)
	repeat
	concat
	join
	*/
}

// TODO: allocator param
impl<T> SliceBoxedChain<MaybeUninit<T>> {
	pub unsafe fn assume_init(self) -> SliceBoxedChain<T> {
		let raw = Box::into_raw(self.inner);
		let raw = raw as *mut [T];
		Box::from_raw(raw).into()
	}
}

// TODO: [AsciiChar] as_str, as_bytes,
// TODO: SliceBoxedChain<AsciiChar> into_boxed_str, into_string, into_boxed_bytes(??), into_vec

impl<T, const N: usize> SliceBoxedChain<[T; N]> {
	pub fn flatten(self) -> SliceBoxedChain<T> {
		// taken from std's flatten fn
		// TODO: use SizedTypeProperties or slice `flatten`, whichever gets stabilised first
		let len = if size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			unsafe { self.inner.len().unchecked_mul(N) }
		};

		let raw = Box::into_raw(self.inner);
		let raw = raw as *mut [T] as *mut T;

		unsafe {
			let raw = slice::from_raw_parts_mut(raw, len);
			Box::from_raw(raw).into()
		}
	}
}

impl SliceBoxedChain<f32> {
	// sort floats
}

impl SliceBoxedChain<f64> {
	// sort floats
}

impl SliceBoxedChain<u8> {
	// is_ascii
	// nightly as_ascii
	// nightly as_ascii_unchecked
	// eq_ignore_ascii_case
	// make_ascii_uppercase/lowercase
	// escape_ascii
	// trim_ascii
	// trim_ascii/start/end

	// to_ascii_uppercase/lowercase
}

impl<T> IntoChainer for Box<[T]> {
	type Chain = SliceBoxedChain<T>;
	fn into_chainer(self) -> SliceBoxedChain<T> {
		self.into()
	}
}

// TODO: allocator param
impl<T> From<Box<[T]>> for SliceBoxedChain<T> {
	#[inline]
	fn from(inner: Box<[T]>) -> Self {
		Self { inner }
	}
}

// AsRef/AsMut for itself, Box<[T]>, [T], the other chain types
// bufread for &[u8]?
// Clone, Copy, Debug, default, From<&[T]> where clone,
// Concat?
// ? from arc, rc, cow?
// TODO: not done looking for traits
