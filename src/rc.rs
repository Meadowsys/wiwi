// todo remove this lol
#![allow(dead_code, unused_imports, unused_variables)]

use crate::prelude_std::*;
use self::alloc_mod::Layout;
use self::counter_access::{ CounterAccess, Thread, Atomic };

/// Assert at compile time that `usize` alignment satisfies (is larger than or
/// equal to) `AtomicUsize` align
const _: () = assert!(
	align_of::<usize>() >= align_of::<AtomicUsize>(),
	"`AtomicUsize` align is greater than `usize` align (please report this)"
);

/// Reference counting thin pointer, that can hold one sized
/// value (struct) and one (dynamically sized) slice
#[repr(transparent)]
pub struct Rc<T, U, A: CounterAccess = Thread> {
	ptr: ptr::NonNull<RcInner<T, U, A>>
}

#[repr(transparent)]
pub struct RcWeak<T, U, A: CounterAccess = Thread> {
	ptr: ptr::NonNull<RcInner<T, U, A>>
}

pub type Arc<T, U> = Rc<T, U, Atomic>;
pub type ArcWeak<T, U> = RcWeak<T, U, Atomic>;

impl<T, A: CounterAccess> Rc<T, (), A> {
	#[inline]
	pub fn new(value: T) -> Self {
		let ptr = RcInner::alloc(0);

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, 0, value) }

		Self { ptr }
	}

	#[inline]
	pub fn from_array_into_data<const N: usize>(array: [T; N]) -> Rc<[T; N], (), A> {
		Rc::new(array)
	}
}

impl<U, A: CounterAccess> Rc<(), U, A> {
	#[inline]
	pub fn from_slice_copy(slice: &[U]) -> Self
	where
		U: Copy
	{
		Rc::from_value_and_slice_copy((), slice)
	}

	#[inline]
	pub fn from_slice_clone(slice: &[U]) -> Self
	where
		U: Clone
	{
		Rc::from_value_and_slice_clone((), slice)
	}

	#[inline]
	pub fn from_array_into_slice<const N: usize>(array: [U; N]) -> Self {
		let array = ManuallyDrop::new(array);

		// SAFETY: we put the array in `ManuallyDrop` to avoid double drop
		unsafe { Self::from_value_and_slice_copy_unchecked((), &*array) }
	}
}

impl<T, U, A: CounterAccess> Rc<T, U, A> {
	#[inline]
	pub fn from_value_and_slice_copy(value: T, slice: &[U]) -> Self
	where
		U: Copy
	{
		// SAFETY: we have `U: Copy` bound
		unsafe { Self::from_value_and_slice_copy_unchecked(value, slice) }
	}

	/// # Safety
	///
	/// You must make sure that `U: Copy` is satisfied, or the old slice is
	/// prevented from being dropped to avoid double drop.
	#[inline]
	unsafe fn from_value_and_slice_copy_unchecked(value: T, slice: &[U]) -> Self {
		let ptr = RcInner::alloc(slice.len());

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, slice.len(), value) }

		// SAFETY: just allocated, and with `slice.len()` as arg so
		// slice_ptr is valid for `slice.len()` writes
		unsafe {
			ptr::copy_nonoverlapping(
				slice.as_ptr(),
				RcInner::slice_ptr(ptr).as_ptr(),
				slice.len()
			)
		}

		Self { ptr }
	}

	#[inline]
	pub fn from_value_and_slice_clone(value: T, slice: &[U]) -> Self
	where
		U: Clone
	{
		let ptr = RcInner::alloc(slice.len());

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, slice.len(), value) }

		let slice_ptr = RcInner::slice_ptr(ptr).as_ptr();
		slice.iter().enumerate().for_each(|(i, value)| {
			// SAFETY: `ptr` is valid for `slice.len()` writes, and
			// `i` is enumerating over `0..slice.len()`
			let ptr = unsafe { slice_ptr.add(i) };

			// SAFETY: see above
			unsafe { ptr.write(value.clone()) }
		});

		Self { ptr }
	}

	/// # Safety
	///
	/// `ptr` must be just allocated, and allocated with `len` as allocation argument.
	#[inline]
	unsafe fn init_strong_weak_len_value(ptr: ptr::NonNull<RcInner<T, U, A>>, len: usize, value: T) {
		// SAFETY: caller promises pointer was just allocated
		unsafe { A::initial_store(RcInner::strong_ptr(ptr)) }

		// SAFETY: caller promises pointer was just allocated
		unsafe { A::initial_store(RcInner::weak_ptr(ptr)) }

		// SAFETY: caller promises pointer was just allocated
		unsafe { RcInner::len_ptr(ptr).write(0) }

		// SAFETY: caller promises pointer was just allocated
		unsafe { RcInner::value_ptr(ptr).write(value) }
	}
}

