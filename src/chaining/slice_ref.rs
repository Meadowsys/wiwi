use std::mem::{ MaybeUninit, size_of };
use std::slice;
use super::{ IntoChainer, ToMaybeUninit as _ };

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct SliceRefChain<T> {
	inner: [T]
}

/// Constructor functions
impl<T> SliceRefChain<T> {}

/// Chaining functions
impl<T> SliceRefChain<T> {
	pub fn len(&self, out: &mut usize) -> &Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(&self, out: &mut MaybeUninit<usize>) -> &Self {
		out.write(self.inner.len());
		self
	}

	pub fn is_empty(&self, out: &mut bool) -> &Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	pub fn is_empty_uninit(&self, out: &mut MaybeUninit<bool>) -> &Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn first<'h>(&'h self, out: &mut Option<&'h T>) -> &Self {
		self.first_uninit(out.to_maybeuninit_mut())
	}

	pub fn first_uninit<'h>(&'h self, out: &mut MaybeUninit<Option<&'h T>>) -> &Self {
		out.write(self.inner.first());
		self
	}

	pub fn with_first<F>(&self, f: F) -> &Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.first());
		self
	}

	// TODO: more (see SliceBoxedChain)
}

/// Conversion functions
impl<T> SliceRefChain<T> {
	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}
}

// TODO: [AsciiChar] as_str, as_bytes

impl<T, const N: usize> SliceRefChain<[T; N]> {
	pub fn flatten(&self) -> &SliceRefChain<T> {
		// taken from std's flatten fn
		// TODO: use SizedTypeProperties or slice `flatten`, whichever gets stabilised first
		let len = if size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			unsafe { self.inner.len().unchecked_mul(N) }
		};

		let ptr = self as *const SliceRefChain<[T; N]> as *const T;
		unsafe { slice::from_raw_parts(ptr, len).into() }
	}
}

impl<T> From<&[T]> for &SliceRefChain<T> {
	fn from(value: &[T]) -> Self {
		unsafe { &*(value as *const [T] as *const SliceRefChain<T>) }
	}
}

// TODO: trait impls (see boxed slice chainer)


















// /// instance methods
// impl<T> SliceChain<T> {
// 	pub fn len(&self, len: &mut usize) -> &Self {
// 		*len = self.inner.len();
// 		self
// 	}

// 	pub fn len_f<F>(&self, f: F) -> &Self
// 	where
// 		F: FnOnce(usize)
// 	{
// 		f(self.inner.len());
// 		self
// 	}
// }

// impl<T> SliceChain<T> {
// 	// TODO: len, is_empty, first/mut, split_first/mut, split_last/mut, last/mut
// 	// TODO: first_chunk/mut, split_first_chunk/mut
// 	// TODO: last_chunk/mut, split_last_chunk/mut
// 	// TODO: get/mut, get_unchecked/mut
// 	// TODO: as_ptr/mut, as_ptr_range/mut

// 	#[inline]
// 	pub fn swap(&mut self, a: usize, b: usize) -> &mut Self {
// 		self.inner.swap(a, b);
// 		self
// 	}

// 	// TODO: nightlly swap_unchecked

// 	#[inline]
// 	pub fn reverse(&mut self) -> &mut Self {
// 		self.inner.reverse();
// 		self
// 	}

// 	// TODO: iter/mut, windows, chunks/exact/mut
// 	// TODO: nightly as_chunks/umchecked/mut, as_rchunks/mut, array_chunks/mut, array_windows
// 	// TODO: rchunks/exact/mut, chunk_by/mut, split_at/unchecked/mut, split_at_checked/mut, split/inclusive/mut
// 	// TODO: rsplit/mut, splitn/mut, rsplitn/mut, split_once, rsplit_once, contains, starts_with, ends_with
// 	// TODO: strip_prefix, strip_suffix, binary_search/by/key

// 	#[inline]
// 	pub fn sort(&mut self) -> &mut Self
// 	where
// 		T: Ord
// 	{
// 		self.inner.sort();
// 		self
// 	}

