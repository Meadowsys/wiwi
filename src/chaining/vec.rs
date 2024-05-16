use std::mem::{ self, MaybeUninit};
use std::ops::RangeBounds;
use std::{ slice, vec };
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

	// TODO: as_ptr / as_mut_ptr?
	// TODO: nightly allocator
	// I dunno where else to put these, I'll figure it out when the time comes lol
	// maybe allocator goes with len/capacity/etc?
}

/// Chaining functions
// TODO: allocator param
impl<T> VecChain<T> {
	// TODO: nightly into_raw_parts
	// TODO: nightly into_raw_parts_with_alloc

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

	// TODO: try_reserve
	// TODO: try_reserve_exact

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

	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
		self.inner.set_len(new_len);
		self
	}

	pub fn with_first<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}

	pub fn swap_remove(mut self, index: usize, out: &mut T) -> Self {
		self.swap_remove_uninit(index, out.to_maybeuninit_mut())
	}

	pub fn swap_remove_uninit(mut self, index: usize, out: &mut MaybeUninit<T>) -> Self {
		out.write(self.inner.swap_remove(index));
		self
	}

	pub fn insert(mut self, index: usize, element: T) -> Self {
		self.inner.insert(index, element);
		self
	}

	pub fn remove(mut self, index: usize, out: &mut T) -> Self {
		self.remove_uninit(index, out.to_maybeuninit_mut())
	}

	pub fn remove_uninit(mut self, index: usize, out: &mut MaybeUninit<T>) -> Self {
		out.write(self.inner.remove(index));
		self
	}

	pub fn retain<F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> bool
	{
		self.inner.retain(f);
		self
	}

	pub fn retain_mut<F>(mut self, f: F) -> Self
	where
		F: FnMut(&mut T) -> bool
	{
		self.inner.retain_mut(f);
		self
	}

	pub fn dedup_by<F>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut T, &mut T) -> bool
	{
		self.inner.dedup_by(same_bucket);
		self
	}

	pub fn dedup_by_key<F, K>(mut self, key: F) -> Self
	where
		F: FnMut(&mut T) -> K,
		K: PartialEq
	{
		self.inner.dedup_by_key(key);
		self
	}

	pub fn push(mut self, value: T) -> Self {
		self.inner.push(value);
		self
	}

	// TODO: push_within_capacity

	pub fn pop(mut self, out: &mut Option<T>) -> Self {
		self.pop_uninit(out.to_maybeuninit_mut())
	}

	pub fn pop_uninit(mut self, out: &mut MaybeUninit<Option<T>>) -> Self {
		out.write(self.inner.pop());
		self
	}

	pub fn append(mut self, other: &mut Self) -> Self {
		self.inner.append(&mut other.inner);
		self
	}

	pub fn append_vec(mut self, other: &mut Vec<T>) -> Self {
		self.inner.append(other);
		self
	}

	pub fn drain_with<R, F>(mut self, range: R, f: F) -> Self
	where
		R: RangeBounds<usize>,
		F: FnOnce(vec::Drain<T>)
	{
		f(self.inner.drain(range));
		self
	}

	pub fn clear(mut self) -> Self {
		self.inner.clear();
		self
	}

	// TODO: split_off

	pub fn resize_with<F>(mut self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> T
	{
		self.inner.resize_with(new_len, f);
		self
	}

	pub fn leak<'h>(self) -> &'h mut SliceMutChain<T> {
		self.inner.leak().into()
	}

	pub fn with_spare_capacity_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut SliceMutChain<MaybeUninit<T>>)
	{
		f(self.inner.spare_capacity_mut().into());
		self
	}

	pub fn with_split_at_spare_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut SliceMutChain<T>, &mut SliceMutChain<MaybeUninit<T>>)
	{
		// TODO: call Vec impl when stabilised
		let (init, spare) = unsafe {
			let ptr = self.inner.as_mut_ptr();
			let len = self.inner.len();

			let spare_ptr = ptr.add(len) as *mut MaybeUninit<T>;
			let spare_len = self.inner.capacity() - len;

			(
				slice::from_raw_parts_mut(ptr, len),
				slice::from_raw_parts_mut(spare_ptr, spare_len)
			)
		};

		f(init.into(), spare.into());
		self
	}

	/*
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
		let (len, cap) = if mem::size_of::<T>() == 0 {
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
		mem::forget(self);

		let ptr = ptr as *mut T;
		unsafe { Vec::from_raw_parts(ptr, len, cap).into() }
	}
}

// TODO: allocator param
impl<T: PartialEq> VecChain<T> {
	pub fn dedup(mut self) -> Self {
		self.inner.dedup();
		self
	}
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
