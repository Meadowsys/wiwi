use crate::iter::{ IntoIter, IntoStdIterator, IntoWiwiIter, IterAdapter };
// TODO: restore
// use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };
use super::{ chainer, chain_fn, new::{ ChainHalf, NonChainHalf } };
use std::{ ptr, vec };
use std::cmp::Ordering;
use std::mem::{ self, MaybeUninit };
use std::ops::RangeBounds;
use std::slice::{ self, SliceIndex };

chainer! {
	chainer: VecChain<T>
	inner: Vec<T>
}

impl<T> VecChain<T> {
	#[inline]
	pub fn new() -> Self {
		Vec::new().into()
	}

	#[inline]
	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		Vec::from_raw_parts(ptr, length, capacity).into()
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}
}

impl<T> VecChain<T> {
	// #[inline]
	// pub fn nonchain_as_ptr(&self) -> *const T {
	// 	self.inner.as_ptr()
	// }
	//
	// #[inline]
	// pub fn nonchain_as_ptr_mut(&mut self) -> *mut T {
	// 	self.inner.as_mut_ptr()
	// }

	// leak
}

// TODO: alloc related
// new_in
// with_capacity_in
// try_with_capacity_in
// from_raw_parts_in
// into_raw_parts_with_alloc
// allocator

impl<T> VecChain<T> {
	chain_fn! {
		/// Takes and moves all elements from another `VecChain` into `self`,
		/// leaving it empty.
		append(inner, other: &mut Self) => inner.append(&mut other.inner)
	}

	chain_fn! {
		/// Takes and moves all elements from a `Vec` into `self`,
		/// leaving it empty.
		append_nonchain(inner, other: &mut Vec<T>) => inner.append(other)
	}

