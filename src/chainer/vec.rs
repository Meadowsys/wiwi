use crate::to_maybeuninit::ToMaybeUninit as _;
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

/// Constructor functions
impl<T> VecChain<T> {
	/// Creates a new vector chain without allocating any capacity.
	///
	/// It will not allocate until it needs to, either by pushing an element,
	/// calling the [`reserve`](Self::reserve) function to explicitly request
	/// allocation, or something else.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// // a chain thingie! yay!...
	/// let chain = VecChain::<String>::new();
	/// ```
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	/// Creates a new vector, and preallocate some memory.
	///
	/// The amount of memory allocated will be _at least_ enough to hold `capacity`
	/// elements without reallocating. No allocation will happen if the provided
	/// capacity is zero.
	///
	/// There is NO GUARANTEE that this function will allocate an exact amount
	/// of memory. If knowing the actual allocated capacity is important, always
	/// do so using the [`capacity`](Self::capacity) function.
	///
	/// If the element type (ie. `T`) is a ZST, the vector chainer will never
	/// allocate, and will always have a capacity of `usize::MAX` bytes.
	///
	/// # Panics
	///
	/// Panics if the new capacity exceeds `isize::MAX` _bytes_ (not elements,
	/// bytes). This is the same behaviour of [`Vec::with_capacity`].
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// # let mut len = 0;
	/// # let mut initial_capacity = 0;
	/// # let mut capacity = 0;
	/// let chain = VecChain::with_capacity(10)
	///    // chaining methods to get the len and capacity of the vec chain
	///    .len(&mut len)
	///    .capacity(&mut initial_capacity);
	///
	/// // The vector chain contains zero elements, and at least room for 10 more
	/// assert_eq!(len, 0);
	/// assert!(initial_capacity >= 10);
	///
	/// // These are all done without reallocating
	/// let chain = (0..10)
	///    .fold(chain, |chain, i| chain.push(i))
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 10);
	/// assert_eq!(capacity, initial_capacity);
	///
	/// // Now however, pushing another element can make the vector reallocate
	/// let chain = chain
	///    .push(11)
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 11);
	/// assert!(capacity >= 11);
	///
	/// # let mut capacity1 = 0;
	/// # let mut capacity2 = 0;
	/// // ZSTs never allocate and always have a capacity of `usize::MAX`
	/// let chain1 = VecChain::<()>::new()
	///    .capacity(&mut capacity1);
	/// let chain2 = VecChain::<()>::with_capacity(10)
	///    .capacity(&mut capacity2);
	///
	/// assert_eq!(capacity1, usize::MAX);
	/// assert_eq!(capacity2, usize::MAX);
	/// ```
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
	/// Unwraps and retrieves the underlying [`Vec`] out.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// # let vec_chain = VecChain::<String>::new();
	/// let regular_vec = vec_chain.into_inner();
	/// ```
	pub fn into_inner(self) -> Vec<T> {
		self.inner
	}

	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.inner
	}

	/// Borrow this vector chain immutably as a [`SliceRefChain`].
	///
	/// Note: this does not consume `self`, but only immutably borrow from it. So,
	/// you will need to keep `self` in somewhere owned.
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

	/// Writes the number of elements (also known as the length) in the vector
	/// into `out`.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// let mut len = 0;
	///
	/// let chain = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3, 4, 5])
	///    .len(&mut len);
	///
	/// assert_eq!(len, 5);
	/// ```
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	/// Writes the number of elements (also known as the length) in the vector
	/// into `out`.
	///
	/// This function will always write to the output, so it is safe to call
	/// [`assume_init`](MaybeUninit::assume_init) after invoking this function.
	///
	/// # Examples
	///
	/// ```
	/// # use std::mem::MaybeUninit;
	/// # use wiwi::chainer::VecChain;
	/// let mut len = MaybeUninit::uninit();
	///
	/// let chain = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3, 4, 5])
	///    .len_uninit(&mut len); // writes to `len`
	///
	/// // this is safe!
	/// let len = unsafe { len.assume_init() };
	/// assert_eq!(len, 5);
	/// ```
	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	/// Writes `true` into the output if the vector contains no elements, and
	/// false otherwise
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// // output variables...
	/// let mut before = false;
	/// let mut after = false;
	///
	/// let chain = VecChain::new()
	///    // chains are evaluated eagerly
	///    .is_empty(&mut before)
	///    .push(1)
	///    .is_empty(&mut after);
	///
	/// assert!(before);
	/// assert!(!after);
	/// ```
	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	/// Writes `true` into the output if the vector contains no elements, and
	/// false otherwise
	///
	/// # Examples
	///
	/// ```
	/// # use std::mem::MaybeUninit;
	/// # use wiwi::chainer::VecChain;
	/// // output variables...
	/// let mut before = MaybeUninit::uninit();
	/// let mut after = MaybeUninit::uninit();
	///
	/// let chain = VecChain::new()
	///    // chains are evaluated eagerly
	///    .is_empty_uninit(&mut before)
	///    .push(1)
	///    .is_empty_uninit(&mut after);
	///
	/// // this is safe!
	/// let before = unsafe { before.assume_init() };
	/// let after = unsafe { after.assume_init() };
	///
	/// assert!(before);
	/// assert!(!after);
	/// ```
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

	pub fn dedup_by<F>(mut self, mut same_bucket: F) -> Self
	where
		F: FnMut(&T, &T) -> bool
	{
		// let rust coerce &mut T to &T
		self.inner.dedup_by(move |a, b| same_bucket(a, b));
		self
	}

	pub fn dedup_by_mut<F>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut T, &mut T) -> bool
	{
		self.inner.dedup_by(same_bucket);
		self
	}

	pub fn dedup_by_key<F, K>(mut self, mut key: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: PartialEq
	{
		// let rust coerce &mut T to &T
		self.inner.dedup_by_key(|v| key(v));
		self
	}

	pub fn dedup_by_key_mut<F, K>(mut self, key: F) -> Self
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
		self.append_vec(&mut other.inner)
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

	/// Consumes `self` and leaks it, returning a mutable reference to the content.
	/// You may choose any lifetime `'h`, as long as `T` outlives `'h`. It can even
	/// be `'static`.
	///
	/// The vector is shrunk as much as it can be (ie. [`shrink_to_fit`] is called),
	/// but it might still have some excess capacity, in the same way [`with_capacity`]
	/// and [`reserve`] can allocate more than is requested.
	///
	/// This function is mainly useful for something that needs to be kept for
	/// the remainder of a program's life. As this function's name implies, if
	/// you drop the returned reference, it will leak memory. Absence of memory
	/// leaks is not part of Rust's memory model guarantees (for some reason...),
	/// so this is safe.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// let static_ref = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3])
	///    .leak::<'static>();
	///
	/// static_ref.as_mut_slice()[1] = 20;
	/// assert_eq!(static_ref.as_slice(), [1, 20, 3]);
	/// ```
	///
	/// [`shrink_to_fit`]: Self::shrink_to_fit
	/// [`with_capacity`]: Self::with_capacity
	/// [`reserve`]: Self::reserve
	pub fn leak<'h>(self) -> &'h mut SliceMutChain<T> {
		self.shrink_to_fit().into_inner().leak().into()
	}

	/// Calls the provided closure with the spare capacity of the vector as
	/// a [`SliceMutChain`] of [`MaybeUninit`]s.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// # let mut spare_len = 0;
	/// let chain = VecChain::with_capacity(10)
	///    .extend_from_slice(&[1, 2, 3, 4, 5])
	///    .with_spare_capacity_mut(|mut spare| {
	///       spare = spare.len(&mut spare_len);
	///
	///       // VecChain allocated at least 10 elements worth of space
	///       // since we pushed 5 elements, it should have at least
	///       // 5 elements of spare capacity left
	///       assert!(spare_len >= 5);
	///    })
	///    .push(6)
	///    .push(7)
	///    .with_spare_capacity_mut(|mut spare| {
	///       spare = spare.len(&mut spare_len);
	///
	///       // Just pushed 2 more elements...
	///       assert!(spare_len >= 3);
	///    });
	/// ```
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
		// TODO: call Vec impl when it is stabilised
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
	// TODO: contains
	// TODO: starts_with
	// TODO: ends_with
	// TODO: strip_prefix/suffix
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
	// TODO: sort/by/key

	pub fn sort(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort();
		self
	}

	// TODO: sort_by_cached_key
	// TODO: to_vec/in????? no lol just clone?
	// TODO: repeat
	// TODO: concat
	// TODO: join
	// TODO: to_ascii_uppercase/lowercase
}

