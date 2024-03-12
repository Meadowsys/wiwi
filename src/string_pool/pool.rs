use ::std::hash::{ Hash, Hasher };

mod global;

pub use global::GlobalPool;

pub trait Pool: Clone {
	type Raw;

	// --- required functions ---

	/// # Safety
	///
	/// The provided slices, when joined together using [`SlicesWrap::to_boxed_slice`],
	/// must be valid UTF-8. Strictly speaking individual slices don't have to be
	/// valid UTF-8, as long as the whole is, so this should _not_ be relied on,
	/// even though it is quite likely that they will be.
	unsafe fn raw_from_slices(&self, slices: SlicesWrap) -> Self::Raw;

	fn raw_to_slice<'r>(&self, raw: &'r Self::Raw) -> &'r [u8];

	// --- can-be-optimised functions ---
	// these can be overridden if it can create a more efficient implementation

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	///
	/// # Safety
	///
	/// The provided slice must be valid UTF-8.
	unsafe fn raw_from_slice(&self, slice: &[u8]) -> Self::Raw {
		self.raw_from_slices(SlicesWrap(&[slice]))
	}

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	///
	/// # Safety
	///
	/// The provided vec must be valid UTF-8.
	unsafe fn raw_from_vec(&self, vec: Vec<u8>) -> Self::Raw {
		self.raw_from_slice(&vec)
	}

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	fn raw_empty(&self) -> Self::Raw {
		unsafe { self.raw_from_slice(&[]) }
	}

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	fn raw_into_vec(&self, raw: Self::Raw) -> Vec<u8> {
		self.raw_to_slice(&raw).to_vec()
	}

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	fn raw_into_boxed_slice(&self, raw: Self::Raw) -> Box<[u8]> {
		self.raw_into_vec(raw).into_boxed_slice()
	}

	/// note to implementors: The default implementation
	/// of this function is usually enough; however this can be overridden
	/// for optimisation reasons.
	fn raw_clone(&self, raw: &Self::Raw) -> Self::Raw {
		let slice = self.raw_to_slice(raw);
		unsafe { self.raw_from_slice(slice) }
	}
}

/// Wraps a slice of slices of bytes. This has a simple Hash and Eq implementation
/// that just repeatedly hashes/checks every byte on every u8 in sequence, to ensure
/// that no matter how many slices there are, as long as they have the same byte
/// sequence, they will have the same resulting hash and compare equal. This way,
/// we don't need to copy all the bytes into a new allocation until the absolute
/// last step.
#[derive(Debug)]
#[repr(transparent)]
pub struct SlicesWrap<'h>(pub &'h [&'h [u8]]);

impl<'h> SlicesWrap<'h> {
	pub fn to_vec(&self) -> Vec<u8> {
		self.into_iter().collect()
	}

	/// Joins the slices in sequence, returning a boxed slice.
	pub fn to_boxed_slice(&self) -> Box<[u8]> {
		self.to_vec().into_boxed_slice()
	}
}

impl<'h> Hash for SlicesWrap<'h> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.into_iter().for_each(|b| state.write_u8(b));
	}
}

impl<'h> IntoIterator for &SlicesWrap<'h> {
	type Item = <SlicesWrapIter<'h> as Iterator>::Item;
	type IntoIter = SlicesWrapIter<'h>;

	fn into_iter(self) -> Self::IntoIter {
		let mut vec = Vec::with_capacity(self.0.len());
		self.0.iter().rev().for_each(|slice| vec.push(*slice));
		SlicesWrapIter(vec)
	}
}

/// Iterator for [`SlicesWrap`] that returns elements in one slice after the
/// other in sequence. The sequence returned is identical to if you were to zip
/// all the slice iters to each other.
/// Iterating through the boxed slice returned by [`SlicesWrap::to_boxed_slice`]
/// would also yield the same sequence as if you were to iterate through this iterator.
pub struct SlicesWrapIter<'h>(Vec<&'h [u8]>);

impl<'h> Iterator for SlicesWrapIter<'h> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		// if this is None
		// we will have reached the end of the vec
		let next_slice = self.0.pop()?;

		match next_slice.iter().next() {
			Some(item) => {
				self.0.push(&next_slice[1..]);
				Some(*item)
			}
			None => {
				// we popped so this will start on the slice after current
				self.next()
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.0.iter().map(|s| s.len()).sum();
		(len, Some(len))
	}
}
