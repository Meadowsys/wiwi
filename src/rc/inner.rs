use crate::prelude_std::*;
use self::atomic::Ordering::*;

#[repr(transparent)]
pub(super) struct RcInner<C: Counter, V, S> {
	ptr: ptr::NonNull<RcInnerLayout<C, V, S>>
}

#[repr(C)]
struct RcInnerLayout<C: Counter, V, S> {
	counter: C,
	slice_len: usize,
	value: V,

	// force alignment of (at least) S's alignment, while also not
	// requiring there be at least 1 S element
	slice: [S; 0]
}

#[inline]
fn new_from_value<C: Counter, V>(value: V) -> RcInner<C, V, ()> {
	let instance = alloc_instance(0);

	// SAFETY:
	// - instance just allocated in statement above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { value_uninit(instance).write(value); }

	// no need to initialise slice, as it has length 0

	instance
}

/// # Safety
///
/// The provided slice should either contain elements that implement [`Copy`],
/// or the input slice should be prevented from dropping to avoid double
/// dropping elements.
#[inline]
unsafe fn new_from_slice_copy_unchecked<C: Counter, S>(slice: &[S]) -> RcInner<C, (), S> {
	let instance = alloc_instance(slice.len());

	// no need to initialise value, as it is ZST (unit type)

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: `ptr` is writeable for `slice.len()` elements
	unsafe {
		ptr::copy_nonoverlapping(
			slice.as_ptr(),
			ptr,
			slice.len()
		)
	}

	instance
}

/// # Safety
///
/// The provided slice should either contain elements that implement [`Copy`],
/// or the input slice should be prevented from dropping to avoid double
/// dropping elements.
#[inline]
unsafe fn new_from_value_and_slice_copy_unchecked<C: Counter, V, S>(value: V, slice: &[S]) -> RcInner<C, V, S> {
	let instance = alloc_instance(slice.len());

	// SAFETY:
	// - instance just allocated in statement above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { value_uninit(instance).write(value); }

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: `ptr` is writeable for `slice.len()` elements
	unsafe {
		ptr::copy_nonoverlapping(
			slice.as_ptr(),
			ptr,
			slice.len()
		)
	}

	instance
}

#[inline]
fn new_from_slice_clone<C: Counter, S: Clone>(slice: &[S]) -> RcInner<C, (), S> {
	let instance = alloc_instance::<_, _, S>(slice.len());

	// no need to initialise value, as it is ZST (unit type)

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	slice.iter().enumerate().for_each(|(i, value)| {
		// SAFETY: `ptr` is writeable for `slice.len()` elements
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: see above
		unsafe { ptr.write(value.clone()) }
	});

	instance
}

#[inline]
fn new_from_value_and_slice_clone<C: Counter, V, S: Clone>(value: V, slice: &[S]) -> RcInner<C, V, S> {
	let instance = alloc_instance::<_, _, S>(slice.len());

	// SAFETY:
	// - instance just allocated in statement above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { value_uninit(instance).write(value); }

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	slice.iter().enumerate().for_each(|(i, value)| {
		// SAFETY: `ptr` is writeable for `slice.len()` elements
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: see above
		unsafe { ptr.write(value.clone()) }
	});

	instance
}

#[inline]
fn new_from_array_into_slice<C: Counter, S, const N: usize>(array: [S; N]) -> RcInner<C, (), S> {
	let array = ManuallyDrop::new(array);

	// SAFETY: we put the array into `ManuallyDrop`
	unsafe { new_from_slice_copy_unchecked(&*array) }
}

#[inline]
fn new_from_value_and_array_into_slice<C: Counter, V, S, const N: usize>(value: V, array: [S; N]) -> RcInner<C, V, S> {
	let array = ManuallyDrop::new(array);

	// SAFETY: we put the array into `ManuallyDrop`
	unsafe { new_from_value_and_slice_copy_unchecked(value, &*array) }
}

/// Initialise a new instance with the provided length
///
/// The instance returned will have fields `counter` and `slice_length` fields
/// initialised. Counter is set to 1 strong and 1 weak according to contract of
/// [`Counter`]. Caller is responsible for initialising the `value` and `slice`
/// fields.
#[inline]
fn alloc_instance<C: Counter, V, S>(slice_len: usize) -> RcInner<C, V, S> {
	let layout = calc_layout::<C, V, S>(slice_len);

	// SAFETY: `calc_layout` never returns layout with 0 size
	let ptr = unsafe { alloc(layout) };

	let Some(ptr) = ptr::NonNull::new(ptr.cast()) else {
		alloc_mod::handle_alloc_error(layout)
	};

	let instance = RcInner { ptr };

	// we can fill in counter since we know the type of counter already
	// SAFETY:
	// - instance just allocated in statements above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { counter_uninit(instance).write(C::new()); }

	// we can fill in length since that will never change
	// SAFETY: same as above
	unsafe { slice_len_uninit(instance).write(slice_len); }

	instance
}

