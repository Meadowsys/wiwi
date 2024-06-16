use super::{ IntoIter, Iter, SizeHintBound, SizeHintImpl, SizeHintInner, SizeHintMarker };
use std::ptr;

macro_rules! iter_tuple_impl {
	// input case
	{
		($($next_stuff:tt)+)
		$(($($input_stuff:tt)+))*
	} => {
		iter_tuple_impl!([$(($($input_stuff)+))*] ($($next_stuff)+));
	};

	// base case (everything taken)
	{
		[]
		($($curr_stuff:tt)+)
		$(($($rem_stuff:tt)+))*
	} => {
		iter_tuple_impl!(@impl $($curr_stuff)+ $(($($rem_stuff)+))*);
	};

	// running case
	{
		[($($next_stuff:tt)+) $(($($input_stuff:tt)+))*]
		($($curr_stuff:tt)+)
		$(($($rem_stuff:tt)+))*
	} => {
		iter_tuple_impl!([$(($($input_stuff)+))*] ($($next_stuff)+) $(($($rem_stuff)+))* ($($curr_stuff)+));
		iter_tuple_impl!(@impl $($curr_stuff)+ $(($($rem_stuff)+))*);
	};

	// impl case
	{ @impl $curr_s:ident $curr_t:ident $curr_f:ident $curr_l:literal $(($rem_s:ident $rem_t:ident $rem_f:ident $rem_l:literal))* } => {
		// s = struct name
		// t = T types
		// f = field name
		// l = literal, number of elements in tuple

		/// Iter for tuples of size
		#[doc = concat!(stringify!($curr_l), ".")]
		///
		/// [`IntoIter`] implementations are available for tuples of up to size 32,
		/// and their concrete struct types can be found [here](super). Obtain an
		/// instance by calling [`into_wiwi_iter`](IntoIter::into_wiwi_iter)
		/// on a tuple containing all iters (or structs implementing [`IntoIter`],
		/// or any combination of).
		///
		/// At least for now, you must use [`IntoWiwiIter`]
		/// or [`AsWiwiIter`] for std iterators before putting them into the tuple
		/// for the [`IntoIter`] implementation to be available.
		///
		/// [`IntoWiwiIter`]: super::IntoWiwiIter
		/// [`AsWiwiIter`]: super::AsWiwiIter
		pub struct $curr_s<$($rem_t,)* $curr_t> {
			$($rem_f: $rem_t,)*
			$curr_f: $curr_t,
			exhausted: bool
		}

		impl<$($rem_t,)* $curr_t> IntoIter for ($($rem_t,)* $curr_t,)
		where
			$($rem_t: IntoIter,)*
			$curr_t: IntoIter
		{
			type Item = ($($rem_t::Item,)* $curr_t::Item,);
			type Iter = $curr_s<$($rem_t::Iter,)* $curr_t::Iter>;

			fn into_wiwi_iter(self) -> Self::Iter {
				let ($($rem_f,)* $curr_f,) = self;
				$curr_s {
					$($rem_f: $rem_f.into_wiwi_iter(),)*
					$curr_f: $curr_f.into_wiwi_iter(),
					exhausted: false
				}
			}
		}

		impl<$($rem_t,)* $curr_t> Iter for $curr_s<$($rem_t,)* $curr_t>
		where
			$($rem_t: Iter,)*
			$curr_t: Iter
		{
			type Item = ($($rem_t::Item,)* $curr_t::Item,);

			fn next(&mut self) -> Option<Self::Item> {
				let Self { $($rem_f,)* $curr_f, exhausted } = self;
				if *exhausted { return None }

				let item = ($($rem_f.next(),)* $curr_f.next(),);
				match item {
					($(Some($rem_f),)* Some($curr_f),) => { Some(($($rem_f,)* $curr_f,)) }
					_ => {
						*exhausted = true;
						None
					}
				}
			}

			unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
				let Self { $($rem_f,)* $curr_f, exhausted } = self;
				if *exhausted { return unsafe { SizeHintImpl::hard(0) } }

				// using curr_t since I need some "seed value" for the var
				// and curr_t is very conveniently seperated and always present
				let hint = $curr_f.size_hint().into();
				$(let hint = unsafe { min_size_hint(hint, $rem_f.size_hint().into()) };)*
				hint
			}
		}
	};
}

