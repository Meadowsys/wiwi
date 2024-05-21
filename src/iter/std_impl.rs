use std::{ slice, vec };
use std::marker::PhantomData;
use std::mem::{ ManuallyDrop, size_of };
use std::ptr::{ self, NonNull };
use super::{ IntoIter, Iter, SizeHint };

pub struct VecIntoIter<T> {
	/// - if ZST: this is a pointer to the start (vec.as_ptr()) and never changed
	/// - if not ZST: this ptr gets offset by 1 every iteration, so it points at the
	/// next element. We keep track of amount of remaining elements (in addition to
	/// capacity and (original) len) so we can offset it back to the original spot
	/// upon drop
	ptr: *const T,
	/// Vec capacity
	capacity: usize,
	/// Vec len at the time of `into_iter` call
	len: usize,
	/// Remaining elements to emit. Used in drop to unshift `ptr` for non-ZSTs
	remaining: usize,
	_marker: PhantomData<T>
}

impl<T> Iter for VecIntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		if self.remaining == 0 { return None }

		let ptr = self.ptr;
		if size_of::<T>() != 0 {
			self.ptr = unsafe { self.ptr.add(1) }
		}

		self.remaining -= 1;
		Some(unsafe { ptr.read() })
	}

	fn size_hint(&self) -> SizeHint {
		unsafe { SizeHint::hard_bound(self.remaining) }
	}
}

impl<T> Drop for VecIntoIter<T> {
	fn drop(&mut self) {
		let ptr = if size_of::<T>() == 0 || self.capacity == 0 {
			self.ptr as *mut T
		} else {
			unsafe {
				let consumed = self.len - self.remaining;
				let original_ptr = (self.ptr as *mut T).sub(consumed);

				// copy remaining elements to the front of ptr
				ptr::copy(self.ptr, original_ptr, self.remaining);

				original_ptr
			}
		};

		// This is probably the safest way to do this...
		// TODO: improve this with alloc api when it's stabilised?

		// ... let Vec handle the rest
		let _ = unsafe { Vec::from_raw_parts(ptr, self.remaining, self.capacity) };
	}
}

impl<T> IntoIter for Vec<T> {
	type Item = T;
	type Iter = VecIntoIter<T>;

	fn into_iter(self) -> VecIntoIter<T> {
		let me = ManuallyDrop::new(self);

		let ptr = me.as_ptr();
		let capacity = me.capacity();
		let len = me.len();
		let remaining = len;
		let _marker = PhantomData;

		VecIntoIter { ptr, capacity, len, remaining, _marker }
	}
}
