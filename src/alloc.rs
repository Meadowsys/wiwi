pub use crate::rust_std::debug_assert;
pub use crate::rust_std::alloc::{ GlobalAlloc, Layout };
pub use crate::rust_std::option::{ Option, Option::Some, Option::None };
pub use crate::rust_std::ptr::NonNull;

pub trait Allocator {
	fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>;

	#[expect(clippy::missing_safety_doc)]
	unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout);

	#[inline]
	fn alloc_zeroed(&self, layout: Layout) -> Option<NonNull<u8>> {
		let ptr = self.alloc(layout)?;
		// SAFETY: `alloc` returns a valid ptr to a block of memory specified by `layout`
		unsafe { ptr.as_ptr().write_bytes(0, layout.size()) }
		Some(ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() <= new_layout.size());

		let new_ptr = self.alloc(new_layout)?;

		// SAFETY: caller promises that `ptr` is valid, `old_layout` fits `ptr`,
		// `old_layout` is smaller than  `new_layout`, and `alloc` returns a valid ptr
		unsafe { new_ptr.copy_from_nonoverlapping(ptr, old_layout.size()) }

		// SAFETY: caller promises `ptr` is valid and `old_layout` fits `ptr`
		unsafe { self.dealloc(ptr, old_layout) }

		Some(new_ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		// SAFETY: caller promises to uphold promises of `grow`
		let new_ptr = unsafe { self.grow(ptr, old_layout, new_layout)? };

		// SAFETY: `grow` returns valid block of memory, and `new_layout` is larger than `old_layout`
		let new_region = unsafe { new_ptr.add(old_layout.size()) };

		// SAFETY: this is the region that's from the end of the old len to the
		// end of the new len, valid for writes
		unsafe { new_region.write_bytes(0, new_layout.size() - old_layout.size()) }

		Some(new_ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() >= new_layout.size());
		let new_ptr = self.alloc(new_layout)?;

		// SAFETY: `alloc` returns a valid ptr to `new_layout`, which caller
		// promises is smaller than `old_layout`
		unsafe { new_ptr.copy_from_nonoverlapping(ptr, new_layout.size()) }

		// SAFETY: caller promises that `ptr` fits `old_layout`
		unsafe { self.dealloc(ptr, old_layout) }

		Some(new_ptr)
	}
}