// 	#[inline]
// 	pub fn sort_by<F>(&mut self, compare: F) -> &mut Self
// 	where
// 		F: FnMut(&T, &T) -> Ordering
// 	{
// 		self.inner.sort_by(compare);
// 		self
// 	}

// 	pub fn sort_by_key<K, F>(&mut self, f: F) -> &mut Self
// 	where
// 		F: FnMut(&T) -> K,
// 		K: Ord
// 	{
// 		self.inner.sort_by_key(f);
// 		self
// 	}

// 	pub fn sort_by_cached_key<K, F>(&mut self, f: F) -> &mut Self
// 	where
// 		F: FnMut(&T) -> K,
// 		K: Ord
// 	{
// 		self.inner.sort_by_cached_key(f);
// 		self
// 	}

// 	#[inline]
// 	pub fn sort_unstable(&mut self) -> &mut Self
// 	where
// 		T: Ord
// 	{
// 		self.inner.sort_unstable();
// 		self
// 	}

// 	#[inline]
// 	pub fn sort_unstable_by<F>(&mut self, compare: F) -> &mut Self
// 	where
// 		F: FnMut(&T, &T) -> Ordering
// 	{
// 		self.inner.sort_unstable_by(compare);
// 		self
// 	}

// 	#[inline]
// 	pub fn sort_unstable_by_key<K, F>(&mut self, f: F) -> &mut Self
// 	where
// 		F: FnMut(&T) -> K,
// 		K: Ord
// 	{
// 		self.inner.sort_unstable_by_key(f);
// 		self
// 	}

// 	// TODO: select_nth_unstable/by/key, partition_dedup/by/key

// 	#[inline]
// 	pub fn rotate_left(&mut self, mid: usize) -> &mut Self {
// 		self.inner.rotate_left(mid);
// 		self
// 	}

// 	#[inline]
// 	pub fn rotate_right(&mut self, k: usize) -> &mut Self {
// 		self.inner.rotate_right(k);
// 		self
// 	}

// 	#[inline]
// 	pub fn fill(&mut self, value: T) -> &mut Self
// 	where
// 		T: Clone
// 	{
// 		self.inner.fill(value);
// 		self
// 	}

// 	#[inline]
// 	pub fn fill_with<F>(&mut self, f: F) -> &mut Self
// 	where
// 		F: FnMut() -> T
// 	{
// 		self.inner.fill_with(f);
// 		self
// 	}

// 	#[inline]
// 	pub fn clone_from_slice(&mut self, src: &[T]) -> &mut Self
// 	where
// 		T: Clone
// 	{
// 		self.inner.clone_from_slice(src);
// 		self
// 	}

// 	#[inline]
// 	pub fn copy_from_slice(&mut self, src: &[T]) -> &mut Self
// 	where
// 		T: Copy
// 	{
// 		self.inner.copy_from_slice(src);
// 		self
// 	}

// 	#[inline]
// 	pub fn swap_with_slice(&mut self, other: &mut [T]) -> &mut Self {
// 		self.inner.swap_with_slice(other);
// 		self
// 	}

// 	// TODO: align_to/mut
// 	// TODO: nightly as_simg/mut
// 	// TODO: is_sorted/by/key, partition_point
// 	// TODO: nightly take/first/last/mut, get_many_unchecked_mut, get_many_mut

// 	#[inline]
// 	pub fn to_vec(&self) -> VecChain<T>
// 	where
// 		T: Clone
// 	{
// 		self.inner.to_vec().into()
// 	}

// 	// TODO: nightly to_vec_in
// 	// TODO: into_vec? how do we handle that since we have BoxedSliceChain?
// 	// probably have into_vec on both Box<SliceChain<T>> and BoxedSliceChain<T>

// 	#[inline]
// 	pub fn repeat(&self, n: usize) -> VecChain<T>
// 	where
// 		T: Copy
// 	{
// 		self.inner.repeat(n).into()
// 	}

// 	// TODO: concat, join
// }

// impl SliceChain<f64> {
// 	// TODO: nightly sort_floats
// }

// impl SliceChain<u8> {
// 	// TODO: to_ascii_uppercase/lowercase
// }

// // AsciiChar is nightly
// // impl SliceChain<AsciiChar>
