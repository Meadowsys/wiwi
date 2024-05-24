use super::{ IntoIter, Iter, SizeHint, SizeHintBound };

macro_rules! iter_tuple_impl {
	// input case
	{
		($next_s:ident $next_t:ident)
		$(($input_s:ident $input_t:ident))*
	} => {
		iter_tuple_impl!([$(($input_s $input_t))*] ($next_s $next_t));
	};

	// base case (everything taken)
	{
		[]
		($curr_s:ident $curr_t:ident)
		$(($rem_s:ident $rem_t:ident))*
	} => {
		iter_tuple_impl!(@impl $curr_s $curr_t $(($rem_s $rem_t))*);
	};

	// running case
	{
		[($next_s:ident $next_t:ident) $(($input_s:ident $input_t:ident))*]
		($curr_s:ident $curr_t:ident)
		$(($rem_s:ident $rem_t:ident))*
	} => {
		iter_tuple_impl!([$(($input_s $input_t))*] ($next_s $next_t) $(($rem_s $rem_t))* ($curr_s $curr_t));
		iter_tuple_impl!(@impl $curr_s $curr_t $(($rem_s $rem_t))*);
	};

	// impl case
	{ @impl $curr_s:ident $curr_t:ident $(($rem_s:ident $rem_t:ident))* } => {
		pub struct $curr_s<$($rem_t,)* $curr_t>($($rem_t,)* $curr_t);

		#[allow(non_snake_case)]
		impl<$($rem_t,)* $curr_t> IntoIter for ($($rem_t,)* $curr_t,)
		where
			$($rem_t: IntoIter,)*
			$curr_t: IntoIter
		{
			type Item = ($($rem_t::Item,)* $curr_t::Item,);
			type Iter = $curr_s<$($rem_t::Iter,)* $curr_t::Iter>;

			fn into_wiwi_iter(self) -> Self::Iter {
				let ($($rem_t,)* $curr_t,) = self;
				$curr_s($($rem_t.into_wiwi_iter(),)* $curr_t.into_wiwi_iter())
			}
		}

		#[allow(non_snake_case)]
		impl<$($rem_t,)* $curr_t> Iter for $curr_s<$($rem_t,)* $curr_t>
		where
			$($rem_t: Iter,)*
			$curr_t: Iter
		{
			type Item = ($($rem_t::Item,)* $curr_t::Item,);

			fn next(&mut self) -> Option<Self::Item> {
				let Self($($rem_t,)* $curr_t,) = self;
				let item = ($($rem_t.next(),)* $curr_t.next(),);
				match item {
					($(Some($rem_t),)* Some($curr_t),) => { Some(($($rem_t,)* $curr_t,)) }
					_ => { None }
				}
			}

			// let mut hint to accomodate all tuples, but size 1 won't use it
			#[allow(unused_mut)]
			fn size_hint(&self) -> SizeHint {
				let Self($($rem_t,)* $curr_t,) = self;
				// using curr_t since I need some seed value for the var
				// and curr_t is very conveniently seperated and always present
				let mut hint = $curr_t.size_hint();
				$(hint = unsafe { min_size_hint(hint, $rem_t.size_hint()) };)*
				hint
			}
		}
	};
}

iter_tuple_impl! {
	(Tuple1 I1)   (Tuple2 I2)   (Tuple3 I3)   (Tuple4 I4)
	(Tuple5 I5)   (Tuple6 I6)   (Tuple7 I7)   (Tuple8 I8)
	(Tuple9 I9)   (Tuple10 I10) (Tuple11 I11) (Tuple12 I12)
	(Tuple13 I13) (Tuple14 I14) (Tuple15 I15) (Tuple16 I16)
	(Tuple17 I17) (Tuple18 I18) (Tuple19 I19) (Tuple20 I20)
	(Tuple21 I21) (Tuple22 I22) (Tuple23 I23) (Tuple24 I24)
	(Tuple25 I25) (Tuple26 I26) (Tuple27 I27) (Tuple28 I28)
	(Tuple29 I29) (Tuple30 I30) (Tuple31 I31) (Tuple32 I32)
}

unsafe fn min_size_hint(h1: SizeHint, h2: SizeHint) -> SizeHint {
	let (lower1, upper1) = h1.split();
	let (lower2, upper2) = h2.split();

	let mut hint = SizeHint::new();

	hint = min_size_hint_bound::<true>(lower1, lower2, hint);
	hint = min_size_hint_bound::<false>(upper1, upper2, hint);

	hint
}

