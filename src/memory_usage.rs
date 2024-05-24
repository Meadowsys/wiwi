/// Trait implemented by types that, at compile time, use a
/// known, fixed amount of memory.
///
/// For example, [`u8`] is always 1 byte, [`u64`] is always 8 bytes, etc.
///
/// There is a blanket implementation of the [`Dynamic`] trait for all types that
/// implement [`Static`].
///
/// Note: this is not the same as Rust's [`Sized`] trait. See the docs of the
/// [`Dynamic`] trait for more information.
///
/// Prefer to implement this trait over [`Dynamic`] and use [`Dynamic`] (in eg. trait
/// bounds) over this trait where possible, for maximum flexibility.
pub trait Static: Dynamic {
	/// The memory usage of this type in bytes
	const MEMORY_USAGE: usize;
}

/// Trait implemented by types that at compile time, for whatever reason, don't
/// have a known, fixed memory usage, but can still calculate it at runtime.
///
/// For example, [`Vec<u8>`] isn't just 24 bytes (12 bytes on 32 bit), like what
/// Rust's [`size_of`](std::mem::size_of) function would say... it stores
/// elements on the heap, right? That's what this trait is for, and how it differs
/// from Rust's [`Sized`] trait. This trait is implemented by types that can
/// calculate their current actual memory usage, and not just stack usage. That
/// is how this differs from Rust's [`Sized`] trait.
///
/// There is a blanket implementation of this trait for all types that implement
/// [`Static`].
///
/// Prefer to implement [`Static`] over this trait and use this trait (in eg. trait
/// bounds) over [`Static`] where possible, for maximum flexibility.
pub trait Dynamic {
	/// Calculate the memory usage of this type in bytes
	fn calculate_memory_usage(&self) -> usize;
}

/// Fetches the statically known memory usage of a type.
///
/// For many types, this probably does
/// the same thing as [`size_of`](std::mem::size_of). However, for many types
/// like [`Vec`] that store elements on the heap, actual usage cannot be known
/// at compile time.
pub const fn static_mem_usage_of<T: Static>() -> usize {
	T::MEMORY_USAGE
}

/// Fetches the statically known memory usage of a value.
///
/// Only use this if for some reason you cannot name the type. If you can name
/// the type, prefer to use [`static_mem_usage_of`].
pub const fn static_mem_usage_of_val<T: Static>(_item: &T) -> usize {
	T::MEMORY_USAGE
}

/// Fetches the dynamically calculated memory usage of a value.
pub fn dynamic_mem_usage_of_val<T: Dynamic>(item: &T) -> usize {
	item.calculate_memory_usage()
}

impl<T: Static> Dynamic for T {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		Self::MEMORY_USAGE
	}
}

macro_rules! impl_static_mem_usage {
	($(($type:ty, $usage:expr))*) => {
		$(
			impl Static for $type {
				const MEMORY_USAGE: usize = $usage;
			}
		)*
	};
}

impl_static_mem_usage! {
	(u8, 1)
	(u16, 2)
	(u32, 4)
	(u64, 8)
	(u128, 16)

	(i8, 1)
	(i16, 2)
	(i32, 4)
	(i64, 8)
	(i128, 16)

	(usize, usize::BITS as usize / 8)
	(isize, isize::BITS as usize / 8)
}

macro_rules! impl_static_mem_usage_tuple {
	// entry point
	($($t:ident)*) => {
		impl_static_mem_usage_tuple!(@init $($t)*);
	};

	// base case (ary 1)
	(@init $t:ident) => {
		impl_static_mem_usage_tuple!(@flip [$t]);
	};

	// running/entry case (ary 2+)
	(@init $t:ident $($rest:ident)+) => {
		impl_static_mem_usage_tuple!(@flip [$t $($rest)*]);
		impl_static_mem_usage_tuple!($($rest)*);
	};

	// type param order flipping base case
	(@flip [] $($reversed:ident)*) => {
		impl_static_mem_usage_tuple!(@impl $($reversed)*);
	};

	// type param order flipping running case
	(@flip [$t:ident $($rest:ident)*] $($reversed:ident)*) => {
		impl_static_mem_usage_tuple!(@flip [$($rest)*] $t $($reversed)*);
	};

	// actual impl
	(@impl $($t:ident)+) => {
		impl<$($t: Static,)+> Static for ($($t,)+) {
			const MEMORY_USAGE: usize = 0 $(+ <$t as Static>::MEMORY_USAGE)+;
		}
	};
}

impl_static_mem_usage_tuple!{
	T31 T30 T29 T28
	T27 T26 T25 T24
	T23 T22 T21 T20
	T19 T18 T17 T16
	T15 T14 T13 T12
	T11 T10 T9  T8
	T7  T6  T5  T4
	T3  T2  T1  T0
}

// TODO: trait impl specialisation here or something?
// default impl<T: Static, const N: usize> Static for [T; N] {
// 	const MEMORY_USAGE: usize = T::MEMORY_USAGE * N;
// }

/// Unfortunately, having both a [`Static`] and [`Dynamic`] impl collides with
/// the blanket impl of `Dynamic` for all `T: Static`. We choose to implement
/// `Dynamic` over static because then arrays will at least have a `Dynamic`
/// impl for all items, `Static` or `Dynamic`.
impl<T: Dynamic, const N: usize> Dynamic for [T; N] {
	// const MEMORY_USAGE: usize = T::MEMORY_USAGE * N;
	fn calculate_memory_usage(&self) -> usize {
		self.iter().map(T::calculate_memory_usage).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn static_types_and_std_sized() {
		fn check<T: Static>() {
			assert_eq!(T::MEMORY_USAGE, std::mem::size_of::<T>());
		}

		check::<u8>();
		check::<u16>();
		check::<u32>();
		check::<u64>();
		check::<u128>();

		check::<i8>();
		check::<i16>();
		check::<i32>();
		check::<i64>();
		check::<i128>();

		check::<usize>();
		check::<isize>();
	}
}