// TODO: allocator param
impl<T: Clone> VecChain<T> {
	pub fn resize(mut self, new_len: usize, value: T) -> Self {
		self.inner.resize(new_len, value);
		self
	}

	pub fn extend_from_slice(mut self, other: &[T]) -> Self {
		self.inner.extend_from_slice(other);
		self
	}

	pub fn extend_from_ref_slice_chainer(mut self, other: &SliceRefChain<T>) -> Self {
		self.inner.extend_from_slice(other.as_slice());
		self
	}
	pub fn extend_from_mut_slice_chainer(mut self, other: &mut SliceMutChain<T>) -> Self {
		self.inner.extend_from_slice(other.as_slice());
		self
	}

	pub fn extend_from_within<R>(mut self, src: R) -> Self
	where
		R: RangeBounds<usize>
	{
		self.inner.extend_from_within(src);
		self
	}
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

		// TODO: switch to into_raw_parts impl when it is stabilised
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

impl<T> VecChain<T> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn creation() {
		let mut new_len = 0;
		let mut new_capacity = 0;
		let mut with_capacity_len = 0;
		let mut with_capacity_capacity = 0;
		let mut zst_len = 0;
		let mut zst_capacity = 0;

		let _ = VecChain::<u8>::new()
			.len(&mut new_len)
			.capacity(&mut new_capacity);

