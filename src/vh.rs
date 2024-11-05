use crate::prelude_std::*;

#[repr(transparent)]
pub struct RcStr {
	inner: RcSlice<u8>
}

#[repr(transparent)]
pub struct RcStrWeak {
	inner: RcSliceWeak<u8>
}

impl RcStr {
	#[inline]
	pub fn new(s: &str) -> Self {
		Self { inner: RcSlice::new_copy(s.as_bytes()) }
	}
}

impl RcStr {
	#[inline]
	pub fn downgrade(this: &Self) -> RcStrWeak {
		RcStrWeak { inner: RcSlice::downgrade(&this.inner) }
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		// SAFETY: `self.inner` contains valid utf-8 is invariant of `RcStr`
		unsafe { str::from_utf8_unchecked(&self.inner) }
	}
}

impl Deref for RcStr {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl RcStrWeak {
	#[inline]
	pub fn upgrade(&self) -> Option<RcStr> {
		self.inner.upgrade().map(|inner| RcStr { inner })
	}
}

#[repr(transparent)]
pub struct RcSlice<T> {
	inner: RcSliceInner<T>
}

#[repr(transparent)]
pub struct RcSliceWeak<T> {
	inner: RcSliceInner<T>
}

impl<T> RcSlice<T> {
	#[inline]
	pub fn new_clone(slice: &[T]) -> Self
	where
		T: Clone
	{
		// SAFETY: we are `RcSlice` constructor
		unsafe { Self::finish_init(RcSliceInner::new_clone(slice)) }
	}

	#[inline]
	pub fn new_copy(slice: &[T]) -> Self
	where
		T: Copy
	{
		// SAFETY: we are `RcSlice` constructor
		unsafe { Self::finish_init(RcSliceInner::new_copy(slice)) }
	}

	#[inline]
	pub fn new_boxed_slice(slice: Box<[T]>) -> Self
	where
		T: Copy
	{
		// SAFETY: we are `RcSlice` constructor
		unsafe { Self::finish_init(RcSliceInner::new_boxed_slice(slice)) }
	}

	pub fn new_array<const N: usize>(array: [T; N]) -> Self {
		// SAFETY: we are `RcSlice` constructor
		unsafe { Self::finish_init(RcSliceInner::new_array(array)) }
	}

