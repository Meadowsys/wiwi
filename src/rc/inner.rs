use crate::prelude_std::*;

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
fn new_uninit_thread<V, S>(len: usize) -> RcInner<ThreadCounter, V, S> {
	alloc_instance(len)
}

#[inline]
fn new_uninit_atomic<V, S>(len: usize) -> RcInner<AtomicCounter, V, S> {
	alloc_instance(len)
}

#[inline]
fn alloc_instance<C: Counter, V, S>(len: usize) -> RcInner<C, V, S> {
	let layout = calc_layout::<C, V, S>(len);

	// SAFETY: `calc_layout` never returns layout with 0 size
	let ptr = unsafe { alloc(layout) };

	let Some(ptr) = ptr::NonNull::new(ptr.cast()) else {
		alloc_mod::handle_alloc_error(layout)
	};

	RcInner { ptr }
}

/// # Safety
///
/// This must be the first time this is called on this particular `instance`.
#[inline]
unsafe fn drop_instance<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	todo!()
}

/// # Safety
///
/// This must be the first time this is called on this particular `instance`.
#[inline]
unsafe fn dealloc_instance<C: Counter, V, S>(instance: RcInner<C, V, S>) {
	todo!()
}

/// Calculate the layout to allocate a new instance with the specified counter,
/// value type, slice type, and slice length
#[inline]
fn calc_layout<C: Counter, V, S>(len: usize) -> alloc_mod::Layout {
	alloc_mod::Layout::new::<RcInner<C, V, S>>()
		.extend(alloc_mod::Layout::array::<S>(len).unwrap())
		.unwrap()
		.0
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn counter_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<C> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).counter };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn counter_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h MaybeUninit<C> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &*ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
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
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn slice_len_ptr<C: Counter, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<usize> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw const (*instance.ptr.as_ptr()).slice_len };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr.cast_mut()) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn slice_len_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h MaybeUninit<usize> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_len_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &*ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
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
#[inline]
unsafe fn value_uninit<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h MaybeUninit<V> {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &*ptr.cast() }
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
/// - The provided `instance` must have `len` elements in `slice` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn slice_ref<'h, C: Counter, V, S>(instance: RcInner<C, V, S>) -> &'h [S] {
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: caller promises to uphold the requirements
	let len = unsafe { slice_len(instance) };

	// SAFETY: caller promises ptr is valid for at least `len` elements
	unsafe { slice::from_raw_parts(ptr, len) }
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
}
