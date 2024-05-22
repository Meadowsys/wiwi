use crate::to_maybeuninit::ToMaybeUninit as _;
use std::cmp::Ordering;
use std::mem::{ self, MaybeUninit };
use std::ops::RangeBounds;
use std::slice::{ self, SliceIndex };
use std::{ ptr, vec };
use super::{ IntoChainer, SliceMutChain, SliceRefChain };

/// Vec type that provides a chaining API.
///
/// It contains similar methods as [`Vec`], but in some cases, the API needs to
/// differ to accomodate the chaining API.
// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
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
	pub fn with_split_at_spare_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut SliceMutChain<T>, &mut SliceMutChain<MaybeUninit<T>>)
	{
		// TODO: call Vec impl when it is stabilised
		unsafe {
			let ptr = self.inner.as_mut_ptr();
			let len = self.inner.len();

			let spare_ptr = ptr.add(len) as *mut MaybeUninit<T>;
			let spare_len = self.inner.capacity() - len;

			let init = slice::from_raw_parts_mut(ptr, len).into();
			let spare = slice::from_raw_parts_mut(spare_ptr, spare_len).into();

			f(init, spare);
		}

		self
	}

	pub fn splice_with<R, I, F>(mut self, range: R, replace_with: I, f: F) -> Self
	where
		R: RangeBounds<usize>,
		I: IntoIterator<Item = T>,
		F: FnOnce(vec::Splice<I::IntoIter>)
	{
		f(self.inner.splice(range, replace_with));
		self
	}

	// TODO: nightly feature, would have to roll our own if we wanted it right now
	// TODO: name's kinda weird, do I wanna change it?
	// TODO: mut/nonmut versions?
	// pub fn extract_if_with<F, F2>(mut self, filter: F, f: F) -> Self
	// where
	// 	F: FnMut(&mut T) -> bool,
	// 	F2: FnOnce(vec::ExtractIf<T, F>)
	// {
	// 	f(self.inner.extract_if(filter));
	// 	self
	// }


	// TODO: (nightly) as_str
	// TODO: (nightly) as_bytes
	// ^ think those are for AsciiChar

	pub fn with_first<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.first());
		self
	}

	pub fn with_first_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}

	pub fn with_last<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.last());
		self
	}

	pub fn with_last_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.last_mut());
		self
	}

	pub fn split_first_with<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<(&T, &SliceRefChain<T>)>)
	{
		f(self.inner.split_first().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_first_mut_with<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<(&mut T, &mut SliceMutChain<T>)>)
	{
		f(self.inner.split_first_mut().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_last_with<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<(&T, &SliceRefChain<T>)>)
	{
		f(self.inner.split_last().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_last_mut_with<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<(&mut T, &mut SliceMutChain<T>)>)
	{
		f(self.inner.split_last_mut().map(|(a, b)| (a, b.into())));
		self
	}

	// TODO: do once we have array ref chainers
	// doc link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.first_chunk
	// pub fn with_first_chunk<F, const N: usize>(self, f: F) -> Self
	// pub fn with_first_chunk_mut<F, const N: usize>(mut self, f: F) -> Self
	// pub fn with_last_chunk<F, const N: usize>(self, f: F) -> Self
	// pub fn with_last_chunk_mut<F, const N: usize>(mut self, f: F) -> Self
	// pub fn split_first_chunk_with<F, const N: usize>(self, f: F) -> Self
	// pub fn split_first_chunk_mut_with<F, const N: usize>(mut self, f: F) -> Self
	// pub fn split_last_chunk_with<F, const N: usize>(self, f: F) -> Self
	// pub fn split_last_chunk_mut_with<F, const N: usize>(mut self, f: F) -> Self

	pub fn with<I, F>(self, index: I, f: F) -> Self
	where
		I: SliceIndex<[T]>,
		F: FnOnce(Option<&I::Output>)
	{
		f(self.inner.get(index));
		self
	}

	pub fn with_mut<I, F>(mut self, index: I, f: F) -> Self
	where
		I: SliceIndex<[T]>,
		F: FnOnce(Option<&mut I::Output>)
	{
		f(self.inner.get_mut(index));
		self
	}

	pub unsafe fn with_unchecked<I, F>(self, index: I, f: F) -> Self
	where
		I: SliceIndex<[T]>,
		F: FnOnce(&I::Output)
	{
		f(self.inner.get_unchecked(index));
		self
	}

	pub unsafe fn with_unchecked_mut<I, F>(mut self, index: I, f: F) -> Self
	where
		I: SliceIndex<[T]>,
		F: FnOnce(&mut I::Output)
	{
		f(self.inner.get_unchecked_mut(index));
		self
	}

	pub fn swap(mut self, a: usize, b: usize) -> Self {
		self.inner.swap(a, b);
		self
	}

	pub unsafe fn swap_unchecked(mut self, a: usize, b: usize) -> Self {
		// TODO: replace with Vec::swap_unchecked call when it's stabilised
		let ptr = self.inner.as_mut_ptr();
		ptr::swap(ptr.add(a), ptr.add(b));
		self
	}

	// TODO: swap_with_slice (chains and self and slice?)

	pub fn reverse(mut self) -> Self {
		self.inner.reverse();
		self
	}

	pub fn with_iter<F>(self, f: F) -> Self
	where
		F: FnOnce(slice::Iter<T>)
	{
		f(self.inner.iter());
		self
	}

	pub fn with_iter_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(slice::IterMut<T>)
	{
		f(self.inner.iter_mut());
		self
	}

	pub fn with_windows<F>(self, size: usize, f: F) -> Self
	where
		F: FnOnce(slice::Windows<T>)
	{
		f(self.inner.windows(size));
		self
	}

	// TODO: nightly array_windows

	pub fn with_chunks_iter<F>(self, chunk_size: usize, f: F) -> Self
	where
		F: FnOnce(slice::Chunks<T>)
	{
		f(self.inner.chunks(chunk_size));
		self
	}

	pub fn with_chunks_iter_mut<F>(mut self, chunk_size: usize, f: F) -> Self
	where
		F: FnOnce(slice::ChunksMut<T>)
	{
		f(self.inner.chunks_mut(chunk_size));
		self
	}

	pub fn with_chunks_exact_iter<F>(self, chunk_size: usize, f: F) -> Self
	where
		F: FnOnce(slice::ChunksExact<T>)
	{
		f(self.inner.chunks_exact(chunk_size));
		self
	}

	pub fn with_chunks_exact_iter_mut<F>(mut self, chunk_size: usize, f: F) -> Self
	where
		F: FnOnce(slice::ChunksExactMut<T>)
	{
		f(self.inner.chunks_exact_mut(chunk_size));
		self
	}

	pub fn with_chunks<F, const N: usize>(mut self, f: F) -> Self
	where
		F: FnOnce(&[[T; N]], &[T])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			let mut len = MaybeUninit::uninit();
			self = self.len_uninit(&mut len);
			let len = len.assume_init();

			let full_chunks = len / N;
			let partial_len = len % N;

			let ptr = self.inner.as_ptr();
			let ptr_partial = ptr.add(full_chunks * N);

			let full_chunks = slice::from_raw_parts(ptr as *const [T; N], full_chunks);
			let partial_chunk = slice::from_raw_parts(ptr_partial, partial_len);

			f(full_chunks, partial_chunk);
		}

		self
	}

	// TODO: nightly as_chunks/unchecked, as_chunks_mut/unchecked, as_rchunks/mut, array_chunks/mut

	// TODO: rchunks/mut
	// TODO: rchunks_exact/mut
	// TODO: chunk_by/mut
	// TODO: split_at/mut
	// TODO: split_at_unchecked
	// TODO: split_at_mut_unchecked
	// TODO: split_at_checked
	// TODO: split_at_mut_checked
	// TODO: split/mut
	// TODO: split_inclusive/mut
	// TODO: rsplit/mut
	// TODO: splitn/mut
	// TODO: rsplitn/mut
	// TODO: split_once
	// TODO: rsplit_once

	pub fn contains(self, x: &T, out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.contains_uninit(x, out.to_maybeuninit_mut())
	}

	pub fn contains_uninit(self, x: &T, out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.contains(x));
		self
	}

	pub fn starts_with(self, needle: &[T], out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.starts_with_uninit(needle, out.to_maybeuninit_mut())
	}

	pub fn starts_with_uninit(self, needle: &[T], out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.starts_with(needle));
		self
	}

	pub fn ends_with(self, needle: &[T], out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.ends_with_uninit(needle, out.to_maybeuninit_mut())
	}

	pub fn ends_with_uninit(self, needle: &[T], out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.ends_with(needle));
		self
	}

	// TODO: strip_prefix/suffix

	pub fn binary_search(self, x: &T, out: &mut Result<usize, usize>) -> Self
	where
		T: Ord
	{
		self.binary_search_uninit(x, out.to_maybeuninit_mut())
	}

	pub fn binary_search_uninit(self, x: &T, out: &mut MaybeUninit<Result<usize, usize>>) -> Self
	where
		T: Ord
	{
		out.write(self.inner.binary_search(x));
		self
	}

	// TODO: binary_search/by/key
	// TODO: sort_unstable/by/key
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: rotate_left/right
	// TODO: fill
	// TODO: fill_with
	// TODO: clone_from_slice
	// TODO: copy_from_slice
	// TODO: copy_within
	// TODO: clone_within (not in std)?
	// TODO: align_to/mut
	// TODO: nightly as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: get_many/get_many_unchecked (non mut? not in std?)
	// TODO: sort_floats
	// TODO: is_ascii
	// TODO: as_ascii/unchecked
	// TODO: eq_ignore_ascii_case
	// TODO: make_ascii_uppercase/lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii
	// TODO: trim_ascii_start/end

	pub fn sort(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort();
		self
	}

	pub fn sort_by<F>(mut self, compare: F) -> Self
	where
		F: FnMut(&T, &T) -> Ordering
	{
		self.inner.sort_by(compare);
		self
	}

	pub fn sort_by_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_key(f);
		self
	}

	pub fn sort_by_cached_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_cached_key(f);
		self
	}

	// TODO: to_vec/in????? no lol just clone?

	pub fn repeat(mut self, n: usize) -> Self
	where
		T: Copy
	{
		// TODO: ...this can be more efficient (done in place?)
		self = self.inner.repeat(n).into();
		self
	}

	// TODO: concat
	// TODO: join
	// TODO: to_ascii_uppercase/lowercase
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
			// TODO: wait until 1.79 when this is stabilised
			// unsafe { (
			// 	self.inner.len().unchecked_mul(N),
			// 	self.inner.capacity().unchecked_mul(N)
			// ) }
			(
				self.inner.len() * N,
				self.inner.capacity() * N
			)
		};

		// TODO: switch to into_raw_parts impl when it is stabilised
		// let (ptr, _len, _capacity) = self.inner.into_raw_parts();

		let ptr = self.inner.as_mut_ptr();
		mem::forget(self);

		let ptr = ptr as *mut T;
		unsafe { Vec::from_raw_parts(ptr, len, cap).into() }
	}
}