impl<T, U, A: CounterAccess> Rc<T, U, A> {
	#[inline]
	pub fn downgrade(this: &Self) -> RcWeak<T, U, A> {
		// SAFETY: ptr obtained from `this.ptr`
		let prev_value = unsafe { A::increment_relaxed(RcInner::weak_ptr(this.ptr)) };

		RcWeak { ptr: this.ptr }
	}

	#[inline]
	pub fn value(&self) -> &T {
		// SAFETY: ptr taken from `self.ptr`
		unsafe { RcInner::value(self.ptr) }
	}

	#[inline]
	pub fn slice(&self) -> &[U] {
		// SAFETY: ptr taken from `self.ptr`
		unsafe { RcInner::slice(self.ptr) }
	}

	#[inline]
	pub fn strong_count(&self) -> usize {
		// SAFETY: ptr obtained from `self.ptr`
		unsafe { A::simple_get(RcInner::strong_ptr(self.ptr)) }
	}

	#[inline]
	pub fn weak_count(&self) -> usize {
		// SAFETY: ptr obtained from `self.ptr`
		unsafe { A::simple_get(RcInner::weak_ptr(self.ptr)) }
	}
}

impl<T, U, A: CounterAccess> Drop for Rc<T, U, A> {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: ptr obtained from `self.ptr`
		let prev_value = unsafe { A::decrement_release(RcInner::strong_ptr(self.ptr)) };
		if prev_value != 1 { return }

		atomic::fence(atomic::Ordering::Acquire);

		// Let this drop to take care of the "fake" weak pointer held
		// by the strong pointers
		let _weak = RcWeak { ptr: self.ptr };

		// SAFETY: we are last strong reference, drop the value but keep the
		// allocation for the weak pointers.
		unsafe { RcInner::drop_contents(self.ptr) }
	}
}

impl<T, U, A: CounterAccess> RcWeak<T, U, A> {
	#[inline]
	pub fn upgrade(&self) -> Option<Rc<T, U, A>> {
		// SAFETY: ptr obtained from `this.ptr`
		let can_upgrade = unsafe { A::try_increment_for_upgrade(RcInner::strong_ptr(self.ptr)) };

		can_upgrade.then(|| Rc { ptr: self.ptr })
	}
}

impl<T, U, A: CounterAccess> Drop for RcWeak<T, U, A> {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: ptr obtained from `self.ptr`
		let prev_value = unsafe { A::decrement_release(RcInner::weak_ptr(self.ptr)) };
		if prev_value != 1 { return }

		atomic::fence(atomic::Ordering::Acquire);

		// SAFETY: we are last weak pointer and no more strong pointers exist,
		// time to deallocate (contents were dropped by last strong pointer)
		unsafe { RcInner::dealloc(self.ptr) }
	}
}

// SAFETY: we are `Send` if all the data and the counter access method is `Send`
unsafe impl<T: Send, U: Send, A: Send + CounterAccess> Send for Rc<T, U, A> {}

// SAFETY: we are `Sync` if all the data and the counter access method is `Sync`
unsafe impl<T: Sync, U: Sync, A: Sync + CounterAccess> Sync for RcWeak<T, U, A> {}