iter_tuple_impl! {
	(Tuple1 I1 iter1 1)    (Tuple2 I2 iter2 2)    (Tuple3 I3 iter3 3)    (Tuple4 I4 iter4 4)
	(Tuple5 I5 iter5 5)    (Tuple6 I6 iter6 6)    (Tuple7 I7 iter7 7)    (Tuple8 I8 iter8 8)
	(Tuple9 I9 iter9 9)    (Tuple10 I10 iter10 10) (Tuple11 I11 iter11 11) (Tuple12 I12 iter12 12)
	(Tuple13 I13 iter13 13) (Tuple14 I14 iter14 14) (Tuple15 I15 iter15 15) (Tuple16 I16 iter16 16)
	(Tuple17 I17 iter17 17) (Tuple18 I18 iter18 18) (Tuple19 I19 iter19 19) (Tuple20 I20 iter20 20)
	(Tuple21 I21 iter21 21) (Tuple22 I22 iter22 22) (Tuple23 I23 iter23 23) (Tuple24 I24 iter24 24)
	(Tuple25 I25 iter25 25) (Tuple26 I26 iter26 26) (Tuple27 I27 iter27 27) (Tuple28 I28 iter28 28)
	(Tuple29 I29 iter29 29) (Tuple30 I30 iter30 30) (Tuple31 I31 iter31 31) (Tuple32 I32 iter32 32)
}