/// Drop the value and slice contents of the provided instance
///
/// # Safety
///
/// This instance must be fully initialised, and this must be the first time
/// this function is called on this particular `instance`.
#[inline]
unsafe fn drop_instance<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	// SAFETY: caller promises `instance` is fully initialised
	let slice_ref = unsafe { slice_ref(instance) };

	#[expect(clippy::as_conversions)]
	let slice_ptr = slice_ref as *const [S] as *mut [S];

	// SAFETY: see above
	unsafe { ptr::drop_in_place(slice_ptr) }

	// SAFETY: caller promises `instance` is fully initialised
	let value_ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: see above
	unsafe { ptr::drop_in_place(value_ptr) }
}

/// Drop the counter and deallocate the backing allocation of the provided instance
///
/// # Safety
///
/// This instance must be in the partially initialised state following a call to
/// [`drop_instance`], and this must be the first time this function is called on
/// this particular `instance`. This may be called on an instance that is still
/// fully initialised (ie. [`drop_instance`] has not been called on it), but
/// that is equivalent to leaking the value and slice fields, and is almost
/// certainly incorrect.
#[inline]
unsafe fn dealloc_instance<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	// SAFETY: caller promises `counter` is initialised
	let counter_ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: see above
	unsafe { ptr::drop_in_place(counter_ptr) }

	// SAFETY: caller promises `slice_len` is initialised
	let slice_len = unsafe { slice_len(instance) };

	let layout = calc_layout::<C, V, S>(slice_len);

	// SAFETY: see above
	unsafe { dealloc(instance.ptr.as_ptr().cast(), layout) }
}