	chain_fn! {
		as_chunks[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&[[T; N]], &[T])
		} => unsafe {
			let len = inner.len();
			let ptr = inner.as_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr as *const [T; N];
			let partial_ptr = ptr.add(len - partial);

			let full_chunk = slice::from_raw_parts(full_ptr, full);
			let partial_chunk = slice::from_raw_parts(partial_ptr, partial);

			cb(full_chunk, partial_chunk);
		}
	}

	chain_fn! {
		as_chunks_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&mut [[T; N]], &mut [T])
		} => unsafe {
			let len = inner.len();
			let ptr = inner.as_mut_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr as *mut [T; N];
			let partial_ptr = ptr.add(len - partial);

			let full_chunk = slice::from_raw_parts_mut(full_ptr, full);
			let partial_chunk = slice::from_raw_parts_mut(partial_ptr, partial);

			cb(full_chunk, partial_chunk);
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&[[T; N]])
		} => {
			let ptr = inner.as_ptr() as *const [T; N];
			let chunks = inner.len() / N;

			let slice = slice::from_raw_parts(ptr, chunks);
			cb(slice);
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&mut [[T; N]])
		} => {
			let ptr = inner.as_mut_ptr() as *mut [T; N];
			let chunks = inner.len() / N;

			let slice = slice::from_raw_parts_mut(ptr, chunks);
			cb(slice);
		}
	}

	chain_fn! {
		as_rchunks[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&[T], &[[T; N]])
		} => unsafe {
			let len = inner.len();
			let ptr = inner.as_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial) as *const [T; N];

			let partial_chunk = slice::from_raw_parts(ptr, partial);
			let full_chunk = slice::from_raw_parts(full_ptr, full);

			cb(partial_chunk, full_chunk);
		}
	}

	chain_fn! {
		as_rchunks_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&mut [T], &mut [[T; N]])
		} => unsafe {
			let len = inner.len();
			let ptr = inner.as_mut_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial) as *mut [T; N];

			let partial_chunk = slice::from_raw_parts_mut(ptr, partial);
			let full_chunk = slice::from_raw_parts_mut(full_ptr, full);

			cb(partial_chunk, full_chunk);
		}
	}

	chain_fn! {
		binary_search(inner, x: &T, out: &mut Result<usize, usize>) where {
			T: Ord
		} => *out = inner.binary_search(x)
	}

	chain_fn! {
		binary_search_uninit(inner, x: &T, out: &mut MaybeUninit<Result<usize, usize>>) where {
			T: Ord
		} => out.write(inner.binary_search(x))
	}

	chain_fn! {
		binary_search_by[F](inner, f: F, out: &mut Result<usize, usize>) where {
			F: FnMut(&T) -> Ordering
		} => *out = inner.binary_search_by(f)
	}

	chain_fn! {
		binary_search_by_uninit[F](inner, f: F, out: &mut MaybeUninit<Result<usize, usize>>) where {
			F: FnMut(&T) -> Ordering
		} => out.write(inner.binary_search_by(f))
	}

	chain_fn! {
		binary_search_by_key[B, F](inner, b: &B, f: F, out: &mut Result<usize, usize>) where {
			F: FnMut(&T) -> B,
			B: Ord
		} => *out = inner.binary_search_by_key(b, f)
	}

	chain_fn! {
		binary_search_by_key_uninit[B, F](inner, b: &B, f: F, out: &mut MaybeUninit<Result<usize, usize>>) where {
			F: FnMut(&T) -> B,
			B: Ord
		} => out.write(inner.binary_search_by_key(b, f))
	}

	chain_fn! {
		capacity(inner, out: &mut usize) => *out = inner.capacity()
	}

	chain_fn! {
		capacity_uninit(inner, out: &mut MaybeUninit<usize>) => out.write(inner.capacity())
	}

	chain_fn! {
		clear(inner) => inner.clear()
	}

	chain_fn! {
		clone_from_slice(inner, src: &[T]) where {
			T: Clone
		} => inner.clone_from_slice(src)
	}

	chain_fn! {
		contains(inner, x: &T, out: &mut bool) where {
			T: PartialEq
		} => *out = inner.contains(x)
	}

	chain_fn! {
		contains_uninit(inner, x: &T, out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => out.write(inner.contains(x))
	}

	chain_fn! {
		copy_from_slice(inner, src: &[T]) where {
			T: Copy
		} => inner.copy_from_slice(src)
	}

	chain_fn! {
		copy_within[R](inner, src: R, dest: usize) where {
			R: RangeBounds<usize>,
			T: Copy
		} => inner.copy_within(src, dest)
	}

	chain_fn! {
		dedup(inner) where {
			T: PartialOrd
		} => inner.dedup()
	}

	chain_fn! {
		dedup_by[F](inner, mut same_bucket: F) where {
			F: FnMut(&T, &T) -> bool
		} => inner.dedup_by(move |a, b| same_bucket(a, b))
	}

	chain_fn! {
		dedup_by_mut[F](inner, same_bucket: F) where {
			F: FnMut(&mut T, &mut T) -> bool
		} => inner.dedup_by(same_bucket)
	}

	chain_fn! {
		dedup_by_key[F, K](inner, mut key: F) where {
			F: FnMut(&T) -> K,
			K: PartialEq
		} => inner.dedup_by_key(|v| key(v))
	}

	chain_fn! {
		dedup_by_key_mut[F, K](inner, mut key: F) where {
			F: FnMut(&mut T) -> K,
			K: PartialEq
		} => inner.dedup_by_key(key)
	}

	// TODO: drain

	chain_fn! {
		ends_with(inner, needle: &[T], out: &mut bool) where {
			T: PartialEq
		} => *out = inner.ends_with(needle)
	}

	chain_fn! {
		ends_with_uninit(inner, needle: &[T], out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => out.write(inner.ends_with(needle))
	}

	chain_fn! {
		extend_from_slice(inner, other: &[T]) where {
			T: Clone
		} => inner.extend_from_slice(other)
	}

	chain_fn! {
		extend_from_within[R](inner, src: R) where {
			T: Clone,
			R: RangeBounds<usize>
		} => inner.extend_from_within(src)
	}

	// extract_if

	chain_fn! {
		fill(inner, value: T) where {
			T: Clone
		} => inner.fill(value)
	}

	chain_fn! {
		fill_with[F](inner, f: F) where {
			F: FnMut() -> T
		} => inner.fill_with(f)
	}

	chain_fn! {
		first[CB](inner, cb: CB) where {
			CB: FnOnce(Option<&T>)
		} => cb(inner.first())
	}

	chain_fn! {
		first_mut[CB](inner, cb: CB) where {
			CB: FnOnce(Option<&mut T>)
		} => cb(inner.first_mut())
	}

	chain_fn! {
		first_chunk[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<&[T; N]>)
		} => cb(inner.first_chunk())
	}

	chain_fn! {
		first_chunk_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<&mut [T; N]>)
		} => cb(inner.first_chunk_mut())
	}

	chain_fn! {
		get[I, CB](inner, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			CB: FnOnce(Option<&I::Output>)
		} => cb(inner.get(index))
	}

	chain_fn! {
		get_mut[I, CB](inner, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			CB: FnOnce(Option<&mut I::Output>)
		} => cb(inner.get_mut(index))
	}

	chain_fn! {
		unsafe get_unchecked[I, CB](inner, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			CB: FnOnce(&I::Output)
		} => cb(inner.get_unchecked(index))
	}

	chain_fn! {
		unsafe get_unchecked_mut[I, CB](inner, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			CB: FnOnce(&mut I::Output)
		} => cb(inner.get_unchecked_mut(index))
	}

	chain_fn! {
		insert(inner, index: usize, element: T) => inner.insert(index, element)
	}

	chain_fn! {
		is_empty(inner, out: &mut bool) => *out = inner.is_empty()
	}

	chain_fn! {
		is_empty_uninit(inner, out: &mut MaybeUninit<bool>) => out.write(inner.is_empty())
	}

	chain_fn! {
		last[CB](inner, cb: CB) where {
			CB: FnOnce(Option<&T>)
		} => cb(inner.last())
	}

	chain_fn! {
		last_mut[CB](inner, cb: CB) where {
			CB: FnOnce(Option<&mut T>)
		} => cb(inner.last_mut())
	}

	chain_fn! {
		last_chunk[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<&[T; N]>)
		} => cb(inner.last_chunk())
	}

	chain_fn! {
		last_chunk_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<&mut [T; N]>)
		} => cb(inner.last_chunk_mut())
	}

	chain_fn! {
		len(inner, out: &mut usize) => *out = inner.len()
	}

	chain_fn! {
		len_uninit(inner, out: &mut MaybeUninit<usize>) => out.write(inner.len())
	}

	chain_fn! {
		pop(inner, out: &mut Option<T>) => *out = inner.pop()
	}

	chain_fn! {
		pop_uninit(inner, out: &mut MaybeUninit<Option<T>>) => out.write(inner.pop())
	}

	// TODO: pop_if

	chain_fn! {
		push(inner, value: T) => inner.push(value)
	}

	// TODO: push_within_capacity

	chain_fn! {
		remove(inner, index: usize, out: &mut T) => *out = inner.remove(index)
	}

	chain_fn! {
		remove_uninit(inner, index: usize, out: &mut MaybeUninit<T>) => out.write(inner.remove(index))
	}

	chain_fn! {
		// TODO: ...this can be more efficient (done in place?)
		move repeat(inner, n: usize) where {
			T: Copy
		} => inner.repeat(n)
	}

	chain_fn! {
		reserve(inner, additional: usize) => inner.reserve(additional)
	}

	chain_fn! {
		reserve_exact(inner, additional: usize) => inner.reserve_exact(additional)
	}

	chain_fn! {
		resize(inner, new_len: usize, value: T) where {
			T: Clone
		} => inner.resize(new_len, value)
	}

	chain_fn! {
		resize_with[F](inner, new_len: usize, f: F) where {
			F: FnMut() -> T
		} => inner.resize_with(new_len, f)
	}

	chain_fn! {
		retain[F](inner, f: F) where {
			F: FnMut(&T) -> bool
		} => inner.retain(f)
	}

	chain_fn! {
		retain_mut[F](inner, f: F) where {
			F: FnMut(&mut T) -> bool
		} => inner.retain_mut(f)
	}

	chain_fn! {
		reverse(inner) => inner.reverse()
	}

	chain_fn! {
		unsafe set_len(inner, new_len: usize) => inner.set_len(new_len)
	}

	chain_fn! {
		shrink_to(inner, min_capacity: usize) => inner.shrink_to(min_capacity)
	}

	chain_fn! {
		shrink_to_fit(inner) => inner.shrink_to_fit()
	}

	chain_fn! {
		sort(inner) where {
			T: Ord
		} => inner.sort()
	}

	chain_fn! {
		sort_by[F](inner, compare: F) where {
			F: FnMut(&T, &T) -> Ordering
		} => inner.sort_by(compare)
	}

	chain_fn! {
		sort_by_cached_key[K, F](inner, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => inner.sort_by_cached_key(f)
	}

	chain_fn! {
		sort_by_key[K, F](inner, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => inner.sort_by_key(f)
	}

	chain_fn! {
		sort_unstable(inner) where {
			T: Ord
		} => inner.sort_unstable()
	}

	chain_fn! {
		sort_unstable_by[F](inner, compare: F) where {
			F: FnMut(&T, &T) -> Ordering
		} => inner.sort_unstable_by(compare)
	}

	chain_fn! {
		sort_unstable_by_key[K, F](inner, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => inner.sort_unstable_by_key(f)
	}

	chain_fn! {
		splice[R, I, CB](inner, range: R, replace_with: I, cb: CB) where {
			R: RangeBounds<usize>,
			I: IntoIter<Item = T>,
			CB: FnOnce(vec::Splice<IterAdapter<I::Iter>>)
		} => cb(inner.splice(range, replace_with.convert_wiwi_into_std_iterator()))
	}

	chain_fn! {
		split_at_spare_mut[CB](inner, cb: CB) where {
			CB: FnOnce(&mut [T], &mut [MaybeUninit<T>])
		} => unsafe {
			// TODO: use std impl when its stabilised
			let ptr = inner.as_mut_ptr();
			let len = inner.len();
			let cap = inner.capacity();

			let spare_ptr = ptr.add(len) as *mut MaybeUninit<T>;
			let spare_len = cap - len;

			let init = slice::from_raw_parts_mut(ptr, len);
			let spare = slice::from_raw_parts_mut(spare_ptr, spare_len);

			cb(init, spare);
		}
	}

	chain_fn! {
		split_first[CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&T, &[T])>)
		} => cb(inner.split_first())
	}

	chain_fn! {
		split_first_mut[CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&mut T, &mut [T])>)
		} => cb(inner.split_first_mut())
	}

	chain_fn! {
		split_first_chunk[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&[T; N], &[T])>)
		} => cb(inner.split_first_chunk())
	}

	chain_fn! {
		split_first_chunk_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&mut [T; N], &mut [T])>)
		} => cb(inner.split_first_chunk_mut())
	}

	chain_fn! {
		split_last[CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&T, &[T])>)
		} => cb(inner.split_last())
	}

	chain_fn! {
		split_last_mut[CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&mut T, &mut [T])>)
		} => cb(inner.split_last_mut())
	}

	chain_fn! {
		split_last_chunk[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&[T], &[T; N])>)
		} => cb(inner.split_last_chunk())
	}

	chain_fn! {
		split_last_chunk_mut[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(Option<(&mut [T], &mut [T; N])>)
		} => cb(inner.split_last_chunk_mut())
	}

	// TODO: split_off

	chain_fn! {
		starts_with(inner, needle: &[T], out: &mut bool) where {
			T: PartialEq
		} => *out = inner.starts_with(needle)
	}

	chain_fn! {
		starts_with_uninit(inner, needle: &[T], out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => out.write(inner.starts_with(needle))
	}

	chain_fn! {
		swap(inner, a: usize, b: usize) => inner.swap(a, b)
	}

	chain_fn! {
		unsafe swap_unchecked(inner, a: usize, b: usize) => {
			// TODO: replace with std impl once stabilised
			let ptr = inner.as_mut_ptr();
			ptr::swap(ptr.add(a), ptr.add(b));
		}
	}

	chain_fn! {
		swap_remove(inner, index: usize, out: &mut T) => *out = inner.swap_remove(index)
	}

	chain_fn! {
		swap_remove_uninit(inner, index: usize, out: &mut MaybeUninit<T>) => out.write(inner.swap_remove(index))
	}

	chain_fn! {
		swap_with_slice(inner, other: &mut [T]) => inner.swap_with_slice(other)
	}

	chain_fn! {
		truncate(inner, len: usize) => inner.truncate(len)
	}

	// TODO: try_reserve/exact

	chain_fn! {
		spare_capacity_mut[CB](inner, cb: CB) where {
			CB: FnOnce(&mut [MaybeUninit<T>])
		} => cb(inner.spare_capacity_mut())
	}

	chain_fn! {
		windows[CB](inner, size: usize, cb: CB) where {
			CB: FnOnce(IterAdapter<slice::Windows<T>>)
		} => cb(inner.windows(size).convert_std_into_wiwi_iter())
	}


	// TODO: utf8_chunks
	// TODO: eq_ignore_ascii_case
	// TODO: escape_ascii
	// TODO: trim_ascii_start/end
	// TODO: trim_ascii

	// doc link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.first_chunk
	// TODO: iter/mut

	// TODO: chunks/mut
	// TODO: chunks_exact/mut
	// TODO: array_chunks
	// TODO: array_chunks_mut
	// TODO: nightly array_windows
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
	// TODO: starts/ends_with
	// TODO: strip_prefix/suffix
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: rotate_left/right
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
	// TODO: flatten/mut
	// TODO: as_str
	// TODO: as_bytes
	// TODO: to_ascii_uppercase/lowercase
	// TODO: to_vec/in
	// TODO: repeat
	// TODO: concat
	// TODO: join

	// TODO: nightly array_chunks/mut

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
	// TODO: split_once
	// TODO: rsplit_once
	// TODO: strip_prefix/suffix
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: rotate_left/right
	// TODO: clone_within (not in std)?
	// TODO: align_to/mut
	// TODO: nightly as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: get_many/get_many_unchecked (non mut? not in std?)
	// TODO: eq_ignore_ascii_case
	// TODO: make_ascii_uppercase/lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii
	// TODO: trim_ascii_start/end
}

