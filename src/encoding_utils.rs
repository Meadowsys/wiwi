use std::{ slice, ptr };

/// In debug mode, keeps track of the amount of bytes written, and asserts
/// preconditions like not writing over capacity and having all preallocated
/// capacity filled. However, in release mode, its just a wrapper around a vec,
/// its ptr, raw ptr operations ([`ptr::copy_nonoverlapping`] etc), and a method
/// that unsafetly sets the len of the vec before unwrapping it.
// Note. if this is dropped prematurely, the included vec will be dropped per its
// regular drop rules, and `u8` doesn't have any special drop requirements, so
// this is correct and won't resource leak.
pub struct UnsafeBufWriteGuard {
	vec: Vec<u8>,
	ptr: *mut u8,
	requested_capacity: usize,
	#[cfg(debug_assertions)]
	bytes_written: usize
}

impl UnsafeBufWriteGuard {
	/// Create new [`UnsafeBufWriteGuard`] with specified capacity. The amount of
	/// capacity specified must be _exactly_ calculated, and _all_ capacity allocated
	/// here _must_ be used up before calling [`into_full_vec`][Self::into_full_vec].
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self {
		let mut vec = Vec::with_capacity(capacity);
		let ptr = vec.as_mut_ptr();

		debug_assert!(vec.capacity() >= capacity);

		Self {
			vec,
			ptr,
			requested_capacity: capacity,
			#[cfg(debug_assertions)]
			bytes_written: 0
		}
	}

	/// Writes an amount of bytes into self. Does the same as [`write_bytes`][Self::write_bytes]
	/// functionality-wise, but maybe the constant param N will enable more
	/// optimisations?
	///
	/// # Safety
	///
	/// You must not write over the amount of capacity that you preallocated.
	#[inline(always)]
	pub unsafe fn write_bytes_const<const N: usize>(&mut self, src: *const u8) {
		#[cfg(debug_assertions)] {
			self.bytes_written += N;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		ptr::copy_nonoverlapping(src, self.ptr, N);
		self.ptr = self.ptr.add(N);
	}

	/// Writes an amount of bytes into self.
	///
	/// # Safety
	///
	/// You must not write over the amount of capacity that you preallocated.
	#[inline(always)]
	pub unsafe fn write_bytes(&mut self, src: *const u8, n: usize) {
		#[cfg(debug_assertions)] {
			self.bytes_written += n;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		ptr::copy_nonoverlapping(src, self.ptr, n);
		self.ptr = self.ptr.add(n);
	}

	/// Make sure to also call `add_byte_count` function afterwards, to keep
	/// proper track of the ptr inside. Calling this will yield a pointer
	/// pointing at the start of the uninitialised chunk (ie. no need to
	/// keep track of how many bytes have been written, and no need to call
	/// `.add()` on the returned pointer.) It is necessary to call
	/// [`add_byte_count`][Self::add_byte_count] after this, to make sure the pointer
	/// stored internally is correct.
	///
	/// # Safety
	///
	/// As with the rest of the write functions, you must not write over the
	/// amount of capacity that you preallocated. Additionally, you must call
	/// add_byte_count.
	#[inline(always)]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
		self.ptr
	}

	/// Call this after writing to the pointer returned by [`as_mut_ptr`][Self::as_mut_ptr],
	/// ensuring the pointer stored internally is still going to point at the
	/// right spot (start of uninitialised chunk).
	///
	/// # Safety
	///
	/// You must have written to the amount of bytes that you are giving, otherwise
	/// there will be uninitialised memory "holes" in the resulting vec.
	#[inline(always)]
	pub unsafe fn add_byte_count(&mut self, n: usize) {
		#[cfg(debug_assertions)] {
			self.bytes_written += n;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		self.ptr = self.ptr.add(n);
	}

	/// Returns the internal vec.
	///
	/// # Safety
	///
	/// By calling this, you assert that you have written to all the
	/// bytes that you preallocated when creating this. If this still has
	/// uninitialised bytes left at the end, the length will be set too far,
	/// violating contract of [`Vec::set_len`].
	#[inline(always)]
	pub unsafe fn into_full_vec(mut self) -> Vec<u8> {
		debug_assert!(self.bytes_written == self.requested_capacity);

		self.vec.set_len(self.requested_capacity);
		self.vec
	}
}

/// Utility to emit fixed size (const) chunks, in an unchecked manner, from
/// a slice. Contains assertions to assert preconditions in debug mode.
#[repr(transparent)]
pub struct ChunkedSlice<'h, const N: usize> {
	bytes: &'h [u8]
}

impl<'h, const N: usize> ChunkedSlice<'h, N> {
	#[inline(always)]
	pub fn new(bytes: &'h [u8]) -> Self {
		Self { bytes }
	}

	/// Takes N bytes off the front of the internal slice, returning that slice,
	/// and saving the rest for future calls.
	///
	/// # Safety
	///
	/// `self.bytes` must have `N` or more bytes left in it,
	/// otherwise invalid memory will be read from.
	pub unsafe fn next_frame_unchecked(&mut self) -> &'h [u8; N] {
		debug_assert!(self.bytes.len() >= N, "enough bytes left to form another whole frame");

		let self_ptr = self.bytes as *const [u8] as *const u8;
		let self_len = self.bytes.len();

		// SAFETY: this is the fixed size slice that is returned. Caller asserts
		// that self contains at least N bytes (and so the ptr created from self
		// will be safe to read from).
		let new_slice = &*(self_ptr as *const [u8; N]);

		// SAFETY: see function doc comment. Caller asserts self has at least N bytes.
		// `self_len - N` and `self_ptr.add(N)` is safe becaue we have at least N
		// bytes, and is correct because we just took N bytes out above, and are
		// returning reference to it.
		self.bytes = slice::from_raw_parts(self_ptr.add(N), self_len - N);

		new_slice
	}

	/// Consumes self, takes the remainder slice, copies it into a temporary
	/// buffer of length `N`, and calls the provided function with this buffer.
	/// Returns the amount of bytes in that buffer that aren't padding (ie. the
	/// amount of bytes that are actual data bytes).
	///
	/// # Safety
	///
	/// `self.bytes` must have N or less bytes left in it, otherwise invalid
	/// memory (at the end of the temporary buffer created) will be written to.
	pub unsafe fn with_remainder_unchecked<F>(self, mut f: F)
	where
		F: FnMut(&[u8; N])
	{
		let len = self.bytes.len();

		debug_assert!(len < N, "(strictly) less than a whole frame remaining");

		// temp buffer of correct length, to add padding
		let mut slice = [0u8; N];

		// ptr to self
		let self_ptr = self.bytes as *const [u8] as *const u8;
		// ptr to temp buffer
		let slice_ptr = &mut slice as *mut [u8] as *mut u8;

		// SAFETY: slice in self has less than N bytes remaining as guaranteed by
		// caller; therefore, the amount of bytes copied will be the correct
		// amount, and always fit in the temp buffer. We also read `len` from self,
		// meaning we can always read `len` bytes from self.
		ptr::copy_nonoverlapping(self_ptr, slice_ptr, len);

		f(&slice);
	}

	/// If debug assertions are enabled, this asserts that the slice contained in
	/// `self` is empty (ie. len 0), and panics if not. Otherwise, this does nothing.
	#[inline(always)]
	pub fn debug_assert_is_empty(&self) {
		debug_assert!(self.bytes.is_empty(), "all bytes were consumed");
	}
}
