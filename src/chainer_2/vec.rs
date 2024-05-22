use crate::to_maybeuninit::ToMaybeUninit as _;
use std::cmp::Ordering;
use std::mem::{ ManuallyDrop, MaybeUninit };
use std::ops::RangeBounds;
use std::slice::{ self, SliceIndex };
use std::vec;
use super::{ SliceBoxChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

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
	/// # use wiwi::chainer_2::VecChain;
	/// // a chain thingie! yay!...
	/// let chain = VecChain::<String>::new();
	/// ```
	pub fn new() -> Self {
		Vec::new().into()
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
	/// # use wiwi::chainer_2::VecChain;
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

	// TODO: try_with_capacity

	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		Vec::from_raw_parts(ptr, length, capacity).into()
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
	pub fn as_slice_ref_chainer(&self) -> SliceRefChain<T> {
		(*self.inner).into()
	}

	/// Borrow this vector chain mutably as a [`SliceMutChain`].
	///
	/// Note: this does not consume `self`, but only mutably borrow from it. So,
	/// you will need to keep `self` in somewhere owned.
	pub fn as_slice_mut_chainer(&mut self) -> SliceMutChain<T> {
		(&mut *self.inner).into()
	}

	pub fn as_ptr(&self) -> *const T {
		self.inner.as_ptr()
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.inner.as_mut_ptr()
	}

	pub fn as_vec(&self) -> &Vec<T> {
		&self.inner
	}

	pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}

	/// Unwraps and retrieves the underlying [`Vec`] out.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer_2::VecChain;
	/// # let vec_chain = VecChain::<String>::new();
	/// let regular_vec = vec_chain.into_inner();
	/// ```
	pub fn into_inner(self) -> Vec<T> {
		self.inner
	}

	pub fn into_boxed_slice(self) -> Box<[T]> {
		self.inner.into_boxed_slice()
	}

	pub fn into_boxed_slice_chainer(self) -> SliceBoxChain<T> {
		self.into_boxed_slice().into()
	}
}

// TODO: for alloc param
impl<T> VecChain<T> {
	// TODO: new_in
	// TODO: with_capacity_in
	// TODO: try_with_capacity_in
	// TODO: from_raw_parts_in

	pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
		// TODO: use std's version once stable

		let mut me = ManuallyDrop::new(self);

		let pointer = me.inner.as_mut_ptr();
		let length = me.inner.len();
		let capacity = me.inner.capacity();

		(pointer, length, capacity)
	}

	// TODO: into_raw_parts_with_alloc
	// TODO: fn allocator
}

impl<T> VecChain<T> {
	/// Writes the number of elements (also known as the length) in the vector
	/// into `out`.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer_2::VecChain;
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
	/// # use wiwi::chainer_2::VecChain;
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
	/// # use wiwi::chainer_2::VecChain;
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
	/// # use wiwi::chainer_2::VecChain;
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

	// TODO: try_reserve/exact

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	pub fn resize(mut self, new_len: usize, value: T) -> Self
	where
		T: Clone
	{
		self.inner.resize(new_len, value);
		self
	}

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