unsafe fn min_size_hint(h1: SizeHintImpl, h2: SizeHintImpl) -> SizeHintImpl {
	use SizeHintInner::*;
	use SizeHintBound::*;

	match (h1.into_inner(), h2.into_inner()) {
		// when I write a comment that just says "reversed", I mean that the below pattern
		// is just a previous one but with the tuple items flipped

		// if both are unknown, of course we have no info to give

		(Unknown, Unknown) => { SizeHintImpl::unknown() }

		// if one is unknown, we can return the other as estimate (regardless of
		// it being hard or estimate). If the second were hard, we still cannot
		// provide a hard bound because we don't know what amount that unknown is
		// (if its greater or less than the other), so we can only provide estimate

		(Unknown, Single { bound: Hard { count } | Estimate { count } }) |
		// reversed
		(Single { bound: Hard { count } | Estimate { count } }, Unknown)
		=> { SizeHintImpl::estimate(count) }

		(Unknown, Lower { bound: Hard { count } | Estimate { count } }) |
		// reversed
		(Lower { bound: Hard { count } | Estimate { count } }, Unknown)
		=> { SizeHintImpl::lower_estimate(count) }

		(Unknown, Upper { bound: Hard { count } | Estimate { count } }) |
		// reversed
		(Upper { bound: Hard { count } | Estimate { count } }, Unknown)
		=> { SizeHintImpl::upper_estimate(count) }

		(Unknown, Range {
			lower: Hard { count: lower } | Estimate { count: lower },
			upper: Hard { count: upper } | Estimate { count: upper }
		}) |
		// reversed
		(Range {
			lower: Hard { count: lower } | Estimate { count: lower },
			upper: Hard { count: upper } | Estimate { count: upper }
		}, Unknown)
		=> { SizeHintImpl::range_estimate(lower, upper) }

		// if both are same type and hard bound, we can provide the lowest of the two
		// (as we stop if any iter returns `None`)

		(Single { bound: Hard { count: c1 } }, Single { bound: Hard { count: c2 } }) => {
			SizeHintImpl::hard(usize::min(c1, c2))
		}

		(Lower { bound: Hard { count: c1 } }, Lower { bound: Hard { count: c2 } }) => {
			SizeHintImpl::lower_hard(usize::min(c1, c2))
		}

		(Upper { bound: Hard { count: c1 } }, Upper { bound: Hard { count: c2 } }) => {
			SizeHintImpl::upper_hard(usize::min(c1, c2))
		}

		// if not both are hard, we can provide the smallest number as an estimate

		(Single { bound: Hard { count: c1 } }, Single { bound: Estimate { count: c2 } }) |
		(Single { bound: Estimate { count: c1 } }, Single { bound: Hard { count: c2 } }) |
		(Single { bound: Estimate { count: c1 } }, Single { bound: Estimate { count: c2 } })
		=> { SizeHintImpl::estimate(usize::min(c1, c2)) }

		(Lower { bound: Hard { count: c1 } }, Lower { bound: Estimate { count: c2 } }) |
		(Lower { bound: Estimate { count: c1 } }, Lower { bound: Hard { count: c2 } }) |
		(Lower { bound: Estimate { count: c1 } }, Lower { bound: Estimate { count: c2 } })
		=> { SizeHintImpl::lower_estimate(usize::min(c1, c2)) }

		(Upper { bound: Hard { count: c1 } }, Upper { bound: Estimate { count: c2 } }) |
		(Upper { bound: Estimate { count: c1 } }, Upper { bound: Hard { count: c2 } }) |
		(Upper { bound: Estimate { count: c1 } }, Upper { bound: Estimate { count: c2 } })
		=> { SizeHintImpl::upper_estimate(usize::min(c1, c2)) }

		// single estimate and one sided any / range any, can only return estimate

		(Single { bound: Estimate { count: c1 } }, Lower { bound: Estimate { count: c2 } | Hard { count: c2 } }) |
		// reversed
		(Lower { bound: Estimate { count: c2 } | Hard { count: c2 } }, Single { bound: Estimate { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2), c1) }

		(Single { bound: Estimate { count: c1 } }, Upper { bound: Estimate { count: c2 } | Hard { count: c2 } }) |
		// reversed
		(Upper { bound: Estimate { count: c2 } | Hard { count: c2 } }, Single { bound: Estimate { count: c1 } })
		=> { SizeHintImpl::range_estimate(c1, usize::min(c1, c2)) }

		(Single { bound: Estimate { count: c1 } }, Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}, Single { bound: Estimate { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2_l), usize::min(c1, c2_u)) }

		// single hard and one sided hard / range hard, can return hard for sides
		// with both hard, and estimate for the others

		(Single { bound: Hard { count: c1 } }, Lower { bound: Hard { count: c2 } }) |
		// reversed
		(Lower { bound: Hard { count: c2 } }, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lhard_uestimate(usize::min(c1, c2), c1) }

		(Single { bound: Hard { count: c1 } }, Upper { bound: Hard { count: c2 } }) |
		// reversed
		(Upper { bound: Hard { count: c2 } }, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lestimate_uhard(c1, usize::min(c1, c2)) }

		(Single { bound: Hard { count: c1 } }, Range {
			lower: Hard { count: c2_l },
			upper: Hard { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l },
			upper: Hard { count: c2_u }
		}, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_hard(usize::min(c1, c2_l), usize::min(c1, c2_u)) }

		(Single { bound: Hard { count: c1 } }, Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lestimate_uhard(usize::min(c1, c2_l), usize::min(c1, c2_u)) }

		(Single { bound: Hard { count: c1 } }, Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lhard_uestimate(usize::min(c1, c2_l), usize::min(c1, c2_u)) }

		(Single { bound: Hard { count: c1 } }, Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2_l), usize::min(c1, c2_u)) }

		// single hard and one sided estimate, can only return estimate

		(Single { bound: Hard { count: c1 } }, Lower { bound: Estimate { count: c2 } }) |
		// reversed
		(Lower { bound: Estimate { count: c2 } }, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2), c1) }

		(Single { bound: Hard { count: c1 } }, Upper { bound: Estimate { count: c2 } }) |
		// reversed
		(Upper { bound: Estimate { count: c2 } }, Single { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_estimate(c1, usize::min(c1, c2)) }

		// completely disjoint

		(Lower { bound: Hard { count: l } | Estimate { count: l } }, Upper { bound: Hard { count: u } | Estimate { count: u } }) |
		// reversed
		(Upper { bound: Hard { count: u } | Estimate { count: u } }, Lower { bound: Hard { count: l } | Estimate { count: l } })
		=> { SizeHintImpl::range_estimate(l, u) }

		// lower hard + range, when range lower is hard, emit that side as hard,
		// the rest is estimates

		(Lower { bound: Hard { count: c1 } }, Range {
			lower: Hard { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}, Lower { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lhard_uestimate(usize::min(c1, c2_l), c2_u) }

		// can only be estimate

		(Lower { bound: Hard { count: c1 } }, Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}, Lower { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2_l), c2_u) }

		// upper hard + range, when range upper is hard, emit that side as hard,
		// the rest is estimates

		(Upper { bound: Hard { count: c1 } }, Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}, Upper { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_lestimate_uhard(c2_l, usize::min(c1, c2_u)) }

		// can only be estimate

		(Upper { bound: Hard { count: c1 } }, Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Upper { bound: Hard { count: c1 } })
		=> { SizeHintImpl::range_estimate(c2_l, usize::min(c1, c2_u)) }

		// lower estimate + range

		(Lower { bound: Estimate { count: c1 } }, Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}, Lower { bound: Estimate { count: c1 } })
		=> { SizeHintImpl::range_estimate(usize::min(c1, c2_l), c2_u) }

		// upper estimate + range

		(Upper { bound: Estimate { count: c1 } }, Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l } | Estimate { count: c2_l },
			upper: Hard { count: c2_u } | Estimate { count: c2_u }
		}, Upper { bound: Estimate { count: c1 } })
		=> { SizeHintImpl::range_estimate(c2_l, usize::min(c1, c2_u)) }

		// apparently I never did range with itself until now lol
		// I swear, this match statement

		(Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Hard { count: c2_l },
			upper: Hard { count: c2_u }
		}) => { SizeHintImpl::range_hard(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Estimate { count: c1_l },
			upper: Estimate { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) => { SizeHintImpl::range_estimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Hard { count: c1_l },
			upper: Estimate { count: c1_u }
		}, Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}) => { SizeHintImpl::range_lhard_uestimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Estimate { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}) => { SizeHintImpl::range_lestimate_uhard(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		// disjoint

		(Range {
			lower: Estimate { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c1_l },
			upper: Estimate { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}) => { SizeHintImpl::range_estimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Hard { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		})
		=> { SizeHintImpl::range_lhard_uestimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Hard { count: c2_u }
		}, Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		})
		=> { SizeHintImpl::range_lestimate_uhard(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Range {
			lower: Hard { count: c1_l },
			upper: Hard { count: c1_u }
		})
		=> { SizeHintImpl::range_estimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Hard { count: c1_l },
			upper: Estimate { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Range {
			lower: Hard { count: c1_l },
			upper: Estimate { count: c1_u }
		})
		=> { SizeHintImpl::range_estimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		(Range {
			lower: Estimate { count: c1_l },
			upper: Hard { count: c1_u }
		}, Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}) |
		// reversed
		(Range {
			lower: Estimate { count: c2_l },
			upper: Estimate { count: c2_u }
		}, Range {
			lower: Estimate { count: c1_l },
			upper: Hard { count: c1_u }
		})
		=> { SizeHintImpl::range_estimate(usize::min(c1_l, c2_l), usize::min(c1_u, c2_u)) }

		// wait we're exhaustive now? :o
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn min_size_hint() {
		macro_rules! check {
			{
				h1: $h1:expr,
				h2: $h2:expr,
				expected: $expected:expr
			} => {
				unsafe {
					assert_eq!(super::min_size_hint($h1, $h2), $expected);
					assert_eq!(super::min_size_hint($h2, $h1), $expected);
				}
			}
		}


			// all unknown
			check! {
				h1: SizeHintImpl::unknown(),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::unknown()
			}

			// one lower or upper estimate
			check! {
				h1: SizeHintImpl::lower_estimate(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::lower_estimate(10)
			}
			check! {
				h1: SizeHintImpl::upper_estimate(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::upper_estimate(10)
			}

			// one both estimate
			check! {
				h1: SizeHintImpl::estimate(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::estimate(10)
			}

			// one lower, other upper estimate
			check! {
				h1: SizeHintImpl::lower_estimate(10),
				h2: SizeHintImpl::upper_estimate(10),
				expected: SizeHintImpl::estimate(10)
			}

			// // hard bound + unknown
			check! {
				h1: SizeHintImpl::lower_hard(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::lower_estimate(10)
			}
			check! {
				h1: SizeHintImpl::upper_hard(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::upper_estimate(10)
			}
			check! {
				h1: SizeHintImpl::hard(10),
				h2: SizeHintImpl::unknown(),
				expected: SizeHintImpl::estimate(10)
			}

			// hard bound + estimate
			check! {
				h1: SizeHintImpl::lower_hard(10),
				h2: SizeHintImpl::upper_estimate(10),
				expected: SizeHintImpl::estimate(10)
			}
			check! {
				h1: SizeHintImpl::upper_hard(10),
				h2: SizeHintImpl::lower_estimate(10),
				expected: SizeHintImpl::estimate(10)
			}

			// differing estimates
			check! {
				// actual range values makes no sense, but whatever lol
				h1: SizeHintImpl::range_estimate(10, 5),
				h2: SizeHintImpl::range_estimate(5, 10),
				expected: SizeHintImpl::estimate(5)
			}

			// differing hard
			check! {
				h1: SizeHintImpl::range_hard(10, 5),
				h2: SizeHintImpl::range_hard(5, 10),
				expected: SizeHintImpl::hard(5)
			}

			// differing hard + estimate
			check! {
				h1: SizeHintImpl::range_hard(10, 7),
				h2: SizeHintImpl::range_estimate(9, 12),
				expected: SizeHintImpl::range_estimate(9, 7)
			}
			check! {
				h1: SizeHintImpl::range_lestimate_uhard(7, 10),
				h2: SizeHintImpl::range_lhard_uestimate(9, 12),
				expected: SizeHintImpl::range_estimate(7, 10)
			}
	}

	#[test]
	fn size_hint() {
		let mut iter = (vec![1u8, 2, 3, 4, 5], vec![7usize, 6, 5, 4, 3, 2, 1]).into_wiwi_iter();

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(5) });
		assert_eq!(iter.next(), Some((1, 7)));

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(4) });
		assert_eq!(iter.next(), Some((2, 6)));

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some((3, 5)));

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some((4, 4)));

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some((5, 3)));

		assert!(!iter.exhausted);
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.next(), None);

		assert!(iter.exhausted);
	}
}