impl VecChain<f32> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(inner) => inner.sort_unstable_by(f32::total_cmp)
	}
}

impl VecChain<f64> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(inner) => inner.sort_unstable_by(f64::total_cmp)
	}
}

impl VecChain<u8> {
	chain_fn! {
		is_ascii(inner, out: &mut bool) => *out = inner.is_ascii()
	}

	chain_fn! {
		is_ascii_uninit(inner, out: &mut MaybeUninit<bool>) => out.write(inner.is_ascii())
	}

	chain_fn! {
		make_ascii_lowercase(inner) => inner.make_ascii_lowercase()
	}

	chain_fn! {
		make_ascii_uppercase(inner) => inner.make_ascii_uppercase()
	}

	// TODO: as_ascii/unchecked nightly
}

impl<T, const N: usize> VecChain<[T; N]> {
	pub fn flatten(mut self) -> VecChain<T> {
		let (len, cap) = if mem::size_of::<T>() == 0 {
			let len = self.inner.len()
				.checked_mul(N)
				.expect("vec len overflow");
			(len, usize::MAX)
		} else {
			// TODO: use unchecked mul when rust 1.79
			(
				self.inner.len() * N,
				self.inner.capacity() * N
			)
		};

		let ptr = self.inner.as_mut_ptr() as *mut T;
		mem::forget(self);

		unsafe { Vec::from_raw_parts(ptr, len, cap) }.into()
	}
}

