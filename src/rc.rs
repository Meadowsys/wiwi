// todo remove this lol
#![allow(dead_code, unused_imports, unused_variables)]

use crate::prelude_std::*;
use self::alloc_mod::Layout;
use self::counter_access::{ CounterAccess, Thread, Atomic };

#[expect(clippy::as_conversions)]
const MAX_REFS: usize = isize::MAX as _;

/// Assert at compile time that `usize` alignment satisfies (is larger than or
/// equal to) `AtomicUsize` align
const _: () = assert!(
	align_of::<usize>() >= align_of::<AtomicUsize>(),
	"`AtomicUsize` align is greater than `usize` align (please report this)"
);

/// Reference counting thin pointer, that can hold one sized
/// value (struct) and one (dynamically sized) slice
pub struct Rc<T, U, A: CounterAccess = Thread> {
	ptr: ptr::NonNull<RcInner<T, U, A>>
}

impl<T, A: CounterAccess> Rc<T, (), A> {
	#[inline]
	pub fn new(value: T) -> Self {
		let ptr = RcInner::alloc(0);

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, 0, value) }

		Self { ptr }
	}
}

impl<U, A: CounterAccess> Rc<(), U, A> {
	#[inline]
	pub fn from_slice_copy(slice: &[U]) -> Self
	where
		U: Copy
	{
		let ptr = RcInner::alloc(slice.len());

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, slice.len(), ()) }

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
	pub fn from_slice_clone(slice: &[U]) -> Self
	where
		U: Clone
	{
		let ptr = RcInner::alloc(slice.len());

		// SAFETY: just allocated
		unsafe { Self::init_strong_weak_len_value(ptr, slice.len(), ()) }

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
}

impl<T, U, A: CounterAccess> Rc<T, U, A> {
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

	// /// # Safety
	// ///
	// /// `ptr` must point to a valid allocation of this type, and the field `strong`
	// /// must be initialised.
	// #[inline]
	// unsafe fn strong(ptr: ptr::NonNull<Self>) -> usize {
	// 	// SAFETY: caller promises ptr is valid, and `strong` field is initialised
	// 	unsafe { *Self::strong_ptr(ptr).as_ptr() }
	// }

	// /// # Safety
	// ///
	// /// `ptr` must point to a valid allocation of this type, and the field `weak`
	// /// must be initialised.
	// #[inline]
	// unsafe fn weak(ptr: ptr::NonNull<Self>) -> usize {
	// 	// SAFETY: caller promises ptr is valid, and `weak` field is initialised
	// 	unsafe { *Self::weak_ptr(ptr).as_ptr() }
	// }

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
	}

	/// Non-atomic (single thread) access to reference counts
	pub struct Thread;

	/// SAFETY: we provide valid single thread access (regular increment/decrement)
	unsafe impl CounterAccess for Thread {
		#[inline]
		unsafe fn initial_store(ptr: ptr::NonNull<usize>) {
			// SAFETY: caller promises value is valid to write to.
			unsafe { ptr.write(1) }
		}
	}

	/// Atomic (thread-safe) access to reference counts
	pub struct Atomic;

	/// SAFETY: we provide valid atomic access
	unsafe impl CounterAccess for Atomic {
		#[inline]
		unsafe fn initial_store(ptr: ptr::NonNull<usize>) {
			// SAFETY: caller promises value is valid to write to.
			// Additionally, this is the only reference to this value right now,
			// we don't need to do an atomic store
			unsafe { ptr.write(1) }
		}
	}
}
