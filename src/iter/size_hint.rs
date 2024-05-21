pub struct SizeHint {
	lower: SizeHintBound,
	upper: SizeHintBound
}

/// One side of the bound of size hint.
///
/// This struct is only useful either when used internally by [`SizeHint`] or
/// returned by [`SizeHint`] for consumers of an iter. You can construct instances
/// of this, but they're kinda useless on their own. [`SizeHint`] has no methods
/// that take instances of this enum directly.
pub enum SizeHintBound {
	/// Hard bound, unsafe, reliable information for use in unsafe operations.
	///
	/// When this is returned as the lower bound, the iter promises to _always_ return
	/// _at least_ this many items.
	///
	/// When this is returned as the upper bound, the iter promises to _always_ return
	/// _at most_ this many items.
	///
	/// Since it is allowed for unsafe code to rely on this value, if the iter
	/// does not uphold its promise, it is undefined behaviour.
	///
	/// If the returned value is `usize::MAX`, the iter thinks it will return at
	/// least / at most `usize::MAX` elements. The exact number has overflown and
	/// that information has been lost. With this value, the bound is
	/// automatically relaxed to an estimate, meaning unsafe code now _cannot_
	/// rely on this value for safety/correctness.
	///
	/// # Rust std iterators
	///
	/// It is _never_ correct to return this for iters adapted from std's
	/// iterator type, as std's `size_hint` does not provide such hard guarantees.
	HardBound { bound: usize },

	/// Only an estimation (unreliable for unsafe operations).
	///
	/// The most that this can be relied on is for optimisations. For example, a
	/// vector may use this to preallocate values. The iter is allowed to not
	/// follow the value it returns.
	///
	/// When this is returned as the lower bound, the iter says it will likely
	/// return at least this many items.
	///
	/// When this is returned as the upper bound, the iter says it will likely
	/// return at most this many items.
	///
	/// Unsafe code _must not_ rely on this value for safety/correctness.
	///
	/// If the returned value is `usize::MAX`, the iter thinks it will return at
	/// least / at most `usize::MAX` elements. The exact number has overflown and
	/// that information has been lost.
	///
	/// # Rust std iterators
	///
	/// You likely want this or [`Unknown`](SizeHintBound::Unknown) when adapting
	/// an std iterator's size hint.
	Estimate { estimate: usize },

	/// Unknown bound. Anything goes :p
	Unknown
}

impl SizeHint {
	pub fn new() -> Self {
		Self::unknown()
	}

	pub fn into_std_hint(self) -> (usize, Option<usize>) {
		use SizeHintBound::*;

		let lower = match self.lower {
			HardBound { bound: val } | Estimate { estimate: val } => { val }
			Unknown => { 0 }
		};

		let upper = match self.upper {
			HardBound { bound: val } | Estimate { estimate: val } => { Some(val) }
			Unknown => { None }
		};

		(lower, upper)
	}

	pub fn unknown() -> Self {
		Self {
			lower: SizeHintBound::Unknown,
			upper: SizeHintBound::Unknown
		}
	}

	pub fn with_lower_unknown(mut self) -> Self {
		self.lower = SizeHintBound::Unknown;
		self
	}

	pub fn with_upper_unknown(mut self) -> Self {
		self.upper = SizeHintBound::Unknown;
		self
	}

	pub fn estimate(estimate: usize) -> Self {
		Self {
			lower: SizeHintBound::Estimate { estimate },
			upper: SizeHintBound::Estimate { estimate }
		}
	}

	pub fn with_lower_estimate(mut self, estimate: usize) -> Self {
		self.lower = SizeHintBound::Estimate { estimate };
		self
	}

	pub fn with_upper_estimate(mut self, estimate: usize) -> Self {
		self.upper = SizeHintBound::Estimate { estimate };
		self
	}

	/// # Safety
	///
	/// Consuming code is allow to rely on these bounds for safety/correctness.
	/// The iter _must_ produce _exactly_ `bound` items.
	pub unsafe fn hard_bound(bound: usize) -> Self {
		Self {
			lower: SizeHintBound::HardBound { bound },
			upper: SizeHintBound::HardBound { bound }
		}
	}

	/// # Safety
	///
	/// Consuming code is allowed to rely on this value for safety/correctness.
	/// The iter _must_ produce at least `bound` items.
	pub unsafe fn with_lower_hard_bound(mut self, bound: usize) -> Self {
		self.lower = SizeHintBound::HardBound { bound };
		self
	}

	/// # Safety
	///
	/// Consuming code is allowed to rely on this value for safety/correctness.
	/// The iter _must_ produce at most `bound` items.
	pub unsafe fn with_upper_hard_bound(mut self, bound: usize) -> Self {
		self.upper = SizeHintBound::HardBound { bound };
		self
	}

	/// Returns (lower, upper) bounds. Use this to `match` on a size hint to check it.
	pub fn split(self) -> (SizeHintBound, SizeHintBound) {
		(self.lower, self.upper)
	}
}

impl Default for SizeHint {
	/// Returns default size hint, or `(Unknown, Unknown)`.
	fn default() -> Self {
		Self::unknown()
	}
}
