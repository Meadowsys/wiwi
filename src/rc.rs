use crate::prelude_std::*;

pub use self::inner::{ Counter, ThreadCounter, AtomicCounter };

/// This module encapsulates and provides low level manipulation APIs on
/// a raw and opaque inner reference counting pointer
mod inner;

/// Reference counted thin pointer, that can hold one sized
/// value and one (dynamically sized) slice
#[repr(transparent)]
pub struct Rc<C: Counter, V, S> {
	/// Opaque reference to inner data
	inner: inner::RcInner<C, V, S>
}

/// Weak pointer to a reference counted thin pointer [`Rc`]
#[repr(transparent)]
pub struct RcWeak<C: Counter, V, S> {
	/// Opaque reference to inner data
	inner: inner::RcInner<C, V, S>
}

/// Single threaded reference counting thin pointer
pub type RcThread<V, S = ()> = Rc<ThreadCounter, V, S>;

/// Weak pointer to a single threaded reference counted thin pointer [`RcThread`]
pub type RcThreadWeak<V, S = ()> = RcWeak<ThreadCounter, V, S>;

/// Atomically counted reference counting thin pointer
pub type RcAtomic<V, S = ()> = Rc<AtomicCounter, V, S>;

/// Weak pointer to an atomically counted reference counted thin pointer [`RcAtomic`]
pub type RcAtomicWeak<V, S = ()> = RcWeak<AtomicCounter, V, S>;

impl<C: Counter, V> Rc<C, V, ()> {
	/// Creates a reference counter from a (sized) value, storing it in the `value`
	/// field
	#[inline]
	pub fn from_value(value: V) -> Self {
		Self { inner: inner::new_from_value(value) }
	}
}

impl<C: Counter, S> Rc<C, (), S> {
	/// Creates a reference counter from an array, storing it in the `slice` field
	/// and erasing the array length from the type
	///
	/// If you want to keep the const length of the array, you should use
	/// [`from_value`](Rc::from_value) instead. The memory usage of the RC
	/// allocation actually does not differ between these two.
	#[inline]
	pub fn from_array_into_slice<const N: usize>(array: [S; N]) -> Self {
		Self { inner: inner::new_from_array_into_slice(array) }
	}
}

impl<C: Counter, S: Clone> Rc<C, (), S> {
	/// Creates a reference counter from a slice, cloning all elements into the
	/// `slice` field
	///
	/// If your slice contains elements that implement copy, you should use
	/// [`from_slice_copy`](Rc::from_slice_copy) instead, which can be more
	/// efficient. (Rust, specialisation when?)
	#[inline]
	pub fn from_slice_clone(slice: &[S]) -> Self {
		Self { inner: inner::new_from_slice_clone(slice) }
	}
}

impl<C: Counter, S: Copy> Rc<C, (), S> {
	/// Creates a reference counter from a slice, copying all elements into
	/// the `slice` field
	#[inline]
	pub fn from_slice_copy(slice: &[S]) -> Self {
		Self { inner: inner::new_from_slice_copy(slice) }
	}
}

impl<C: Counter, V, S> Rc<C, V, S> {
	/// Creates a reference counter from a value and an array, with the array
	/// being stored in the `slice` field
	#[inline]
	pub fn from_value_and_array_into_slice<const N: usize>(value: V, array: [S; N]) -> Self {
		Self { inner: inner::new_from_value_and_array_into_slice(value, array) }
	}
}

impl<C: Counter, V, S: Clone> Rc<C, V, S> {
	/// Creates a reference counter from a value and a slice, cloning all
	/// elements of the slice into the `slice` field
	///
	/// If your slice contains elements that implement copy, you should use
	/// [`from_value_and_slice_copy`](Rc::from_value_and_slice_copy) instead,
	/// which can be more efficient. (Rust, specialisation when?)
	#[inline]
	pub fn from_value_and_slice_clone(value: V, slice: &[S]) -> Self {
		Self { inner: inner::new_from_value_and_slice_clone(value, slice) }
	}
}

impl<C: Counter, V, S: Copy> Rc<C, V, S> {
	/// Creates a reference counter from a value and a slice, copying all
	/// elements of the slice into the `slice` field
	#[inline]
	pub fn from_value_and_slice_copy(value: V, slice: &[S]) -> Self {
		Self { inner: inner::new_from_value_and_slice_copy(value, slice) }
	}
}

impl<C: Counter, V, S> Rc<C, V, S> {
	/// Gets an immurable reference to the value stored in the `value` field
	#[inline]
	pub fn as_value_ref(&self) -> &V {
		// SAFETY: ptr is valid
		unsafe { inner::value_ref(self.inner) }
	}

	/// Gets an immurable reference to the slice stored in the `slice` field
	#[inline]
	pub fn as_slice_ref(&self) -> &[S] {
		// SAFETY: ptr is valid
		unsafe { inner::slice_ref(self.inner) }
	}

	/// Gets the strong pointer count
	#[inline]
	pub fn strong_count(&self) -> usize {
		// SAFETY: ptr is valid
		unsafe { inner::strong_count(self.inner) }
	}

	/// Gets the weak pointer count
	#[inline]
	pub fn weak_count(&self) -> usize {
		// SAFETY: ptr is valid
		unsafe { inner::weak_count(self.inner) - 1 }
	}

