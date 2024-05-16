use std::mem::{ forget, MaybeUninit, size_of };
use super::{ IntoChainer, ToMaybeUninit as _ };

// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

impl<T> VecChain<T> {
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}

	// TODO: nightly try_with_capacity

	// TODO: from_raw_parts
}

// TODO: allocator param
impl<T> VecChain<T> {
	// TODO: nightly new_in
	// TODO: nightly with_capacity_in
	// TODO: nightly try_with_capacity_in
	// TODO: nightly from_raw_parts_in
	// TODO: nightly into_raw_parts
	// TODO: nightly into_raw_parts_with_alloc

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

	// TODO: figure out try_methods and errors
	// cause if it errors, the thing is just gonna get consumed?
	// maybe we return (Error, Self)? but that's clunky hmm

	// try_reserve
	// try_reserve_exact

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	// TODO: into_boxed_slice (maybe should chain?)

	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}
	/*
	as_slice/as_mut_slice
	as_ptr/as_mut_ptr
	nightly allocator
	set_len
	swap_remove
	insert
	remove
	retain/mut
	dedup_by
	dedup_by_key
	push
	push_within_capacity????
	pop
	append
	drain
	clear
	len
	is_empty
	split_off
	resize_with
	leak?
	spare_capacity_mut
	split_at_spare_mut?
	splice
	extract_if


	as_str
	as_bytes
	flatten/mut
	len
	is_empty
	first/mut
	split_first/mut
	split_last/mut
	last
	last_mut
	first_chunk/mut
	split_first_chunk/mut
	split_last_chunk/mut
	last_chunk/mut
	get/mut
	get_unchecked/mut
	swap_unchecked (checked/panic?????)
	reverse
	iter/mut
	windows
	chunks/mut
	chunks_exact/mut
	as_chunks_unchecked
	as_chunks
	as_rchunks
	array_chunks/mut
	as_chunks_unchecked_mut
	as_chunks_mut
	as_rchunks_mut
	array_windows
	rchunks/mut
	rchunks_exact/mut
	chunk_by/mut
	split_at/mut
	split_at_unchecked
	split_at_mut_unchecked
	split_at_checked
	split_at_mut_checked
	split/mut
	split_inclusive/mut
	rsplit/mut
	splitn/mut
	rsplitn/mut
	split_once
	rsplit_once
	contains
	starts_with
	ends_with
	strip_prefix/suffix
	binary_search/by/key
	sort_unstable/by/key
	select_nth_unstable/by/key
	partition_dedup/by/key
	rotate_left/right
	fill
	fill_with
	clone_from_slice
	copy_from_slice
	copy_within
	clone_within (not in std)?
	swap_with_slice
	align_to/mut
	nightly as_simd/mut
	is_sorted/by/key
	partition_point
	take/mut
	take_first/mut
	take_last/mut
	get_many_unchecked_mut
	get_many_mut
	get_many/get_many_unchecked (non mut? not in std?)
	sort_floats
	is_ascii
	as_ascii/unchecked
	eq_ignore_ascii_case
	make_ascii_uppercase/lowercase
	escape_ascii
	trim_ascii
	trim_ascii_start/end
	sort/by/key
	sort_by_cached_key
	to_vec/in????? no lol just clone?
	repeat
	concat
	join
	to_ascii_uppercase/lowercase
	*/
}

// TODO: allocator param
impl<T: Clone> VecChain<T> {
	/*
	resize
	extend_from_slice
	extend_from_within
	*/
}

// TODO: allocator param
impl<T, const N: usize> VecChain<[T; N]> {
	pub fn flatten(mut self) -> VecChain<T> {
		let (len, cap) = if size_of::<T>() == 0 {
			let len = self.inner.len()
				.checked_mul(N)
				.expect("vec len overflow");
			(len, usize::MAX)
		} else {
			unsafe { (
				self.inner.len().unchecked_mul(N),
				self.inner.capacity().unchecked_mul(N)
			) }
		};

		// TODO: switch to into_raw_parts impl when stabilised
		// let (ptr, _len, _capacity) = self.inner.into_raw_parts();

		let ptr = self.inner.as_mut_ptr();
		forget(self);

		let ptr = ptr as *mut T;
		unsafe { Vec::from_raw_parts(ptr, len, cap).into() }
	}
}

// TODO: allocator param
impl<T: PartialEq> VecChain<T> {
	// dedup
}

impl<T> IntoChainer for Vec<T> {
	type Chain = VecChain<T>;
	fn into_chainer(self) -> VecChain<T> {
		self.into()
	}
}

impl<T> From<Vec<T>> for VecChain<T> {
	#[inline]
	fn from(inner: Vec<T>) -> Self {
		Self { inner }
	}
}
/*
AsRef/AsMut for itself, Vec<T>, [T], other chain types
Borrow/mut for slice chainer?
whatever ArrayLike is
clone debug default
drop?? nah
extend T, &T
from &[T], &[T; N], other chainers, cow<[T]>
from muts?
from str, String
from owned array
from binary heap? boxed slice and chainer, cstring for u8? cow slice?
from nonzero u8 for cstring (probably some other crate, I dunno, if we cstring support?)
from vec for cow
from rc, arc,
TODO: stopped around here (from vec for Rc<[T]> something like that)
*/

























// // TODO: allocator param
// impl<T> VecChain<T> {

// 	#[inline]
// 	pub fn as_slice(&self) -> &SliceChain<T> {
// 		(&*self.inner).into()
// 	}

// 	#[inline]
// 	pub fn as_mut_slice(&mut self) -> &mut SliceChain<T> {
// 		(&mut *self.inner).into()
// 	}

