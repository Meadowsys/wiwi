use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ ManuallyDrop, MaybeUninit };
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
	/// # use wiwi::chainer::VecChain;
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
	/// # use wiwi::chainer::VecChain;
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

	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}

	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
		self.inner.set_len(new_len);
		self
	}

	// TODO: swap_remove
	// TODO: insert
	// TODO: remove
	// TODO: retain
	// TODO: retain_mut
	// TODO: dedup/by/key
	// TODO: push
	// TODO: push_within_capacity
	// TODO: pop
	// TODO: pop_if
	// TODO: append
	// TODO: drain
	// TODO: clear
	// TODO: len/is_empty
	// TODO: split_off
	// TODO: resize_with
	// TODO: leak
	// TODO: spare_capacity_mut
	// TODO: split_at_spare_mut
	// TODO: resize
	// TODO: extend_from_slice
	// TODO: extend_from_within
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