#[repr(C)]
struct RcInner<T, U, A: CounterAccess> {
	_marker: PhantomData<A>,
	strong: usize,
	weak: usize,
	len: usize,
	value: T,

	// force alignment of (at least) T's alignment, while also not
	// requiring there be at least 1 T element
	slice: [U; 0]
}

impl<T, U, A: CounterAccess> RcInner<T, U, A> {
	#[inline]
	fn alloc(len: usize) -> ptr::NonNull<Self> {
		let layout = Self::calc_layout(len);

		// SAFETY: `RcInnerStruct` is never ZST
		let ptr = unsafe { alloc(layout) };

		let Some(ptr) = ptr::NonNull::new(ptr.cast()) else {
			alloc_mod::handle_alloc_error(layout)
		};

		ptr
	}

	/// # Safety
	///
	/// `ptr` must be a valid pointer to a valid instance of self allocated using
	/// the global allocator, and fully initialised (ie. gotten from `Self::alloc`
	/// then initialised, is fine).
	///
	/// This function does not deallocate the memory pointed to by pointer
	/// (see [`RcInner::dealloc`]).
	#[inline]
	unsafe fn drop_contents(ptr: ptr::NonNull<Self>) {
		// SAFETY: caller promises `ptr` is valid instance of `Self`
		unsafe { ptr::drop_in_place(Self::value_ptr(ptr).as_ptr()) }

		// SAFETY: caller promises to uphold invariants of `Self::slice`
		let slice = unsafe { Self::slice(ptr) };

		#[expect(clippy::as_conversions)]
		let slice_ptr = slice as *const [U] as *mut [U];

		// SAFETY: ptr derived from a valid slice from a valid ptr
		unsafe { ptr::drop_in_place(slice_ptr) }
	}

	/// # Safety
	///
	/// `ptr` must be a valid pointer to a valid instance of self allocated using
	/// the global allocator, and its `len` field must contain the correct amount
	/// for the elements in `slice` (ie. gotten from `Self::alloc`, then length
	/// is set, is fine).
	///
	/// This function does not drop the elements pointed to in `value` or `slice`
	/// (see [`RcInner::drop_contents`]).
	#[inline]
	unsafe fn dealloc(ptr: ptr::NonNull<Self>) {
		// SAFETY: caller promises `ptr` is valid and follows invariants
		let len = unsafe { Self::len(ptr) };

		let layout = Self::calc_layout(len);

		// SAFETY: caller promises to uphold invariants of `dealloc`
		unsafe { dealloc(ptr.as_ptr().cast(), layout) }
	}

	#[inline]
	fn calc_layout(len: usize) -> Layout {
		let layout_main = Layout::new::<Self>();

		// I believe it won't insert padding after `stuff` to the
		// extended array, but just to be sure
		debug_assert!({
			let layout_extended = layout_main
				.extend(Layout::array::<T>(0).unwrap())
				.unwrap()
				.0;
			layout_main == layout_extended
		});

		layout_main
			.extend(Layout::array::<T>(len).unwrap())
			.unwrap()
			.0
	}

	#[inline]
	fn strong_ptr(ptr: ptr::NonNull<Self>) -> ptr::NonNull<usize> {
		let ptr = ptr.as_ptr();

		// SAFETY: in bounds field projection using raw reference expr
		let ptr = unsafe { &raw mut (*ptr).strong };

		// SAFETY: field projection into nonnull ptr is not null either
		unsafe { ptr::NonNull::new_unchecked(ptr) }
	}

	#[inline]
	fn weak_ptr(ptr: ptr::NonNull<Self>) -> ptr::NonNull<usize> {
		let ptr = ptr.as_ptr();

		// SAFETY: in bounds field projection using raw reference expr
		let ptr = unsafe { &raw mut (*ptr).weak };

		// SAFETY: field projection into nonnull ptr is not null either
		unsafe { ptr::NonNull::new_unchecked(ptr) }
	}

