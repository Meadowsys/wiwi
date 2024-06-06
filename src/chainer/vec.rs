use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };
use std::slice;

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
		unsafe as_chunks_unchecked[const N: usize, CB](inner, cb: CB) where {
			CB: FnOnce(&[[T; N]])
		} => {
			let ptr = inner.as_ptr() as *const [T; N];
			let chunks = inner.len() / N;

			let slice = slice::from_raw_parts(ptr, chunks);
			cb(slice);
		}
	}
}