	pub fn dedup(mut self) -> Self
	where
		T: PartialOrd
	{
		self.inner.dedup();
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

	pub fn extend_from_slice(mut self, other: &[T]) -> Self
	where
		T: Clone
	{
		self.inner.extend_from_slice(other);
		self
	}

	pub fn extend_from_slice_ref_chainer(mut self, other: SliceRefChain<T>) -> Self
	where
		T: Clone
	{
		self.inner.extend_from_slice(other.as_slice());
		self
	}
	pub fn extend_from_slice_mut_chainer(mut self, other: SliceMutChain<T>) -> Self
	where
		T: Clone
	{
		self.inner.extend_from_slice(other.as_slice());
		self
	}

	pub fn extend_from_within<R>(mut self, src: R) -> Self
	where
		T: Clone,
		R: RangeBounds<usize>
	{
		self.inner.extend_from_within(src);
		self
	}

	pub fn pop(mut self, out: &mut Option<T>) -> Self {
		self.pop_uninit(out.to_maybeuninit_mut())
	}

	pub fn pop_uninit(mut self, out: &mut MaybeUninit<Option<T>>) -> Self {
		out.write(self.inner.pop());
		self
	}

	// TODO: pop_if

	pub fn append(mut self, other: &mut Vec<T>) -> Self {
		self.inner.append(other);
		self
	}

	pub fn append_vec_chain(mut self, other: &mut Self) -> Self {
		self.append(&mut other.inner)
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
	/// # use wiwi::chainer_2::VecChain;
	/// let mut static_ref = VecChain::new()
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
	pub fn leak<'h>(self) -> SliceMutChain<'h, T> {
		self.shrink_to_fit().into_inner().leak().into()
	}

	/// Calls the provided closure with the spare capacity of the vector as
	/// a [`SliceMutChain`] of [`MaybeUninit`]s.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer_2::VecChain;
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
		F: FnOnce(SliceMutChain<MaybeUninit<T>>)
	{
		f(self.inner.spare_capacity_mut().into());
		self
	}

	// TODO: resize
	// TODO: splice
	// TODO: extract_if

	// TODO: utf8_chunks
	// TODO: is_ascii
	// TODO: as_ascii
	// TODO: as_ascii_unchecked
	// TODO: eq_ignore_ascii_case
	// TODO: make_ascii_uppercase
	// TODO: make_ascii_lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii_start/end
	// TODO: trim_ascii
	// TODO: sort_floats
	// TODO: first/mut
	// TODO: split_first/mut
	// TODO: split_last/mut
	// TODO: last/mut
	// TODO: first_chunk/mut
	// TODO: split_first_chunk/mut
	// TODO: split_last_chunk/mut
	// TODO: last_chunk/mut
	// TODO: get/mut
	// TODO: get_unchecked/mut
	// TODO: as_ptr_range/as_mut_ptr_range
	// TODO: swap/unchecked
	// TODO: reverse
	// TODO: iter/mut
	// TODO: windows
	// TODO: chunks/mut
	// TODO: chunks_exact/mut
	// TODO: as_chunks_unchecked
	// TODO: as_chunks
	// TODO: as_rchunks
	// TODO: array_chunks
	// TODO: as_chunks_unchecked_mut
	// TODO: as_chunks_mut
	// TODO: as_rchunks_mut
	// TODO: array_chunks_mut
	// TODO: array_windows
	// TODO: rchunks/mut
	// TODO: rchunks_exact/mut
	// TODO: chunk_by/mut
	// TODO: split_at/mut
	// TODO: split_at_unchecked/mut
	// TODO: split_at_checked/mut
	// TODO: split/mut
	// TODO: split_inclusive/mut
	// TODO: rsplit/mut
	// TODO: splitn/mut
	// TODO: rsplitn/mut
	// TODO: split_once/mut
	// TODO: rsplit_once/mut
	// TODO: contains
	// TODO: starts/ends_with
	// TODO: strip_prefix/suffix
	// TODO: binary_search/by/key
	// TODO: sort_unstable/by/key
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: rotate_left/right
	// TODO: fill/fill_with
	// TODO: clone_from_slice
	// TODO: copy_from_slice
	// TODO: copy_within
	// TODO: why std doesn't have clone_within?
	// TODO: swap_with_slice
	// TODO: align_to/mut
	// TODO: as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: why not non-mut of the above 2?
	// TODO: sort_floats
	// TODO: flatten/mut
	// TODO: as_str
	// TODO: as_bytes
	// TODO: to_ascii_uppercase/lowercase
	// TODO: sort/by/key/cached_key
	// TODO: sort_by_cached_key
	// TODO: to_vec/in
	// TODO: repeat
	// TODO: concat
	// TODO: join
}

impl<T, const N: usize> VecChain<[T; N]> {
	// TODO: into_flattened
}

impl<T> From<Vec<T>> for VecChain<T> {
	fn from(value: Vec<T>) -> Self {
		Self { inner: value }
	}
}

// TODO: look through traits