	/// Increments one strong and one weak, then wraps in `Self`
	///
	/// # Safety
	///
	/// To be used only in `RcSlice` constructors.
	#[inline]
	unsafe fn finish_init(inner: RcSliceInner<T>) -> Self {
		// SAFETY: todo put something more detailed here when
		// safety of `inc_strong` is documented or something
		unsafe { inner.inc_strong() }

		// SAFETY: todo put something more detailed here when
		// safety of `inc_weak` is documented or something
		unsafe { inner.inc_weak() }

		Self { inner }
	}
}

impl<T> RcSlice<T> {
	#[inline]
	pub fn downgrade(this: &Self) -> RcSliceWeak<T> {
		// SAFETY: todo put something more detailed here when
		// safety of `inc_weak` is documented or something
		unsafe { this.inner.inc_weak() }

		let inner = RcSliceInner { ptr: this.inner.ptr, _marker: PhantomData };
		RcSliceWeak { inner }
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<T> Deref for RcSlice<T> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &[T] {
		self.inner.slice()
	}
}

impl<T> Drop for RcSlice<T> {
	fn drop(&mut self) {
		// SAFETY: todo put something more detailed here when
		// safety of `dec_strong` is documented or something
		unsafe { self.inner.dec_strong() }

		if self.inner.strong() == 0 {
			// SAFETY: we held the last (strong) reference
			unsafe { self.inner.drop_contents() }

			// SAFETY: todo put something more detailed here when
			// safety of `dec_weak` is documented or something
			unsafe { self.inner.dec_weak() }

			if self.inner.weak() == 0 {
				// SAFETY: there were no weak references
				// (we decremented out the "fake" strong one)
				unsafe { self.inner.dealloc() }
			}
		}
	}
}

impl<T> RcSliceWeak<T> {
	#[inline]
	pub fn upgrade(&self) -> Option<RcSlice<T>> {
		(self.inner.strong() > 0).then(|| {
			// SAFETY: todo put something more detailed here when
			// safety of `inc_strong` is documented or something
			unsafe { self.inner.inc_strong() }

			let inner = RcSliceInner { ptr: self.inner.ptr, _marker: PhantomData };
			RcSlice { inner }
		})
	}
}

impl<T> Drop for RcSliceWeak<T> {
	fn drop(&mut self) {
		// SAFETY: todo put something more detailed here when
		// safety of `dec_weak` is documented or something
		unsafe { self.inner.dec_weak() }

		if self.inner.weak() == 0 {
			// SAFETY: there will be a "fake" weak reference if there are strong
			// references still around. The last strong reference will be responsible
			// for dropping the contents of the slice.
			unsafe { self.inner.dealloc() }
		}
	}
}

// Layout of data pointer if `align_of::<T>() <= align_of::<usize>()`:
// - offset of 0 is strong count with type `usize`
// - offset of `align_of::<usize>()` is weak count with type `usize`
// - offset of `2 * align_of::<usize>()` is len with type `usize`
// - offset of `3 * align_of::<usize>()` is start of elements
//
// Layout of data pointer if `align_of::<T>() == 2 * align_of::<usize>()`
// - strong count, weak count, and length fields are accessed the same as the first
// - offset of `2 * align_of::<T>()` is start of elements
// - there will be an empty `usize`-sized padding before start of elements
//
// Layout of data pointer if `align_of::<T>() >= 4 * align_of::<usize>()`
// - strong count, weak count, and length fields are accessed same as the first
// - offset of `align_of::<T>()` is start of elements
// - there will be at least one `usize`-sized padding before the elements
#[repr(transparent)]
struct RcSliceInner<T> {
	// there is significance of `usize` in this pointer type
	ptr: ptr::NonNull<usize>,
	_marker: PhantomData<[T]>
}

impl<T> RcSliceInner<T> {
	#[inline]
	fn new_clone(slice: &[T]) -> Self
	where
		T: Clone
	{
		let ptr = Self::alloc(slice.len());
		// on panic, this just gets leaked, since we do not implement `Drop`
		// it's up to the strong/weak containers to implement drop and manually
		// drop us when needed
		// TODO: setup drop guards?
		let new = Self { ptr, _marker: PhantomData };
		let ptr = new.slice_ptr();

		slice.iter().enumerate().for_each(|(i, elem)| {
			let elem = elem.clone();

			// SAFETY:
			// - `ptr` obtained from `new` is valid for `slice.len()` writes
			// - `i` will iterate over `0..slice.len()`, so won't offset
			//   out of past the allocation
			let ptr = unsafe { ptr.add(i) };

			// SAFETY: ptr is valid for writes, see above
			unsafe { ptr.write(elem) }
		});

		// SAFETY: we just wrote that much items
		unsafe { new.len_ptr().write(slice.len()) }

		new
	}

	#[inline]
	fn new_copy(slice: &[T]) -> Self
	where
		T: Copy
	{
		// SAFETY: `T: Copy`, enforced by method trait bound
		unsafe { Self::new_copy_unchecked(slice) }
	}

	#[inline]
	fn new_boxed_slice(slice: Box<[T]>) -> Self {
		let slice = ManuallyDrop::new(slice);

		// SAFETY: `slice` was put in `ManuallyDrop` to avoid double drop
		let new = unsafe { Self::new_copy_unchecked(&slice) };

		// doing this makes it so we can deallocate the box memory,
		// but not drop the contents of the box, to avoid double drop
		let ptr = Box::into_raw(ManuallyDrop::into_inner(slice));

		#[expect(clippy::as_conversions)]
		let ptr = ptr as *mut [ManuallyDrop<T>];

		// SAFETY:
		// - `[ManuallyDrop<T>]` has same layout as `[T]`
		// - we just got the pointer from `Box::from_raw`
		drop(unsafe { Box::from_raw(ptr) });

		new
	}