/// Calculate the layout to allocate a new instance with the specified counter,
/// value type, slice type, and slice length
#[inline]
fn calc_layout<C: Counter, V, S>(slice_len: usize) -> alloc_mod::Layout {
	alloc_mod::Layout::new::<RcInner<C, V, S>>()
		.extend(alloc_mod::Layout::array::<S>(slice_len).unwrap())
		.unwrap()
		.0
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn strong_count<C: Counter, V, S>(instance: RcInner<C, V, S>) -> usize {
	// SAFETY: caller promises `instance` is not deallocated
	C::strong_count(unsafe { counter_ptr(instance) })
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn weak_count<C: Counter, V, S>(instance: RcInner<C, V, S>) -> usize {
	// SAFETY: caller promises `instance` is not deallocated
	C::weak_count(unsafe { counter_ptr(instance) }) - 1
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn inc_strong_for_clone<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	// SAFETY: caller promises `instance` is not deallocated
	C::inc_strong_for_clone(unsafe { counter_ptr(instance) })
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn dec_strong_for_drop<C: Counter, V, S>(instance: RcInner<C, V, S>) -> bool {
	// SAFETY: caller promises `instance` is not deallocated
	C::dec_strong_for_drop(unsafe { counter_ptr(instance) })
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn inc_weak_for_clone<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	// SAFETY: caller promises `instance` is not deallocated
	C::inc_weak_for_clone(unsafe { counter_ptr(instance) })
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
fn dec_weak_for_drop<C: Counter, V, S>(instance: RcInner<C, V, S>) -> bool {
	// SAFETY: caller promises `instance` is not deallocated
	C::dec_weak_for_drop(unsafe { counter_ptr(instance) })
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
unsafe fn counter_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<C> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).counter };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into counter (exclusive borrow)
#[inline]
unsafe fn counter_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<C> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - The provided `instance` must have field `counter` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn counter_ref<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h C {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { &*ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
unsafe fn slice_len_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<usize> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).slice_len };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into counter (exclusive borrow)
#[inline]
unsafe fn slice_len_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<usize> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_len_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - The provided `instance` must have field `slice_len` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn slice_len<C: Counter, V, S>(instance: RcInner<C, V, S>) -> usize {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_len_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { *ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn value_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<V> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).value };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into counter (exclusive borrow)
#[inline]
unsafe fn value_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<V> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - The provided `instance` must have field `value` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn value_ref<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h V {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { &*ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn slice_thin_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<S> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).slice };
	let ptr = ptr.cast::<S>();

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - The provided `instance` must have field `slice_len` already initialised
/// - The provided `instance` must have `slice_len` elements in `slice` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn slice_ref<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h [S] {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: caller promises to uphold the requirements
	let slice_len = unsafe { slice_len(instance) };

	// SAFETY: caller promises ptr is valid for at least `len` elements
	unsafe { slice::from_raw_parts(ptr, slice_len) }
}

impl<C: Counter, V, S> Clone for RcInner<C, V, S> {
	#[inline]
	fn clone(&self) -> Self {
		*self
	}
}

impl<C: Counter, V, S> Copy for RcInner<C, V, S> {}

pub trait Counter: Sized {
	/// Create a new couter with strong and weak count both set to 1
	fn new() -> Self;

	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn strong_count(this: ptr::NonNull<Self>) -> usize;

	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn weak_count(this: ptr::NonNull<Self>) -> usize;

	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn inc_strong_for_clone(this: ptr::NonNull<Self>);

	/// Decrements the strong count during a drop, returning whether or not
	/// [`drop_instance`] needs to be called on the instance
	///
	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn dec_strong_for_drop(this: ptr::NonNull<Self>) -> bool;

	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn inc_weak_for_clone(this: ptr::NonNull<Self>);

	/// Decrements the weak count during a drop, returning whether or not
	/// [`dealloc_instance`] needs to be called on the instance
	///
	/// # Safety
	///
	/// The pointer provided in `this` must be initialised and valid for
	/// reads and writes (ie. the result of calling [`new`](Counter::new) is valid).
	fn dec_weak_for_drop(this: ptr::NonNull<Self>) -> bool;
}

pub struct ThreadCounter {
	strong: usize,
	weak: usize,
	_not_thread_safe: PhantomData<*const ()>
}

impl Counter for ThreadCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: 1,
			weak: 1,
			_not_thread_safe: PhantomData
		}
	}

	#[inline]
	fn strong_count(this: ptr::NonNull<ThreadCounter>) -> usize {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).strong }
	}

	#[inline]
	fn weak_count(this: ptr::NonNull<ThreadCounter>) -> usize {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).weak }
	}

	#[inline]
	fn inc_strong_for_clone(this: ptr::NonNull<ThreadCounter>) {
		// SAFETY: `this` is valid per caller contract
		let ptr = unsafe { &raw mut (*this.as_ptr()).strong };

		// SAFETY: same as above
		let old = unsafe { *ptr };

		// SAFETY: same as above, the ptr projection is valid
		unsafe { ptr.write(old + 1) }
	}

	#[inline]
	fn dec_strong_for_drop(this: ptr::NonNull<ThreadCounter>) -> bool {
		// SAFETY: `this` is valid per caller contract
		let ptr = unsafe { &raw mut (*this.as_ptr()).strong };

		// SAFETY: same as above
		let old = unsafe { *ptr };

		// SAFETY: same as above, so this ptr projection is valid
		unsafe { ptr.write(old - 1) }

		old == 1
	}

	#[inline]
	fn inc_weak_for_clone(this: ptr::NonNull<ThreadCounter>) {
		// SAFETY: `this` is valid per caller contract
		let ptr = unsafe { &raw mut (*this.as_ptr()).weak };

		// SAFETY: same as above
		let old = unsafe { *ptr };

		// SAFETY: same as above, the ptr projection is valid
		unsafe { ptr.write(old + 1) }
	}

	#[inline]
	fn dec_weak_for_drop(this: ptr::NonNull<ThreadCounter>) -> bool {
		// SAFETY: `this` is valid per caller contract
		let ptr = unsafe { &raw mut (*this.as_ptr()).weak };

		// SAFETY: same as above
		let old = unsafe { *ptr };

		// SAFETY: same as above, so this ptr projection is valid
		unsafe { ptr.write(old - 1) }

		old == 1
	}
}

pub struct AtomicCounter {
	strong: AtomicUsize,
	weak: AtomicUsize
}

impl Counter for AtomicCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: AtomicUsize::new(1),
			weak: AtomicUsize::new(1)
		}
	}

	#[inline]
	fn strong_count(this: ptr::NonNull<Self>) -> usize {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).strong.load(Relaxed) }
	}

	#[inline]
	fn weak_count(this: ptr::NonNull<Self>) -> usize {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).weak.load(Relaxed) }
	}

	#[inline]
	fn inc_strong_for_clone(this: ptr::NonNull<Self>) {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).strong.fetch_add(1, Relaxed); }
	}

	#[inline]
	fn dec_strong_for_drop(this: ptr::NonNull<Self>) -> bool {
		// SAFETY: `this` is valid per caller contract
		let old_count = unsafe { (*this.as_ptr()).strong.fetch_sub(1, Release) };
		if old_count != 1 { return false }

		atomic::fence(Acquire);

		true
	}

	#[inline]
	fn inc_weak_for_clone(this: ptr::NonNull<Self>) {
		// SAFETY: `this` is valid per caller contract
		unsafe { (*this.as_ptr()).weak.fetch_add(1, Relaxed); }
	}

	#[inline]
	fn dec_weak_for_drop(this: ptr::NonNull<Self>) -> bool {
		// SAFETY: `this` is valid per caller contract
		let old_count = unsafe { (*this.as_ptr()).weak.fetch_sub(1, Release) };
		if old_count != 1 { return false }

		atomic::fence(Acquire);

		true
	}
}
