use std::mem::{ forget, MaybeUninit, size_of };
use super::{ IntoChainer, SliceMutChain, SliceRefChain, ToMaybeUninit as _ };

// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

/// Constructor functions
impl<T> VecChain<T> {
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}

	// TODO: nightly try_with_capacity

	// TODO: from_raw_parts
}
// TODO: allocator constructors
// impl<T> VecChain<T> {
// 	// TODO: nightly new_in
// 	// TODO: nightly with_capacity_in
// 	// TODO: nightly try_with_capacity_in
// 	// TODO: nightly from_raw_parts_in
// }

/// Chaining functions
// TODO: allocator param
impl<T> VecChain<T> {
	// TODO: nightly into_raw_parts
	// TODO: nightly into_raw_parts_with_alloc

	pub fn capacity(self, out: &mut usize) -> Self {
		self.capacity_uninit(out.to_maybeuninit_mut())
	}

	pub fn capacity_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.capacity());
		self
	}

	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}

	// TODO: figure out try_methods and errors
	// cause if it errors, the thing is just gonna get consumed?
	// maybe we return (Error, Self)? but that's clunky hmm

	// try_reserve
	// try_reserve_exact

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	// TODO: into_boxed_slice (maybe should chain?)

	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}
	/*
	as_slice/as_mut_slice
	as_ptr/as_mut_ptr
	nightly allocator
	set_len
	swap_remove
	insert
	remove
	retain/mut
	dedup_by
	dedup_by_key
	push
	push_within_capacity????
	pop
	append
	drain
	clear
	len
	is_empty
	split_off
	resize_with
	leak?
	spare_capacity_mut
	split_at_spare_mut?
	splice
	extract_if


	as_str
	as_bytes
	flatten/mut
	len
	is_empty
	first/mut
	split_first/mut
	split_last/mut
	last
	last_mut
	first_chunk/mut
	split_first_chunk/mut
	split_last_chunk/mut
	last_chunk/mut
	get/mut
	get_unchecked/mut
	swap_unchecked (checked/panic?????)
	reverse
	iter/mut
	windows
	chunks/mut
	chunks_exact/mut
	as_chunks_unchecked
	as_chunks
	as_rchunks
	array_chunks/mut
	as_chunks_unchecked_mut
	as_chunks_mut
	as_rchunks_mut
	array_windows
	rchunks/mut
	rchunks_exact/mut
	chunk_by/mut
	split_at/mut
	split_at_unchecked
	split_at_mut_unchecked
	split_at_checked
	split_at_mut_checked
	split/mut
	split_inclusive/mut
	rsplit/mut
	splitn/mut
	rsplitn/mut
	split_once
	rsplit_once
	contains
	starts_with
	ends_with
	strip_prefix/suffix
	binary_search/by/key
	sort_unstable/by/key
	select_nth_unstable/by/key
	partition_dedup/by/key
	rotate_left/right
	fill
	fill_with
	clone_from_slice
	copy_from_slice
	copy_within
	clone_within (not in std)?
	swap_with_slice
	align_to/mut
	nightly as_simd/mut
	is_sorted/by/key
	partition_point
	take/mut
	take_first/mut
	take_last/mut
	get_many_unchecked_mut
	get_many_mut
	get_many/get_many_unchecked (non mut? not in std?)
	sort_floats
	is_ascii
	as_ascii/unchecked
	eq_ignore_ascii_case
	make_ascii_uppercase/lowercase
	escape_ascii
	trim_ascii
	trim_ascii_start/end
	sort/by/key
	sort_by_cached_key
	to_vec/in????? no lol just clone?
	repeat
	concat
	join
	to_ascii_uppercase/lowercase
	*/
}

/// Conversion functions
impl<T> VecChain<T> {
	pub fn into_inner(self) -> Vec<T> {
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

// TODO: allocator param
impl<T: Clone> VecChain<T> {
	/*
	resize
	extend_from_slice
	extend_from_within
	*/
}

// TODO: allocator param
impl<T, const N: usize> VecChain<[T; N]> {
	pub fn flatten(mut self) -> VecChain<T> {
		let (len, cap) = if size_of::<T>() == 0 {
			let len = self.inner.len()
				.checked_mul(N)
				.expect("vec len overflow");
			(len, usize::MAX)
		} else {
			unsafe { (
				self.inner.len().unchecked_mul(N),
				self.inner.capacity().unchecked_mul(N)
			) }
		};

		// TODO: switch to into_raw_parts impl when stabilised
		// let (ptr, _len, _capacity) = self.inner.into_raw_parts();

		let ptr = self.inner.as_mut_ptr();
		forget(self);

		let ptr = ptr as *mut T;
		unsafe { Vec::from_raw_parts(ptr, len, cap).into() }
	}
}

// TODO: allocator param
impl<T: PartialEq> VecChain<T> {
	// dedup
}

impl<T> IntoChainer for Vec<T> {
	type Chain = VecChain<T>;
	fn into_chainer(self) -> VecChain<T> {
		self.into()
	}
}

impl<T> From<Vec<T>> for VecChain<T> {
	#[inline]
	fn from(inner: Vec<T>) -> Self {
		Self { inner }
	}
}
/*
AsRef/AsMut for itself, Vec<T>, [T], other chain types
Borrow/mut for slice chainer?
whatever ArrayLike is
clone debug default
drop?? nah
extend T, &T
from &[T], &[T; N], other chainers, cow<[T]>
from muts?
from str, String
from owned array
from binary heap? boxed slice and chainer, cstring for u8? cow slice?
from nonzero u8 for cstring (probably some other crate, I dunno, if we cstring support?)
from vec for cow
from rc, arc,
TODO: stopped around here (from vec for Rc<[T]> something like that)
*/
