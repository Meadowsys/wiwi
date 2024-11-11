use crate::prelude_std::*;
use super::{ Rc, RcWeak, Counter, ThreadCounter, AtomicCounter };

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

impl<C: Counter, M> RcStr<C, M> {
	#[inline]
	pub fn strong_count(&self) -> usize {
		self.inner.strong_count()
	}

	#[inline]
	pub fn weak_count(&self) -> usize {
		self.inner.weak_count()
	}

	#[inline]
	pub fn downgrade(&self) -> RcStrWeak<C, M> {
		RcStrWeak { inner: self.inner.downgrade() }
	}
}

impl<C: Counter, M> Clone for RcStr<C, M> {
	#[inline]
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<C: Counter, M> Deref for RcStr<C, M> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		// SAFETY: `RcStr` has invariant of containing valid utf-8
		unsafe { str::from_utf8_unchecked(self.inner.as_slice_ref()) }
	}
}

impl<C: Counter, M> RcStrWeak<C, M> {
	#[inline]
	pub fn strong_count(&self) -> usize {
		self.inner.strong_count()
	}

	#[inline]
	pub fn weak_count(&self) -> usize {
		self.inner.weak_count()
	}

	#[inline]
	pub fn upgrade(&self) -> Option<RcStr<C, M>> {
		self.inner.upgrade().map(|inner| RcStr { inner })
	}
}

impl<C: Counter, M> Clone for RcStrWeak<C, M> {
	#[inline]
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}
