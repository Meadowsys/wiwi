use crate::prelude_std::*;

use crate::num::{ FromU8 as _, IntoU8Lossy as _ };

const STRING_SIZE_BYTES: usize = size_of::<usize>() * 3;
const MAX_INLINE_LEN: usize = STRING_SIZE_BYTES - 1;
const CAP_MARKER_BE: usize = (!(usize::MAX >> 1)).to_be();
const CAP_MARKER_U8: u8 = !(u8::MAX >> 1);

const _: () = assert!(size_of::<StringInlineable>() == STRING_SIZE_BYTES);
const _: () = assert!(size_of::<StringInline>() == STRING_SIZE_BYTES);
const _: () = assert!(size_of::<StringHeap>() == STRING_SIZE_BYTES);

pub union StringInlineable {
	inline: ManuallyDrop<StringInline>,
	heap: ManuallyDrop<StringHeap>
}

impl StringInlineable {
	#[inline]
	pub const fn new() -> Self {
		let inline = ManuallyDrop::new(StringInline {
			len: 0,
			rest: MaybeUninit::uninit()
		});
		Self { inline }
	}
}

impl StringInlineable {
	#[inline]
	pub fn len(&self) -> usize {
		self.do_thing(|s| s.len(), |s| s.len())
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.do_thing(|s| s.capacity(), |s| s.capacity())
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl StringInlineable {
	#[inline(always)]
	fn is_inline(&self) -> bool {
		// SAFETY: all memory-valid instancees of `StringHeap` satisfy memory
		// invariants of `StringInline`, so union field access `self.inline` is fine
		let len = unsafe { self.inline.len };

		len & CAP_MARKER_U8 == 0
	}

	#[inline(always)]
	fn do_thing<T, FInline, FHeap>(&self, f_inline: FInline, f_heap: FHeap) -> T
	where
		FInline: FnOnce(&StringInline) -> T,
		FHeap: FnOnce(&StringHeap) -> T
	{
		match self.is_inline() {
			// SAFETY: we just checked `self.is_inline()`
			true => unsafe { f_inline(&self.inline) }
			// SAFETY: we just checked `self.is_inline()`
			false => unsafe { f_heap(&self.heap) }
		}
	}

	#[inline(always)]
	fn do_thing_mut<T, FInline, FHeap>(&mut self, f_inline: FInline, f_heap: FHeap) -> T
	where
		FInline: FnOnce(&mut StringInline) -> T,
		FHeap: FnOnce(&mut StringHeap) -> T
	{
		match self.is_inline() {
			// SAFETY: we just checked `self.is_inline()`
			true => unsafe { f_inline(&mut self.inline) }
			// SAFETY: we just checked `self.is_inline()`
			false => unsafe { f_heap(&mut self.heap) }
		}
	}
}

impl Default for StringInlineable {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl From<&str> for StringInlineable {
	fn from(value: &str) -> Self {
		match value.len() <= MAX_INLINE_LEN {
			true => {
				// TODO: move all this code into like, from_str_unsafe
				// function in StringInline
				let mut inline = ManuallyDrop::new(StringInline {
					len: 0,
					rest: MaybeUninit::uninit()
				});

				// SAFETY:
				// - ptr obtained from `value` is valid, and for `value.len()` reads
				// - ptr obtained from `inline.rest` is valid, and we checked
				//   `value.len()` to be lte its length (`MAX_INLINE_LEN`)
				// - ptrs obtained from aligned sources
				// - passed in reference to memory in `value` cannot overlap with
				//   function local stack memory in `inline`
				unsafe {
					ptr::copy_nonoverlapping(
						value.as_ptr(),
						inline.rest.as_mut_ptr().cast::<u8>(),
						value.len()
					)
				}

				// we just initialised `value.len()` amount of
				// memory with that `copy` call above
				inline.len = value.len().into_u8_lossy();

				Self { inline }
			}
			false => {
				todo!()
			}
		}
	}
}

#[repr(C)]
struct StringInline {
	/// regular u8, represented as is
	len: u8,
	rest: MaybeUninit<[u8; MAX_INLINE_LEN]>
}

impl StringInline {
	#[inline]
	fn len(&self) -> usize {
		usize::from_u8(self.len)
	}

	#[inline]
	fn capacity(&self) -> usize {
		MAX_INLINE_LEN
	}
}

#[repr(C)]
struct StringHeap {
	/// This value needs processing in order to be a valid capacity
	///
	/// This stores the capacity, in big endian, with the highest bit set. Just
	/// use [`capacity`](Self::capacity) function to get the capacity.
	cap_be_and_marker: usize,
	len: usize,
	ptr: *const u8
}

impl StringHeap {
	#[inline]
	fn len(&self) -> usize {
		self.len
	}

	#[inline]
	fn capacity(&self) -> usize {
		usize::from_be(self.cap_be_and_marker ^ CAP_MARKER_BE)
	}
}