	#[inline]
	fn len_ptr(ptr: ptr::NonNull<Self>) -> ptr::NonNull<usize> {
		let ptr = ptr.as_ptr();

		// SAFETY: in bounds field projection using raw reference expr
		let ptr = unsafe { &raw mut (*ptr).len };

		// SAFETY: field projection into nonnull ptr is not null either
		unsafe { ptr::NonNull::new_unchecked(ptr) }
	}

	#[inline]
	fn value_ptr(ptr: ptr::NonNull<Self>) -> ptr::NonNull<T> {
		let ptr = ptr.as_ptr();

		// SAFETY: in bounds field projection using raw reference expr
		let ptr = unsafe { &raw mut (*ptr).value };

		// SAFETY: field projection into nonnull ptr is not null either
		unsafe { ptr::NonNull::new_unchecked(ptr) }
	}

	#[inline]
	fn slice_ptr(ptr: ptr::NonNull<Self>) -> ptr::NonNull<U> {
		let ptr = ptr.as_ptr();

		// SAFETY: in bounds field projection using raw reference expr
		let ptr = unsafe { (&raw mut (*ptr).slice).cast() };

		// SAFETY: field projection into nonnull ptr is not null either
		unsafe { ptr::NonNull::new_unchecked(ptr) }
	}

	/// # Safety
	///
	/// `ptr` must point to a valid allocation of this type, and the field `len`
	/// must be initialised.
	#[inline]
	unsafe fn len(ptr: ptr::NonNull<Self>) -> usize {
		// SAFETY: caller promises ptr is valid, and `len` field is initialised
		unsafe { *Self::len_ptr(ptr).as_ptr() }
	}

	/// # Safety
	///
	/// `ptr` must point to a valid allocation of this type, and the field `value`
	/// must be initialised.
	#[inline]
	unsafe fn value<'h>(ptr: ptr::NonNull<Self>) -> &'h T {
		// SAFETY: caller promises ptr is valid, and `value` field is initialised
		unsafe { &*Self::value_ptr(ptr).as_ptr() }
	}

	/// # Safety
	///
	/// `ptr` must point to a valid allocation of this type, and fields `slice`
	/// and `len` must be initialised.
	#[inline]
	unsafe fn slice<'h>(ptr: ptr::NonNull<Self>) -> &'h [U] {
		let slice_ptr = Self::slice_ptr(ptr).as_ptr();

		// SAFETY: caller promises ptr is valid, and `len` field is initialised
		let len = unsafe { Self::len(ptr) };

		// SAFETY: caller promises ptr is valid, and `slice` is initialised
		unsafe { slice::from_raw_parts(slice_ptr, len) }
	}
}

#[repr(transparent)]
pub struct RcStr<A: CounterAccess = Thread> {
	inner: Rc<(), u8, A>
}

#[repr(transparent)]
pub struct RcStrWeak<A: CounterAccess = Thread> {
	inner: RcWeak<(), u8, A>
}

impl<A: CounterAccess> Deref for RcStr<A> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		// SAFETY: invariant of `RcStr`
		unsafe { str::from_utf8_unchecked(self.inner.slice()) }
	}
}

impl<A: CounterAccess> From<&str> for RcStr<A> {
	#[inline]
	fn from(s: &str) -> Self {
		Self { inner: Rc::from_slice_copy(s.as_bytes()) }
	}
}

mod counter_access {
	use super::*;

	/// Trait providing increment/decrement functions, allowing [`RcInnerStruct`] to be
	/// generic over atomicity
	///
	/// # Safety
	///
	/// Functions in this trait take pointers to counters (`*mut usize`), callers
	/// must pass pointers that are valid to read and write from (depending
	/// on the function).
	///
	/// Implementor must guarantee the functions have the atomicity they claim to
	/// have as well.
	pub unsafe trait CounterAccess {
		/// Store value of 1 into the provided pointer, as part of initialisation
		///
		/// The pointer must be freshly allocated, which also implies we have
		/// exclusive access, so the store does not have to be atomic (if applicable).
		unsafe fn initial_store(ptr: ptr::NonNull<usize>);