	#[inline]
	fn new_array<const N: usize>(array: [T; N]) -> Self {
		let array = ManuallyDrop::new(array);
		// SAFETY: `array` was put in `ManuallyDrop` to avoid double drop
		unsafe { Self::new_copy_unchecked(&*array) }
	}

	/// # Safety
	///
	/// Either `T` needs to be `Copy`, or you need to ensure that the source
	/// slice is not dropped to avoid double dropping the elements of `T`.
	#[inline]
	unsafe fn new_copy_unchecked(slice: &[T]) -> Self {
		let ptr = Self::alloc(slice.len());
		// TODO: setup drop guards?
		let new = Self { ptr, _marker: PhantomData };

		// SAFETY:
		// - ptr obtained from `slice` valid for `slice.len()` reads
		// - ptr obtained from `Self::alloc(slice.len())` valid for `slice.len()` writes
		// - ptr obtained from `slice` is aligned
		// - ptr obtained from `Self::alloc` is aligned
		// - just allocated memory cannot overlap with passed in reference
		// - `T: Copy`, so this is valid semantically as well
		unsafe { ptr::copy_nonoverlapping(slice.as_ptr(), new.slice_ptr(), slice.len()) }

		// SAFETY: we just wrote that much items
		unsafe { new.len_ptr().write(slice.len()) }

		new
	}
}

impl<T> RcSliceInner<T> {
	#[inline]
	fn alloc(len: usize) -> ptr::NonNull<usize> {
		let layout = Self::calculate_layout(len);

		// SAFETY: layout is not zero (it always has at least
		// 3 words size, for strong/weak/len slots)
		let ptr = unsafe { alloc(layout) };

		let Some(ptr) = ptr::NonNull::new(ptr.cast::<usize>()) else {
			alloc_mod::handle_alloc_error(layout)
		};

		ptr
	}

	/// # Safety
	///
	/// You cannot call this more than once.
	#[inline]
	unsafe fn drop_contents(&self) {
		#[expect(clippy::as_conversions)]
		let ptr = self.slice() as *const [T] as *mut [T];

		// SAFETY: caller guarantees to not call this more than once
		unsafe { ptr::drop_in_place(ptr) }
	}

	/// Deallocates backing memory (without dropping the contents in the slice)
	///
	/// # Safety
	///
	/// You must call this only once for this instance of inner.
	#[inline]
	unsafe fn dealloc(&self) {
		let layout = Self::calculate_layout(self.len());

		// SAFETY: caller promises to call this only once. Additionally,
		// ptr/layout are taken/calculated from self, so are correct
		unsafe { dealloc(self.ptr.as_ptr().cast::<u8>(), layout) }
	}

	#[inline]
	fn calculate_layout(len: usize) -> alloc_mod::Layout {
		const _: () = assert!(
			size_of::<usize>() == align_of::<usize>(),
			"do file an issue if this panicks lol"
		);

		let align_usize = align_of::<usize>();
		let align_t = align_of::<T>();

		// this if/else chain should get evaluated at compile time,
		// since `align_t` and `align_usize` are constants
		// in a monomorphised version of this function
		let layout = if align_t <= align_usize {
			let meta = 3 * size_of::<usize>();
			let slice = len * size_of::<T>();

			// SAFETY:
			// - align is a value taken from `align_of` so must be valid
			// - size, in what we are using this for at the time of writing,
			//   will not overflow `isize::MAX`. TODO: do something about that
			//   if/when we move this out to its own thing, this is not good
			unsafe { alloc_mod::Layout::from_size_align_unchecked(meta + slice, align_usize) }
		} else if align_t == 2 * align_usize {
			// enough space for 4 usizes (we need 3 so this is good)
			let meta_and_padding = 2 * align_of::<T>();
			let slice = len * size_of::<T>();

			// SAFETY:
			// - align is a value taken from `align_of` so must be valid
			// - size, in what we are using this for at the time of writing,
			//   will not overflow `isize::MAX`. TODO: do something about that
			//   if/when we move this out to its own thing, this is not good
			unsafe { alloc_mod::Layout::from_size_align_unchecked(meta_and_padding + slice, align_t) }
		} else {
			// align_t >= 4 * align_usize

			// enough for at least 4 usizes, more if T alignment is larger
			// (we need 3 so this is good)
			let meta_and_padding = align_of::<T>();
			let slice = len * size_of::<T>();

			// SAFETY:
			// - align is a value taken from `align_of` so must be valid
			// - size, in what we are using this for at the time of writing,
			//   will not overflow `isize::MAX`. TODO: do something about that
			//   if/when we move this out to its own thing, this is not good
			unsafe { alloc_mod::Layout::from_size_align_unchecked(meta_and_padding + slice, align_t) }
		};

		layout.pad_to_align()
	}

