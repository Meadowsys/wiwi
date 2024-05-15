use std::collections::TryReserveError;
use std::ops::{ Deref, DerefMut, RangeBounds };
use super::SliceChain;

// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

impl<T> VecChain<T> {
	#[inline]
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}

	// TODO: nightly try_with_capacity

	// TODO: from_raw_parts
}

// TODO: allocator param
impl<T> VecChain<T> {
	#[inline]
	pub fn from_vec(vec: Vec<T>) -> Self {
		vec.into()
	}

	// TODO: nightly new_in
	// TODO: nightly with_capacity_in
	// TODO: nightly try_with_capacity_in
	// TODO: nightly from_raw_parts_in
	// TODO: nightly into_raw_parts
	// TODO: nightly into_raw_parts_with_alloc

	// TODO: capacity
	// fn capacity(self, capacity: &mut usize)
	// and then *capacity = self.inner.capacity()?????

	#[inline]
	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	#[inline]
	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}

	// TODO: figure out try_methods and errors
	// cause if it errors, the thing is just gonna get consumed?
	// maybe we return (Error, Self)? but that's clunky hmm

	// try_reserve
	// try_reserve_exact

	#[inline]
	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	#[inline]
	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	// TODO: into_boxed_slice (maybe should chain?)

	#[inline]
	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}

	#[inline]
	pub fn as_slice(&self) -> &SliceChain<T> {
		(&*self.inner).into()
	}

	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut SliceChain<T> {
		(&mut *self.inner).into()
	}

	// #[inline]
	// pub fn as_ptr(&self) -> *const T {
	// 	self.inner.as_ptr()
	// }

	// #[inline]
	// pub fn as_mut_ptr(&mut self) -> *mut T {
	// 	self.inner.as_mut_ptr()
	// }

	// TODO: nightly allocator

	#[inline]
	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
		self.inner.set_len(new_len);
		self
	}

	// TODO: swap_remove

	#[inline]
	pub fn insert(mut self, index: usize, element: T) -> Self {
		self.inner.insert(index, element);
		self
	}

	// TODO: remove

	#[inline]
	pub fn retain<F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> bool
	{
		self.inner.retain(f);
		self
	}

	#[inline]
	pub fn retain_mut<F>(mut self, f: F) -> Self
	where
		F: FnMut(&mut T) -> bool
	{
		self.inner.retain_mut(f);
		self
	}

	#[inline]
	pub fn dedup_by_key<F, K>(mut self, key: F) -> Self
	where
		F: FnMut(&mut T) -> K,
		K: PartialEq
	{
		self.inner.dedup_by_key(key);
		self
	}

	#[inline]
	pub fn dedup_by<F>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut T, &mut T) -> bool
	{
		self.inner.dedup_by(same_bucket);
		self
	}

	#[inline]
	pub fn push(mut self, value: T) -> Self {
		self.inner.push(value);
		self
	}

	// TODO: nigntly push_within_capacity
	// TODO: pop

	#[inline]
	pub fn append(mut self, other: &mut Self) -> Self {
		self.inner.append(&mut other.inner);
		self
	}

	// TODO: drain

	#[inline]
	pub fn clear(mut self) -> Self {
		self.inner.clear();
		self
	}

	// TODO: len
	// TODO: is_empty
	// TODO: split_off?

	#[inline]
	pub fn resize_with<F>(mut self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> T
	{
		self.inner.resize_with(new_len, f);
		self
	}

	// TODO: leak (chain?)
	// TODO: spare_capacity_mut (chain?)
	// TODO: nightly split_at_spare_mut
}

// TODO: allocator param
impl<T: Clone> VecChain<T> {
	#[inline]
	pub fn resize(mut self, new_len: usize, value: T) -> Self {
		self.inner.resize(new_len, value);
		self
	}

	// TODO: version that takes chainin?
	#[inline]
	pub fn extend_from_slice(mut self, other: &[T]) -> Self {
		self.inner.extend_from_slice(other);
		self
	}

	#[inline]
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
	// TODO: nightly into_flattened
}

// TODO: allocator param
impl<T: PartialEq> VecChain<T> {
	#[inline]
	pub fn dedup(mut self) -> Self {
		self.inner.dedup();
		self
	}
}

// TODO: allocator param
impl<T> VecChain<T> {
	// TODO: splice
	// TODO: extract_if
}

// TODO: check docs trait impls

// TODO: These... should probably deref to our chaining slice API? if we add it

// // TODO: allocator param
// impl<T> Deref for VecChain<T> {
// 	type Target = Vec<T>;
//
// 	#[inline]
// 	fn deref(&self) -> &Vec<T> {
// 		&self.inner
// 	}
// }

// // TODO: allocator param
// impl<T> DerefMut for VecChain<T> {
// 	#[inline]
// 	fn deref_mut(&mut self) -> &mut Vec<T> {
// 		&mut self.inner
// 	}
// }

// TODO: allocator param
impl<T> AsRef<VecChain<T>> for VecChain<T> {
	#[inline]
	fn as_ref(&self) -> &Self {
		self
	}
}

// TODO: allocator param
impl<T> AsMut<VecChain<T>> for VecChain<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut Self {
		self
	}
}

// TODO: allocator param
impl<T> AsRef<Vec<T>> for VecChain<T> {
	#[inline]
	fn as_ref(&self) -> &Vec<T> {
		&self.inner
	}
}

// TODO: allocator param
impl<T> AsMut<Vec<T>> for VecChain<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}
}

// TODO: allocator param
impl<T> AsRef<[T]> for VecChain<T> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		&self.inner
	}
}

// TODO: allocator param
impl<T> AsMut<[T]> for VecChain<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}

impl<T> From<Vec<T>> for VecChain<T> {
	#[inline]
	fn from(inner: Vec<T>) -> Self {
		Self { inner }
	}
}

// TODO: allocator param
impl<T> VecChain<T> {
	#[inline]
	pub fn as_vec(&self) -> &Vec<T> {
		&self.inner
	}

	#[inline]
	pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}

	#[inline]
	pub fn into_inner(self) -> Vec<T> {
		self.inner
	}
}