		/// Simple get operation for eg. `rc.strong_count()` calls
		unsafe fn simple_get(ptr: ptr::NonNull<usize>) -> usize;

		/// Decrements the count in `ptr` with `Release` ordering
		/// and returns the _previous_ value
		unsafe fn decrement_release(ptr: ptr::NonNull<usize>) -> usize;

		/// Increments the count in `ptr` with `Relaxed` ordering
		/// and returns the _previous_ value
		unsafe fn increment_relaxed(ptr: ptr::NonNull<usize>) -> usize;

		/// Try to increment the count in `ptr`, returning true if the old count
		/// is greater than 0
		///
		/// Used in [`upgrade`](RcWeak::upgrade).
		unsafe fn try_increment_for_upgrade(ptr: ptr::NonNull<usize>) -> bool;
	}

	/// Non-atomic (single thread) access to reference counts
	pub struct Thread;

	/// Atomic (thread-safe) access to reference counts
	pub struct Atomic {
		_thread_unsafe: PhantomData<*const ()>
	}

	/// SAFETY: we provide valid single thread access (regular increment/decrement)
	unsafe impl CounterAccess for Thread {
		#[inline]
		unsafe fn initial_store(ptr: ptr::NonNull<usize>) {
			// SAFETY: caller promises ptr is valid for writes
			unsafe { ptr.write(1) }
		}

		#[inline]
		unsafe fn simple_get(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			unsafe { *ptr.as_ptr() }
		}

		#[inline]
		unsafe fn decrement_release(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			let prev = unsafe { ptr.read() };

			// SAFETY: see above
			unsafe { ptr.write(prev - 1) }

			prev
		}

		#[inline]
		unsafe fn increment_relaxed(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			let prev = unsafe { ptr.read() };

			// SAFETY: see above
			unsafe { ptr.write(prev + 1) }

			prev
		}

		#[inline]
		unsafe fn try_increment_for_upgrade(ptr: ptr::NonNull<usize>) -> bool {
			// SAFETY: caller promises ptr is valid for reads and writes
			let prev = unsafe { ptr.read() };
			let can_upgrade = prev > 0;

			if can_upgrade {
				// SAFETY: see above
				unsafe { ptr.write(prev + 1) }
			}

			can_upgrade
		}
	}

	/// SAFETY: we provide valid atomic access
	unsafe impl CounterAccess for Atomic {
		#[inline]
		unsafe fn initial_store(ptr: ptr::NonNull<usize>) {
			// SAFETY: caller promises value is valid to write to.
			// Additionally, this is the only reference to this value right now,
			// we don't need to do an atomic store
			unsafe { ptr.write(1) }
		}

		#[inline]
		unsafe fn simple_get(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			let atomic = unsafe { AtomicUsize::from_ptr(ptr.as_ptr()) };

			atomic.load(atomic::Ordering::Relaxed)
		}

		#[inline]
		unsafe fn decrement_release(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			let atomic = unsafe { AtomicUsize::from_ptr(ptr.as_ptr()) };
			atomic.fetch_sub(1, atomic::Ordering::Release)
		}

		#[inline]
		unsafe fn increment_relaxed(ptr: ptr::NonNull<usize>) -> usize {
			// SAFETY: caller promises ptr is valid for reads and writes
			let atomic = unsafe { AtomicUsize::from_ptr(ptr.as_ptr()) };
			// relaxed is fine because we already have a reference, so we
			// won't race to drop/dealloc
			atomic.fetch_add(1, atomic::Ordering::Relaxed)
		}

		#[inline]
		unsafe fn try_increment_for_upgrade(ptr: ptr::NonNull<usize>) -> bool {
			// SAFETY: caller promises ptr is valid for reads and writes
			let atomic = unsafe { AtomicUsize::from_ptr(ptr.as_ptr()) };

			atomic.fetch_update(
				atomic::Ordering::Acquire,
				atomic::Ordering::Relaxed,
				|old| (old > 0).then(|| old + 1)
			).is_ok()
		}
	}
}
