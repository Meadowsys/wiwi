use std::mem::size_of;
pub use wiwiwiwiwi::MemoryUsage;

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
/// Rust's [`size_of`] function would say... it stores
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
	/// Calculate the memory usage of this value in bytes, including things like
	/// excess capacity, if that applies to the type of the value.
	fn calculate_memory_usage(&self) -> usize;

	/// Calculate the memory usage of the value in bytes, but only the memory
	/// that's "actually" being used.
	///
	/// Things like excess capacity would _not_ be
	/// included here.
	///
	/// A default implementation is provided, which just calls
	/// [`calculate_memory_usage`](Dynamic::calculate_memory_usage). You should
	/// not rely on this default implementation if your type manages excess
	/// capacity, and other similar things where there may be some memory that's
	/// not actually used to store any data.
	#[inline]
	fn calculate_values_usage(&self) -> usize {
		self.calculate_memory_usage()
	}
}

/// Fetches the statically known memory usage of a type.
///
/// For many types, this probably does
/// the same thing as [`size_of`]. However, for many types
/// like [`Vec`] that store elements on the heap, actual usage cannot be known
/// at compile time.
#[inline]
pub const fn static_mem_usage_of<T: Static>() -> usize {
	T::MEMORY_USAGE
}

/// Fetches the statically known memory usage of a value.
///
/// Only use this if for some reason you cannot name the type. If you can name
/// the type, prefer to use [`static_mem_usage_of`].
#[inline]
pub const fn static_mem_usage_of_val<T: Static>(_item: &T) -> usize {
	T::MEMORY_USAGE
}

/// Fetches the dynamically calculated memory usage of a value.
#[inline]
pub fn dynamic_mem_usage_of_val<T: Dynamic>(item: &T) -> usize {
	item.calculate_memory_usage()
}

#[inline]
pub fn dynamic_values_usage_of_val<T: Dynamic>(item: &T) -> usize {
	item.calculate_values_usage()
}

impl<T: Static + ?Sized> Dynamic for T {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		Self::MEMORY_USAGE
	}
}

macro_rules! impl_static_via_size_of {
	{ $($type:ty)* } => {
		$(
			impl Static for $type {
				const MEMORY_USAGE: usize = size_of::<$type>();
			}
		)*
	}
}

impl_static_via_size_of! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64 // f16 f128
	bool char
}

macro_rules! impl_dyn_mem_usage_tuple {
	// entry point
	{ $next:ident $($rest:ident)* } => {
		impl_dyn_mem_usage_tuple! { [$($rest)*] $next }
	};

	{ [] $curr:ident $($acc:ident)* } => {
		impl_dyn_mem_usage_tuple! { @impl $($acc)* $curr }
	};

	{ [$next:ident $($rest:ident)*] $curr:ident $($acc:ident)* } => {
		impl_dyn_mem_usage_tuple! { @impl $($acc)* $curr }
		impl_dyn_mem_usage_tuple! { [$($rest)*] $next $($acc)* $curr }
	};

	{ @impl $($stuff:ident)+ } => {
		// macro impl reuses idents that are used for generic param names for macro
		// impl simplicity, those idents are PascalCase and trigger this lint
		#[allow(non_snake_case)]
		impl<$($stuff: Dynamic,)+> Dynamic for ($($stuff,)+) {
			#[inline]
			fn calculate_memory_usage(&self) -> usize {
				let ($($stuff,)*) = self;
				let mut usage = 0;
				$(usage += <$stuff>::calculate_memory_usage($stuff);)*
				usage
			}

			#[inline]
			fn calculate_values_usage(&self) -> usize {
				let ($($stuff,)*) = self;
				let mut usage = 0;
				$(usage += <$stuff>::calculate_values_usage($stuff);)*
				usage
			}
		}
	};

	// actual impl
	(@impl $($t:ident)+) => {
		impl<$($t: Dynamic,)+> Dynamic for ($($t,)+) {
			// macro impl reuses idents that are used for generic param names for macro
			// impl simplicity, those idents are PascalCase and trigger this lint
			#[allow(non_snake_case)]
			fn calculate_memory_usage(&self) -> usize {
				let ($($t,)*) = self;
				let mut usage = 0;
				$(usage += <$t>::calculate_memory_usage($t);)*
				usage
			}

			// macro impl reuses idents that are used for generic param names for macro
			// impl simplicity, those idents are PascalCase and trigger this lint
			#[allow(non_snake_case)]
			fn calculate_values_usage(&self) -> usize {
				let ($($t,)*) = self;
				let mut usage = 0;
				$(usage += <$t>::calculate_values_usage($t);)*
				usage
			}
		}
	};
}