#[inline]
unsafe fn min_size_hint_bound<const LOWER: bool>(b1: SizeHintBound, b2: SizeHintBound, hint: SizeHint) -> SizeHint {
	use SizeHintBound::*;

	match (b1, b2) {
		// if *BOTH* are hard bounds, we can return the minimum, since we will
		// always stop as soon as any iter returns None
		(HardBound { bound: b1 }, HardBound { bound: b2 }) => if LOWER {
			hint.with_lower_hard_bound(usize::min(b1, b2))
		} else {
			hint.with_upper_hard_bound(usize::min(b1, b2))
		}

		// other combinations of hard/estimate
		(HardBound { bound: est1 }, Estimate { estimate: est2 })
			| (Estimate { estimate: est1 }, HardBound { bound: est2 })
			| (Estimate { estimate: est1 }, Estimate { estimate: est2 })
		=> if LOWER {
			hint.with_lower_estimate(usize::min(est1, est2))
		} else {
			hint.with_upper_estimate(usize::min(est1, est2))
		}
		// if any one is not unknown, we can return that as estimate
		// since estimates are not to be relied on for correctness
		(HardBound { bound: est }, _)
			| (Estimate { estimate: est }, _)
			| (_, HardBound { bound: est })
			| (_, Estimate { estimate: est })
		=> if LOWER {
			hint.with_lower_estimate(est)
		} else {
			hint.with_upper_estimate(est)
		}

		// else we have no info to return except unknown
		(Unknown, Unknown) => if LOWER {
			hint.with_lower_unknown()
		} else {
			hint.with_upper_unknown()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn min_size_hint() {
		struct Checker {
			h1: SizeHint,
			h2: SizeHint,
			expected: SizeHint,
		}

		impl Checker {
			fn check(self) {
				let Self { h1, h2, expected } = self;

				// check both ways
				assert_eq!(unsafe { super::min_size_hint(h1.clone(), h2.clone()) }, expected);
				assert_eq!(unsafe { super::min_size_hint(h2, h1) }, expected);
			}
		}

		unsafe {
			// all unknown
			Checker {
				h1: SizeHint::unknown(),
				h2: SizeHint::unknown(),
				expected: SizeHint::unknown()
			}.check();

			// one lower or upper estimate
			Checker {
				h1: SizeHint::new().with_lower_estimate(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::new().with_lower_estimate(10)
			}.check();
			Checker {
				h1: SizeHint::new().with_upper_estimate(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::new().with_upper_estimate(10)
			}.check();

			// one both estimate
			Checker {
				h1: SizeHint::estimate(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::estimate(10)
			}.check();

			// one lower, other upper estimate
			Checker {
				h1: SizeHint::new().with_lower_estimate(10),
				h2: SizeHint::new().with_upper_estimate(10),
				expected: SizeHint::estimate(10)
			}.check();

			// hard bound + unknown
			Checker {
				h1: SizeHint::new().with_lower_hard_bound(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::new().with_lower_estimate(10)
			}.check();
			Checker {
				h1: SizeHint::new().with_upper_hard_bound(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::new().with_upper_estimate(10)
			}.check();
			Checker {
				h1: SizeHint::hard_bound(10),
				h2: SizeHint::unknown(),
				expected: SizeHint::estimate(10)
			}.check();

			// hard bound + estimate
			Checker {
				h1: SizeHint::new().with_lower_hard_bound(10),
				h2: SizeHint::new().with_upper_estimate(10),
				expected: SizeHint::estimate(10)
			}.check();
			Checker {
				h1: SizeHint::new().with_upper_hard_bound(10),
				h2: SizeHint::new().with_lower_estimate(10),
				expected: SizeHint::estimate(10)
			}.check();

			// differing estimates
			Checker {
				h1: SizeHint::new().with_lower_estimate(10).with_upper_estimate(5),
				h2: SizeHint::new().with_upper_estimate(10).with_lower_estimate(5),
				expected: SizeHint::estimate(5)
			}.check();

			// differing hard
			Checker {
				h1: SizeHint::new().with_lower_hard_bound(10).with_upper_hard_bound(5),
				h2: SizeHint::new().with_upper_hard_bound(10).with_lower_hard_bound(5),
				expected: SizeHint::hard_bound(5)
			}.check();

			// differing hard + estimate
			Checker {
				h1: SizeHint::new().with_lower_hard_bound(10).with_upper_hard_bound(7),
				h2: SizeHint::new().with_upper_estimate(12).with_lower_estimate(9),
				expected: SizeHint::new().with_lower_estimate(9).with_upper_estimate(7)
			}.check();
			Checker {
				h1: SizeHint::new().with_upper_hard_bound(10).with_lower_estimate(7),
				h2: SizeHint::new().with_upper_estimate(12).with_lower_hard_bound(9),
				expected: SizeHint::new().with_upper_estimate(10).with_lower_estimate(7)
			}.check();
		}
	}

	#[test]
	fn size_hint() {
		let mut iter = (vec![1u8, 2, 3, 4, 5], vec![7usize, 6, 5, 4, 3, 2, 1]).into_wiwi_iter();

		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(5) });
		assert_eq!(iter.next(), Some((1, 7)));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(4) });
		assert_eq!(iter.next(), Some((2, 6)));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(3) });
		assert_eq!(iter.next(), Some((3, 5)));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(2) });
		assert_eq!(iter.next(), Some((4, 4)));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(1) });
		assert_eq!(iter.next(), Some((5, 3)));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(0) });
		assert_eq!(iter.next(), None);
	}
}