impl<T> VecChain<T> {
	chain_fn! {
		/// Sorts, then dedups, the vector chain.
		///
		/// Nonstandard API, suggested by my good friend
		/// [Silk Rose] c:
		///
		/// This works exactly the same as `chain.sort().dedup()`.
		///
		/// # Examples
		///
		/// TODO
		///
		/// [Silk Rose]: https://github.com/silkrose
		move self sort_and_dedup(self) where {
			T: Ord
		} => self.sort().dedup()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn creation() {
		let mut new_len = 0;
		let mut new_capacity = 0;
		let mut with_capacity_len = 0;
		let mut with_capacity_capacity = 0;
		let mut zst_len = 0;
		let mut zst_capacity = 0;

		let _ = VecChain::<u8>::new()
			.len(&mut new_len)
			.capacity(&mut new_capacity);

		let _ = VecChain::<u8>::with_capacity(30)
			.len(&mut with_capacity_len)
			.capacity(&mut with_capacity_capacity);

		let _ = VecChain::<()>::with_capacity(30)
			.len(&mut zst_len)
			.capacity(&mut zst_capacity);

		assert_eq!(new_len, 0);
		assert_eq!(new_capacity, 0);
		assert_eq!(with_capacity_len, 0);
		assert_eq!(with_capacity_capacity, 30);
		assert_eq!(zst_len, 0);
		assert_eq!(zst_capacity, usize::MAX);
	}

	#[test]
	fn conversion() {
		let slice = &[1u8, 2, 3, 4, 5] as &[_];
		let mut chain = VecChain::new()
			.extend_from_slice(slice);

		assert_eq!(slice, chain.as_nonchain());
		assert_eq!(slice, chain.as_nonchain_mut());
	}

	#[test]
	fn len_and_capacity() {
		let mut len1 = 0;
		let mut cap1 = 0;
		let mut len2 = 0;
		let mut cap2 = 0;
		let mut is_empty = true;
		let mut is_empty_new = false;
		let mut is_empty_with_cap = false;

		let _ = VecChain::with_capacity(8)
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.len(&mut len1)
			.capacity(&mut cap1)
			.push(9)
			.push(10)
			.len(&mut len2)
			.capacity(&mut cap2)
			.is_empty(&mut is_empty);
		let _ = VecChain::<u8>::new()
			.is_empty(&mut is_empty_new);
		let _ = VecChain::<u8>::with_capacity(8)
			.is_empty(&mut is_empty_with_cap);


		assert_eq!(len1, 8);
		assert!(cap1 >= 8);

		assert_eq!(len2, 10);
		assert!(cap2 >= 10);

		assert!(!is_empty);
		assert!(is_empty_new);
		assert!(is_empty_with_cap);
	}

	#[test]
	fn with_split_at_spare_mut() {
		let mut uninit_len = 0;
		let chain = VecChain::new()
			.extend_from_slice(&[1usize, 2, 3, 4, 5, 6, 7, 8])
			.reserve(8)
			.split_at_spare_mut(|mut slice, mut uninit| {
				uninit_len = uninit.len();

				assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8]);
				assert!(uninit.len() >= 8);

				uninit.iter_mut()
					.enumerate()
					.take(4)
					.for_each(|(i, slot)| {
						slot.write(i);
					});
			});

