pub struct SizeHint {
	imp: SizeHintImpl
}

pub struct SizeHintImpl {
	inner: SizeHintImplInner
}

enum SizeHintImplInner {
	Unknown,
	Single { bound: Bound },
	Lower { bound: Bound },
	Upper { bound: Bound },
	Range { lower: Bound, upper: Bound }
}

enum Bound {
	Hard { count: usize },
	Estimate { count: usize }
}

impl SizeHint {
	#[inline]
	unsafe fn with_inner(inner: SizeHintImplInner) -> SizeHintImpl {
		SizeHintImpl { inner }
	}

	#[inline]
	unsafe fn single(bound: Bound) -> SizeHintImpl {
		Self::with_inner(SizeHintImplInner::Lower { bound })
	}

	#[inline]
	unsafe fn lower(bound: Bound) -> SizeHintImpl {
		Self::with_inner(SizeHintImplInner::Lower { bound })
	}

	#[inline]
	unsafe fn upper(bound: Bound) -> SizeHintImpl {
		Self::with_inner(SizeHintImplInner::Upper { bound })
	}

	#[inline]
	unsafe fn range(lower: Bound, upper: Bound) -> SizeHintImpl {
		Self::with_inner(SizeHintImplInner::Range { lower, upper })
	}

	#[inline]
	pub unsafe fn unknown() -> SizeHintImpl {
		Self::with_inner(SizeHintImplInner::Unknown)
	}

	#[inline]
	pub unsafe fn hard(count: usize) -> SizeHintImpl {
		Self::single(Bound::Hard { count })
	}

	#[inline]
	pub unsafe fn estimate(count: usize) -> SizeHintImpl {
		Self::single(Bound::Estimate { count })
	}

	#[inline]
	pub unsafe fn lower_hard(count: usize) -> SizeHintImpl {
		Self::lower(Bound::Hard { count })
	}

	#[inline]
	pub unsafe fn lower_estimate(count: usize) -> SizeHintImpl {
		Self::lower(Bound::Estimate { count })
	}

	#[inline]
	pub unsafe fn upper_hard(count: usize) -> SizeHintImpl {
		Self::upper(Bound::Hard { count })
	}

	#[inline]
	pub unsafe fn upper_estimate(count: usize) -> SizeHintImpl {
		Self::upper(Bound::Estimate { count })
	}

	#[inline]
	pub unsafe fn range_hard(lower: usize, upper: usize) -> SizeHintImpl {
		Self::range(Bound::Hard { count: lower }, Bound::Hard { count: upper })
	}

	#[inline]
	pub unsafe fn range_estimate(lower: usize, upper: usize) -> SizeHintImpl {
		Self::range(Bound::Estimate { count: lower }, Bound::Estimate { count: upper })
	}

	#[inline]
	pub unsafe fn range_lhard_uestimate(lower: usize, upper: usize) -> SizeHintImpl {
		Self::range(Bound::Hard { count: lower }, Bound::Estimate { count: upper })
	}

	#[inline]
	pub unsafe fn range_lestimate_uhard(lower: usize, upper: usize) -> SizeHintImpl {
		Self::range(Bound::Estimate { count: lower }, Bound::Hard { count: upper })
	}

}

impl SizeHintImpl {
	#[inline]
	pub(super) fn wrap(self) -> SizeHint {
		SizeHint { imp: self }
	}
}