impl<T> VecChain<T> {
	/// Sorts, then dedups, the vector chain.
	///
	/// Suggested by my good friend [Silk Rose] c:
	///
	/// # Examples
	///
	/// TODO
	///
	/// [Silk Rose]: https://github.com/silkrose
	pub fn sort_and_dedup(mut self) -> Self
	where
		T: Ord
	{
		self.sort().dedup()
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

impl<T> AsRef<VecChain<T>> for VecChain<T> {
	fn as_ref(&self) -> &VecChain<T> {
		self
	}
}

impl<T> AsMut<VecChain<T>> for VecChain<T> {
	fn as_mut(&mut self) -> &mut VecChain<T> {
		self
	}
}

impl<T> AsRef<Vec<T>> for VecChain<T> {
	fn as_ref(&self) -> &Vec<T> {
		&self.inner
	}
}

impl<T> AsMut<Vec<T>> for VecChain<T> {
	fn as_mut(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}
}

impl<T> AsRef<[T]> for VecChain<T> {
	fn as_ref(&self) -> &[T] {
		&self.inner
	}
}

impl<T> AsMut<[T]> for VecChain<T> {
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}

impl<T> AsRef<SliceRefChain<T>> for VecChain<T> {
	fn as_ref(&self) -> &SliceRefChain<T> {
		(*self.inner).into()
	}
}

impl<T> AsMut<SliceMutChain<T>> for VecChain<T> {
	fn as_mut(&mut self) -> &mut SliceMutChain<T> {
		(&mut *self.inner).into()
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