#[cfg(all(
	not(feature = "large-tuples"),
	not(feature = "omega-tuples-of-doom")
))]
impl_dyn_mem_usage_tuple! {
	T1 T2 T3 T4
	T5 T6 T7 T8
}

#[cfg(all(
	feature = "large-tuples",
	not(feature = "omega-tuples-of-doom")
))]
impl_dyn_mem_usage_tuple! {
	T1  T2  T3  T4
	T5  T6  T7  T8
	T9  T10 T11 T12
	T13 T14 T15 T16
	T17 T18 T19 T20
	T21 T22 T23 T24
	T25 T26 T27 T28
	T29 T30 T31 T32
}

#[cfg(feature = "omega-tuples-of-doom")]
impl_dyn_mem_usage_tuple! {
	T1   T2   T3   T4
	T5   T6   T7   T8
	T9   T10  T11  T12
	T13  T14  T15  T16
	T17  T18  T19  T20
	T21  T22  T23  T24
	T25  T26  T27  T28
	T29  T30  T31  T32
	T33  T34  T35  T36
	T37  T38  T39  T40
	T41  T42  T43  T44
	T45  T46  T47  T48
	T49  T50  T51  T52
	T53  T54  T55  T56
	T57  T58  T59  T60
	T61  T62  T63  T64
	T65  T66  T67  T68
	T69  T70  T71  T72
	T73  T74  T75  T76
	T77  T78  T79  T80
	T81  T82  T83  T84
	T85  T86  T87  T88
	T89  T90  T91  T92
	T93  T94  T95  T96
	T97  T98  T99  T100
	T101 T102 T103 T104
	T105 T106 T107 T108
	T109 T110 T111 T112
	T113 T114 T115 T116
	T117 T118 T119 T120
	T121 T122 T123 T124
	T125 T126 T127 T128
}

impl<T: Dynamic, const N: usize> Dynamic for [T; N] {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		self.iter().map(T::calculate_memory_usage).sum()
	}

	#[inline]
	fn calculate_values_usage(&self) -> usize {
		self.iter().map(T::calculate_values_usage).sum()
	}
}

impl<T: Dynamic> Dynamic for [T] {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		let contents = self.iter().map(T::calculate_memory_usage).sum::<usize>();
		size_of::<&[T]>() + contents
	}

	#[inline]
	fn calculate_values_usage(&self) -> usize {
		let contents = self.iter().map(T::calculate_values_usage).sum::<usize>();
		size_of::<&[T]>() + contents
	}
}

impl<T: ?Sized> Static for *const T {
	const MEMORY_USAGE: usize = size_of::<*const T>();
}

impl<T: ?Sized> Static for *mut T {
	const MEMORY_USAGE: usize = size_of::<*mut T>();
}

impl<T: Dynamic> Dynamic for Vec<T> {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		let contents = self.iter().map(T::calculate_memory_usage).sum::<usize>();
		let uninit = (self.capacity() - self.len()) * size_of::<T>();
		size_of::<Vec<T>>() + contents + uninit
	}

	#[inline]
	fn calculate_values_usage(&self) -> usize {
		let contents = self.iter().map(T::calculate_values_usage).sum::<usize>();
		size_of::<Vec<T>>() + contents
	}
}

impl Dynamic for String {
	#[inline]
	fn calculate_memory_usage(&self) -> usize {
		size_of::<String>() + self.capacity()
	}

	#[inline]
	fn calculate_values_usage(&self) -> usize {
		size_of::<String>() + self.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn static_types_and_std_sized() {
		fn check<T: Static>() {
			assert_eq!(T::MEMORY_USAGE, size_of::<T>());
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

	#[test]
	fn vec_size() {
		let mut vec = Vec::<i32>::new();
		// ??? lol
		let base_vec_usage = <(*const i32, usize, usize)>::calculate_memory_usage(&(std::ptr::NonNull::dangling().as_ptr(), 0, 0));

		assert_eq!(vec.calculate_memory_usage(), base_vec_usage);
		assert_eq!(vec.calculate_values_usage(), base_vec_usage);

		vec.reserve(32);

		let mem_use = vec.calculate_memory_usage();
		let val_use = vec.calculate_values_usage();
		assert!(mem_use >= base_vec_usage + (32 * i32::MEMORY_USAGE));
		assert_eq!(val_use, base_vec_usage);

		vec.extend([1, 2, 3, 4, 5, 6, 7, 8]);
		assert_eq!(vec.calculate_values_usage(), base_vec_usage + (8 * i32::MEMORY_USAGE));
		assert_eq!(vec.calculate_memory_usage(), mem_use);
	}
}