		let _ = VecChain::<u8>::with_capacity(30)
			.len(&mut with_capacity_len)
			.capacity(&mut with_capacity_capacity);

		let _ = VecChain::<()>::with_capacity(30)
			.len(&mut zst_len)
			.capacity(&mut zst_capacity);

		assert_eq!(new_len, 0);
		assert_eq!(new_capacity, 0);
		assert_eq!(with_capacity_len, 0);
		assert_eq!(with_capacity_capacity, 30);
		assert_eq!(zst_len, 0);
		assert_eq!(zst_capacity, usize::MAX);
	}

	#[test]
	fn conversion() {
		let slice = &[1u8, 2, 3, 4, 5];
		let mut chain = VecChain::new()
			.extend_from_slice(slice);

		assert_eq!(slice, chain.as_slice());
		assert_eq!(slice, chain.as_mut_slice());
	}

	#[test]
	fn len_and_capacity() {
		let mut len1 = 0;
		let mut cap1 = 0;
		let mut len2 = 0;
		let mut cap2 = 0;
		let mut is_empty = true;
		let mut is_empty_new = false;
		let mut is_empty_with_cap = false;

		let _ = mk_test_chain()
			.len(&mut len1)
			.capacity(&mut cap1)
			.push(9)
			.push(10)
			.len(&mut len2)
			.capacity(&mut cap2)
			.is_empty(&mut is_empty);
		let _ = VecChain::<u8>::new()
			.is_empty(&mut is_empty_new);
		let _ = VecChain::<u8>::with_capacity(8)
			.is_empty(&mut is_empty_with_cap);


		assert_eq!(len1, 8);
		assert_eq!(cap1, 8);
		assert_eq!(len2, 10);

		assert!(cap2 > 8);
		assert!(!is_empty);
		assert!(is_empty_new);
		assert!(is_empty_with_cap);
	}

	#[test]
	fn with_split_at_spare_mut() {
		let mut uninit_len = 0;
		let chain = VecChain::new()
			.extend_from_slice(&[1usize, 2, 3, 4, 5, 6, 7, 8])
			.reserve(8)
			.with_split_at_spare_mut(|slice, uninit| {
				let slice = slice.as_mut_slice();
				let uninit = uninit.as_mut_slice();
				uninit_len = uninit.len();

				assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8]);
				assert!(uninit.len() >= 8);

				uninit.iter_mut()
					.enumerate()
					.take(4)
					.for_each(|(i, slot)| {
						slot.write(i);
					});
			});

		unsafe {
			let mut len = 0;
			let _ = chain
				.len(&mut len)
				.set_len(len + 4)
				.with_split_at_spare_mut(|slice, uninit| {
					let slice = slice.as_mut_slice();
					let uninit = uninit.as_mut_slice();

					assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3]);
					assert_eq!(uninit_len - 4, uninit.len());
				});
		}
	}

	#[test]
	fn swap_unchecked() {
		unsafe {
			let chain = VecChain::new()
				.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
				.swap_unchecked(4, 6)
				.swap_unchecked(0, 3)
				.swap_unchecked(1, 6)
				.swap_unchecked(6, 7)
				.swap_unchecked(2, 6);
			assert_eq!(chain.as_slice(), &[4, 5, 8, 1, 7, 6, 3, 2]);
		}
	}

	#[test]
	fn reverse() {
		let chain = VecChain::new()
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.reverse();
		assert_eq!(chain.as_slice(), &[8, 7, 6, 5, 4, 3, 2, 1]);
	}

	#[test]
	fn with_chunks() {
		const N: usize = 5;

		let slice = b"1234";

		fn check<'h>(
			expected_chunks: &'h [[u8; N]],
			expected_remainder: &'h [u8]
		) -> impl FnOnce(&[[u8; N]], &[u8]) + 'h {
			move |chunks, rem| {
				assert_eq!(expected_chunks.len(), chunks.len(), "wrong num of chunks");
				assert_eq!(expected_remainder.len(), rem.len(), "wrong num of elements in remainder");

				assert_eq!(expected_chunks, chunks);
				assert_eq!(expected_remainder, rem);
			}
		}

		let _ = VecChain::with_capacity(20)
			.extend_from_slice(slice)
			.with_chunks(check(&[], b"1234"))

			.extend_from_slice(slice)
			.with_chunks(check(&[*b"12341"], b"234"))

			.extend_from_slice(slice)
			.with_chunks(check(&[*b"12341", *b"23412"], b"34"))

			.extend_from_slice(slice)
			.with_chunks(check(&[*b"12341", *b"23412", *b"34123"], b"4"))

			.extend_from_slice(slice)
			.with_chunks(check(&[*b"12341", *b"23412", *b"34123", *b"41234"], b""));
	}

	fn mk_test_chain() -> VecChain<u8> {
		VecChain::with_capacity(8)
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
	}
}