		unsafe {
			let mut len = 0;
			let _ = chain
				.len(&mut len)
				.set_len(len + 4)
				.split_at_spare_mut(|mut slice, mut uninit| {
					assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3]);
					assert_eq!(uninit_len - 4, uninit.len());
				});
		}
	}

	#[test]
	fn swap_unchecked() {
		unsafe {
			let chain = VecChain::new()
				.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
				.swap_unchecked(4, 6)
				.swap_unchecked(0, 3)
				.swap_unchecked(1, 6)
				.swap_unchecked(6, 7)
				.swap_unchecked(2, 6);
			assert_eq!(chain.as_nonchain(), &[4, 5, 8, 1, 7, 6, 3, 2]);
		}
	}

	#[test]
	fn reverse() {
		let chain = VecChain::new()
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.reverse();
		assert_eq!(chain.as_nonchain(), &[8, 7, 6, 5, 4, 3, 2, 1]);
	}

	#[test]
	fn with_chunks() {
		const N: usize = 5;

		let slice = b"1234";

		fn check<'h>(
			expected_chunks: &[&[u8; N]],
			expected_remainder: &'h [u8]
		) -> impl FnOnce(&[[u8; N]], &[u8]) + 'h {
			let expected_chunks = expected_chunks
				.into_iter()
				.map(|item| **item)
				.collect::<Vec<_>>();

			move |chunks, rem| {
				assert_eq!(expected_chunks.len(), chunks.len(), "wrong num of chunks");
				assert_eq!(expected_remainder.len(), rem.len(), "wrong num of elements in remainder");

				assert_eq!(expected_chunks, chunks);
				assert_eq!(expected_remainder, rem);
			}
		}

		let _ = VecChain::with_capacity(20)
			.extend_from_slice(slice)
			.as_chunks(check(&[], b"1234"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341"], b"234"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412"], b"34"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412", b"34123"], b"4"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412", b"34123", b"41234"], b""));
	}
}