	#[inline]
	fn strong_ptr(&self) -> *mut usize {
		self.ptr.as_ptr()
	}

	#[inline]
	fn weak_ptr(&self) -> *mut usize {
		// SAFETY: this offset is within allocation bounds (see `calculate_layout`)
		unsafe { self.ptr.as_ptr().add(1) }
	}

	#[inline]
	fn len_ptr(&self) -> *mut usize {
		// SAFETY: this offset is within allocation bounds (see `calculate_layout`)
		unsafe { self.ptr.as_ptr().add(2) }
	}

	#[inline]
	fn slice_ptr(&self) -> *mut T {
		// see `calculate_layout` for details on these calculations

		let align_usize = align_of::<usize>();
		let align_t = align_of::<T>();
		let ptr = self.ptr.as_ptr();

		// this if/else chain should get evaluated at compile time,
		// since `align_t` and `align_usize` are constants
		// in a monomorphised version of this function
		if align_t <= align_usize {
			// SAFETY: this offset is within allocation bounds (see `calculate_layout`)
			unsafe { ptr.add(3).cast::<T>() }
		} else if align_t == 2 * align_usize {
			// SAFETY: this offset is within allocation bounds (see `calculate_layout`)
			unsafe { ptr.cast::<T>().add(2) }
		} else {
			// align_t >= 4 * align_usize
			// SAFETY: this offset is within allocation bounds (see `calculate_layout`)
			unsafe { ptr.cast::<T>().add(1) }
		}
	}

	#[inline]
	fn strong(&self) -> usize {
		// SAFETY: ptr obtained from `self`
		unsafe { *self.strong_ptr() }
	}

	#[inline]
	fn weak(&self) -> usize {
		// SAFETY: ptr obtained from `self`
		unsafe { *self.weak_ptr() }
	}

	#[inline]
	fn len(&self) -> usize {
		// SAFETY: ptr obtained from `self`
		unsafe { *self.len_ptr() }
	}

	#[inline]
	fn slice(&self) -> &[T] {
		// SAFETY: ptr/len obtained from `self`
		unsafe { slice::from_raw_parts(self.slice_ptr(), self.len()) }
	}

	#[inline]
	unsafe fn inc_strong(&self) {
		// SAFETY: ptr obtained from `self.strong_ptr()` valid for writes
		unsafe { self.strong_ptr().write(self.strong() + 1) }
	}

	#[inline]
	unsafe fn dec_strong(&self) {
		// SAFETY: ptr obtained from `self.strong_ptr()` valid for writes
		unsafe { self.strong_ptr().write(self.strong() - 1) }
	}

	#[inline]
	unsafe fn inc_weak(&self) {
		// SAFETY: ptr obtained from `self.weak_ptr()` valid for writes
		unsafe { self.weak_ptr().write(self.weak() + 1) }
	}

	#[inline]
	unsafe fn dec_weak(&self) {
		// SAFETY: ptr obtained from `self.weak_ptr()` valid for writes
		unsafe { self.weak_ptr().write(self.weak() - 1) }
	}
}