// 	// #[inline]
// 	// pub fn as_ptr(&self) -> *const T {
// 	// 	self.inner.as_ptr()
// 	// }

// 	// #[inline]
// 	// pub fn as_mut_ptr(&mut self) -> *mut T {
// 	// 	self.inner.as_mut_ptr()
// 	// }

// 	// TODO: nightly allocator

// 	#[inline]
// 	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
// 		self.inner.set_len(new_len);
// 		self
// 	}

// 	// TODO: swap_remove

// 	#[inline]
// 	pub fn insert(mut self, index: usize, element: T) -> Self {
// 		self.inner.insert(index, element);
// 		self
// 	}

// 	// TODO: remove

// 	#[inline]
// 	pub fn retain<F>(mut self, f: F) -> Self
// 	where
// 		F: FnMut(&T) -> bool
// 	{
// 		self.inner.retain(f);
// 		self
// 	}

// 	#[inline]
// 	pub fn retain_mut<F>(mut self, f: F) -> Self
// 	where
// 		F: FnMut(&mut T) -> bool
// 	{
// 		self.inner.retain_mut(f);
// 		self
// 	}

// 	#[inline]
// 	pub fn dedup_by_key<F, K>(mut self, key: F) -> Self
// 	where
// 		F: FnMut(&mut T) -> K,
// 		K: PartialEq
// 	{
// 		self.inner.dedup_by_key(key);
// 		self
// 	}

// 	#[inline]
// 	pub fn dedup_by<F>(mut self, same_bucket: F) -> Self
// 	where
// 		F: FnMut(&mut T, &mut T) -> bool
// 	{
// 		self.inner.dedup_by(same_bucket);
// 		self
// 	}

// 	#[inline]
// 	pub fn push(mut self, value: T) -> Self {
// 		self.inner.push(value);
// 		self
// 	}

// 	// TODO: nigntly push_within_capacity
// 	// TODO: pop

// 	#[inline]
// 	pub fn append(mut self, other: &mut Self) -> Self {
// 		self.inner.append(&mut other.inner);
// 		self
// 	}

// 	// TODO: drain

// 	#[inline]
// 	pub fn clear(mut self) -> Self {
// 		self.inner.clear();
// 		self
// 	}

// 	// TODO: len
// 	// TODO: is_empty
// 	// TODO: split_off?

// 	#[inline]
// 	pub fn resize_with<F>(mut self, new_len: usize, f: F) -> Self
// 	where
// 		F: FnMut() -> T
// 	{
// 		self.inner.resize_with(new_len, f);
// 		self
// 	}

// 	// TODO: leak (chain?)
// 	// TODO: spare_capacity_mut (chain?)
// 	// TODO: nightly split_at_spare_mut
// }

// // TODO: allocator param
// impl<T: Clone> VecChain<T> {
// 	#[inline]
// 	pub fn resize(mut self, new_len: usize, value: T) -> Self {
// 		self.inner.resize(new_len, value);
// 		self
// 	}

// 	// TODO: version that takes chainin?
// 	#[inline]
// 	pub fn extend_from_slice(mut self, other: &[T]) -> Self {
// 		self.inner.extend_from_slice(other);
// 		self
// 	}

// 	#[inline]
// 	pub fn extend_from_within<R>(mut self, src: R) -> Self
// 	where
// 		R: RangeBounds<usize>
// 	{
// 		self.inner.extend_from_within(src);
// 		self
// 	}
// }

// // TODO: allocator param
// impl<T, const N: usize> VecChain<[T; N]> {
// 	// TODO: nightly into_flattened
// }

// // TODO: allocator param
// impl<T: PartialEq> VecChain<T> {
// 	#[inline]
// 	pub fn dedup(mut self) -> Self {
// 		self.inner.dedup();
// 		self
// 	}
// }

// // TODO: allocator param
// impl<T> VecChain<T> {
// 	// TODO: splice
// 	// TODO: extract_if
// }

// // TODO: the thing about methods from Deref<[T]> (do after slice chain impl)
// // TODO: other trait impls

// // TODO: allocator param
// impl<T> AsRef<VecChain<T>> for VecChain<T> {
// 	#[inline]
// 	fn as_ref(&self) -> &Self {
// 		self
// 	}
// }

// // TODO: allocator param
// impl<T> AsMut<VecChain<T>> for VecChain<T> {
// 	#[inline]
// 	fn as_mut(&mut self) -> &mut Self {
// 		self
// 	}
// }

// // TODO: allocator param
// impl<T> AsRef<Vec<T>> for VecChain<T> {
// 	#[inline]
// 	fn as_ref(&self) -> &Vec<T> {
// 		&self.inner
// 	}
// }

// // TODO: allocator param
// impl<T> AsMut<Vec<T>> for VecChain<T> {
// 	#[inline]
// 	fn as_mut(&mut self) -> &mut Vec<T> {
// 		&mut self.inner
// 	}
// }

// // TODO: allocator param
// impl<T> AsRef<[T]> for VecChain<T> {
// 	#[inline]
// 	fn as_ref(&self) -> &[T] {
// 		&self.inner
// 	}
// }

// // TODO: allocator param
// impl<T> AsMut<[T]> for VecChain<T> {
// 	#[inline]
// 	fn as_mut(&mut self) -> &mut [T] {
// 		&mut self.inner
// 	}
// }

// // TODO: allocator param
// impl<T> VecChain<T> {
// 	#[inline]
// 	pub fn as_vec(&self) -> &Vec<T> {
// 		&self.inner
// 	}

// 	#[inline]
// 	pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
// 		&mut self.inner
// 	}

// 	#[inline]
// 	pub fn into_inner(self) -> Vec<T> {
// 		self.inner
// 	}
// }