	/// "Downgrades" this pointer, returning a weak pointer [`RcWeak`] to the data
	#[inline]
	pub fn downgrade(&self) -> RcWeak<C, V, S> {
		// SAFETY: ptr is valid
		unsafe { inner::inc_weak_for_clone(self.inner) }

		RcWeak { inner: self.inner }
	}
}

impl<C: Counter, V, S> Clone for Rc<C, V, S> {
	/// Creates a new strong pointer to the same allocation,
	/// incrementing the strong count
	#[inline]
	fn clone(&self) -> Self {
		// SAFETY: ptr is valid
		unsafe { inner::inc_strong_for_clone(self.inner) }
		Self { inner: self.inner }
	}
}

impl<C: Counter, V, S> Drop for Rc<C, V, S> {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: ptr is valid
		let should_drop = unsafe { inner::dec_strong_for_drop(self.inner) };

		if !should_drop { return }

		// take care of the "fake" weak ptr collectively held by strong ptrs
		let _weak = RcWeak { inner: self.inner };

		// SAFETY: we checked we should drop, and early exit if we shouldn't
		unsafe { inner::drop_instance(self.inner) }
	}
}

impl<C: Counter, V, S> RcWeak<C, V, S> {
	/// Gets the strong pointer count
	#[inline]
	pub fn strong_count(&self) -> usize {
		// SAFETY: ptr is valid
		unsafe { inner::strong_count(self.inner) }
	}

	/// Gets the weak pointer count
	#[inline]
	pub fn weak_count(&self) -> usize {
		// SAFETY: ptr is valid
		let weak = unsafe { inner::weak_count_from_weak_ref(self.inner) };

		// SAFETY: same as above
		let strong = unsafe { inner::strong_count(self.inner) };

		#[expect(clippy::as_conversions)]
		let result = weak - (strong > 0) as usize;

		result
	}

	/// "Upgrades" this pointer, returning a strong pointer [`Rc`] to the data
	/// if there are still other strong pointers to it
	#[inline]
	pub fn upgrade(&self) -> Option<Rc<C, V, S>> {
		// SAFETY: ptr is valid
		let should_upgrade = unsafe { inner::inc_strong_for_upgrade(self.inner) };

		should_upgrade.then(|| Rc { inner: self.inner })
	}
}

impl<C: Counter, V, S> Clone for RcWeak<C, V, S> {
	/// Creates a new weak pointer to the same allocation,
	/// incrementing the weak count
	#[inline]
	fn clone(&self) -> Self {
		// SAFETY: ptr is valid
		unsafe { inner::inc_weak_for_clone(self.inner) }

		Self { inner: self.inner }
	}
}

impl<C: Counter, V, S> Drop for RcWeak<C, V, S> {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: ptr is valid
		let should_dealloc = unsafe { inner::dec_weak_for_drop(self.inner) };

		if !should_dealloc { return }

		// SAFETY: we checked we should dealloc, and early exit if we shouldn't
		unsafe { inner::dealloc_instance(self.inner) }
	}
}

// SAFETY: we are `Send` if the counter/value/slice are all `Send`
unsafe impl<C: Send + Counter, V: Send, S: Send> Send for Rc<C, V, S> {}
// SAFETY: same as above
unsafe impl<C: Send + Counter, V: Send, S: Send> Send for RcWeak<C, V, S> {}

// SAFETY: we are `Sync` if the counter/value/slice are all `Sync`
unsafe impl<C: Sync + Counter, V: Sync, S: Sync> Sync for Rc<C, V, S> {}
// SAFETY: same as above
unsafe impl<C: Sync + Counter, V: Sync, S: Sync> Sync for RcWeak<C, V, S> {}

#[repr(transparent)]
pub struct RcStr<C: Counter, M = ()> {
	inner: Rc<C, M, u8>
}

#[repr(transparent)]
pub struct RcStrWeak<C: Counter, M = ()> {
	inner: RcWeak<C, M, u8>
}

/// Single threaded reference counting thin pointer to a [`str`],
/// optionally carrying arbitrary additional metadata
pub type RcStrThread<M = ()> = RcStr<ThreadCounter, M>;

/// Weak pointer to a single threaded reference counted thin pointer [`RcStrThread`]
pub type RcStrThreadWeak<M = ()> = RcStrWeak<ThreadCounter, M>;

/// Atomically counted reference counting thin pointer to a [`str`],
/// optionally carrying arbitrary additional metadata
pub type RcStrAtomic<M = ()> = RcStr<AtomicCounter, M>;

/// Weak pointer to an atomically counted reference counted thin pointer [`RcStrAtomic`]
pub type RcStrAtomicWeak<M = ()> = RcStrWeak<AtomicCounter, M>;

impl<C: Counter> RcStr<C> {
	#[inline]
	pub fn new(s: &str) -> Self {
		Self { inner: Rc::from_slice_copy(s.as_bytes()) }
	}
}

impl<C: Counter, M> RcStr<C, M> {
	#[inline]
	pub fn with_metadata(s: &str, metadata: M) -> Self {
		Self { inner: Rc::from_value_and_slice_copy(metadata, s.as_bytes()) }
	}
}

impl<C: Counter, M> Deref for RcStr<C, M> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		// SAFETY: `RcStr` has variant of containing valid utf-8
		unsafe { str::from_utf8_unchecked(self.inner.as_slice_ref()) }
	}
}
