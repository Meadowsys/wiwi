// pub use wiwiwiwiwi::MemoryUsage;

/// Trait for types that can calculate their actual total memory usage, not
/// just the stack usage (ie. not just [`size_of`])
///
/// # "Heap"
///
/// For the purposes of this trait, and its [`heap_usage`] and [`heap_usage_with_extra_capacity`]
/// methods... what counts as the "heap" is just "anything that's not on the
/// stack". For example, the value a reference is pointing to is considered
/// part of the "heap" (even if it may be just a reference to a local value),
/// and the contents of string literals (`&'static str`), despite actually being
/// embedded somewhere in the binary (and the str reference just points to that),
/// also count towards the "heap".
pub trait MemoryUsage {
	/// Gets the stack usage of this value
	///
	/// For [`Sized`] types, using [`size_of`] will almost always be correct.
	fn stack_usage(&self) -> usize;

	/// Gets the heap usage of this value
	///
	/// For simple values like [`u64`], this should be 0, as it does not
	/// dynamically allocate. For values like [`Box`] or collections like
	/// [`Vec`], this should be calculated.
	///
	/// For values that potentially allocate extra capacity, like [`Vec`], this
	/// method is for the actual memory being utilised. In other words, the minimum
	/// amount of memory required without data loss. For vec this would be just its
	/// `len`.
	///
	/// Also see [`heap_usage_with_extra_capacity`].
	///
	/// [`heap_usage_with_extra_capacity`]: MemoryUsage::heap_usage_with_extra_capacity
	fn heap_usage(&self) -> usize;

	/// Gets the heap usage of this value, including "excess capacity" and other
	/// related extra allocated memory
	///
	/// For values that potentially allocate extra capacity, like [`Vec`], this
	/// method is for _all_ memory allocated, including "excess capacity". In other
	/// words, _all_ memory that is currently allocated. For vec this would be its
	/// `capacity`.
	///
	/// Also see [`heap_usage`].
	///
	/// [`heap_usage`]: MemoryUsage::heap_usage
	fn heap_usage_with_extra_capacity(&self) -> usize;

	/// Gets the total memory usage of this value, including the stack usage
	/// and the heap usage
	///
	/// This should be equivalent to calling [`stack_usage`] and [`heap_usage`]
	/// and adding them up.
	///
	/// [`stack_usage`]: MemoryUsage::stack_usage
	/// [`heap_usage`]: MemoryUsage::heap_usage
	fn total_usage(&self) -> usize;

	/// Gets the total memory usage of this value, including the stack usage,
	/// the heap usage, and the "excess capacity" if applicable.
	///
	/// This should be equivalent to calling [`stack_usage`] and
	/// [`heap_usage_with_extra_capacity`] and adding them up.
	///
	/// [`stack_usage`]: MemoryUsage::stack_usage
	/// [`heap_usage_with_extra_capacity`]: MemoryUsage::heap_usage_with_extra_capacity
	fn total_usage_with_extra_capacity(&self) -> usize;
}

/// Trait for types that can know all of their memory usage at compile time
///
/// [`memory_usage_static`] should be a const fn... but rust doesn't allow
/// const fn in traits for now, so it doesn't really do anything extra over
/// [`MemoryUsage`]. If/when const traits / const fn in traits are implemented /
/// stabilised, this will become a const trait / const fn, and then
/// [`memory_usage_static`] will be usable in const contexts.
///
/// [`memory_usage_static`]: MemoryUsageStatic::memory_usage_static
pub trait MemoryUsageStatic: MemoryUsage {
	/// Statically calculates memory usage
	///
	/// This is useful for things like `&'static str` (ie. string literals),
	/// which can calculate their usage in a `const` context (`.len()` on `str` is
	/// `const`).
	///
	/// See the trait docs for information on the possibility of this fn/trait
	/// being `const` in the future.
	fn memory_usage_static(&self) -> usize;
}

/// Trait for types that can provide their memory usage as a `const` value
///
/// This is a little stricter than [`MemoryUsageStatic`], since the provided
/// `const` would apply to all values of a type. For example, `str` can implement
/// [`MemoryUsageStatic`] correctly, but not this trait. This is because `.len()`
/// on str is `const`, but `.len()` requires to be called on an actual str, but
/// that's not available here.
pub trait MemoryUsageConst: MemoryUsageStatic {
	/// The constant memory usage value
	const MEMORY_USAGE: usize;
}

fn _assert_memory_usage_obj_safe(_: &dyn MemoryUsage) {}
fn _assert_memory_usage_static_obj_safe(_: &dyn MemoryUsageStatic) {}
// const is not object safe

/// Provides an impl of [`stack_usage`](MemoryUsage::stack_usage) using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsage`].
macro_rules! stack_usage_size_of_impl {
	() => {
		#[inline]
		fn stack_usage(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`heap_usage`](MemoryUsage::heap_usage) and
/// [`heap_usage_with_extra_capacity`](MemoryUsage::heap_usage_with_extra_capacity)
/// for types that never allocate onto the heap, returning just 0
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsage`].
macro_rules! heap_usage_zero_impl {
	() => {
		#[inline]
		fn heap_usage(&self) -> usize {
			0
		}

		// we don't actually need to override this... but doing so means one can't
		// use this macro, and then override `heap_usage_with_extra_capacity` with
		// something else, which is just wrong
		#[inline]
		fn heap_usage_with_extra_capacity(&self) -> usize {
			0
		}
	}
}

macro_rules! total_usage_default_impl {
	() => {
		#[inline]
		fn total_usage(&self) -> usize {
			self.stack_usage() + self.heap_usage()
		}
	}
}

macro_rules! total_usage_with_extra_capacity_default_impl {
	() => {
		#[inline]
		fn total_usage_with_extra_capacity(&self) -> usize {
			self.stack_usage() + self.heap_usage_with_extra_capacity()
		}
	}
}

/// Provides an impl of [`memory_usage_static`](MemoryUsageStatic::memory_usage_static)
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsageStatic`].
macro_rules! usage_static_impl {
	() => {
		#[inline]
		fn memory_usage_static(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MEMORY_USAGE`](MemoryUsageConst::MEMORY_USAGE)
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsageConst`].\
macro_rules! usage_const_impl {
	() => {
		const MEMORY_USAGE: usize = ::std::mem::size_of::<Self>();
	}
}

/// Implements [`MemoryUsage`], [`MemoryUsageStatic`], and [`MemoryUsageConst`]
/// for types that only live on the stack (stack usage is using [`size_of`],
/// and heap usage is 0)
///
/// Use by invoking in a standalone place. This macro generates the three
/// entire trait implementations
macro_rules! stack_only_impl {
	{ $($type:path)* } => {
		$(
			impl MemoryUsage for $type {
				stack_usage_size_of_impl!();
				heap_usage_zero_impl!();
				total_usage_default_impl!();
				total_usage_with_extra_capacity_default_impl!();
			}

			impl MemoryUsageStatic for $type {
				usage_static_impl!();
			}

			impl MemoryUsageConst for $type {
				usage_const_impl!();
			}
		)*
	}
}

stack_only_impl! {
	bool char
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	// f16 f128
}

impl<'h, T: MemoryUsage> MemoryUsage for &'h T {
	#[inline]
	fn stack_usage(&self) -> usize {
		T::stack_usage(self)
	}

	#[inline]
	fn heap_usage(&self) -> usize {
		T::heap_usage(self)
	}

	#[inline]
	fn heap_usage_with_extra_capacity(&self) -> usize {
		T::heap_usage_with_extra_capacity(self)
	}

	#[inline]
	fn total_usage(&self) -> usize {
		T::total_usage(self)
	}

	#[inline]
	fn total_usage_with_extra_capacity(&self) -> usize {
		T::total_usage_with_extra_capacity(self)
	}
}

impl<'h, T: MemoryUsage> MemoryUsage for &'h mut T {
	#[inline]
	fn stack_usage(&self) -> usize {
		T::stack_usage(self)
	}

	#[inline]
	fn heap_usage(&self) -> usize {
		T::heap_usage(self)
	}

	#[inline]
	fn heap_usage_with_extra_capacity(&self) -> usize {
		T::heap_usage_with_extra_capacity(self)
	}

	#[inline]
	fn total_usage(&self) -> usize {
		T::total_usage(self)
	}

	#[inline]
	fn total_usage_with_extra_capacity(&self) -> usize {
		T::total_usage_with_extra_capacity(self)
	}
}

impl<T: MemoryUsage, const N: usize> MemoryUsage for [T; N] {
	stack_usage_size_of_impl!();

	#[inline]
	fn heap_usage(&self) -> usize {
		self.iter()
			.map(T::heap_usage)
			.sum()
	}

	#[inline]
	fn heap_usage_with_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::heap_usage_with_extra_capacity)
			.sum()
	}

	#[inline]
	fn total_usage(&self) -> usize {
		self.iter()
			.map(T::total_usage)
			.sum()
	}

	#[inline]
	fn total_usage_with_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::total_usage_with_extra_capacity)
			.sum()
	}
}

impl<T: MemoryUsageStatic, const N: usize> MemoryUsageStatic for [T; N] {
	fn memory_usage_static(&self) -> usize {
		// while loop instead of for loop, for loops aren't available in const
		// fn (yet?),getting ready to make this a const fn
		let mut usage = 0;

		let mut i = 0;
		while i < N {
			usage += self[i].memory_usage_static();

			i += 1;
		}

		usage
	}
}

impl<T: MemoryUsageConst, const N: usize> MemoryUsageConst for [T; N] {
	const MEMORY_USAGE: usize = T::MEMORY_USAGE * N;
}

// impl<T: Dynamic, const N: usize> Dynamic for [T; N] {
// 	#[inline]
// 	fn calculate_memory_usage(&self) -> usize {
// 		self.iter().map(T::calculate_memory_usage).sum()
// 	}
//
// 	#[inline]
// 	fn calculate_values_usage(&self) -> usize {
// 		self.iter().map(T::calculate_values_usage).sum()
// 	}
// }
//
// impl<T: Dynamic> Dynamic for [T] {
// 	#[inline]
// 	fn calculate_memory_usage(&self) -> usize {
// 		let contents = self.iter().map(T::calculate_memory_usage).sum::<usize>();
// 		size_of::<&[T]>() + contents
// 	}
//
// 	#[inline]
// 	fn calculate_values_usage(&self) -> usize {
// 		let contents = self.iter().map(T::calculate_values_usage).sum::<usize>();
// 		size_of::<&[T]>() + contents
// 	}
// }
//
// impl<T: ?Sized> Static for *const T {
// 	const MEMORY_USAGE: usize = size_of::<*const T>();
// }
//
// impl<T: ?Sized> Static for *mut T {
// 	const MEMORY_USAGE: usize = size_of::<*mut T>();
// }
//
// impl<T: Dynamic> Dynamic for Vec<T> {
// 	#[inline]
// 	fn calculate_memory_usage(&self) -> usize {
// 		let contents = self.iter().map(T::calculate_memory_usage).sum::<usize>();
// 		let uninit = (self.capacity() - self.len()) * size_of::<T>();
// 		size_of::<Vec<T>>() + contents + uninit
// 	}
//
// 	#[inline]
// 	fn calculate_values_usage(&self) -> usize {
// 		let contents = self.iter().map(T::calculate_values_usage).sum::<usize>();
// 		size_of::<Vec<T>>() + contents
// 	}
// }
//
// impl Dynamic for String {
// 	#[inline]
// 	fn calculate_memory_usage(&self) -> usize {
// 		size_of::<String>() + self.capacity()
// 	}
//
// 	#[inline]
// 	fn calculate_values_usage(&self) -> usize {
// 		size_of::<String>() + self.len()
// 	}
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;
//
// 	#[test]
// 	fn static_types_and_std_sized() {
// 		fn check<T: Static>() {
// 			assert_eq!(T::MEMORY_USAGE, size_of::<T>());
// 		}
//
// 		check::<u8>();
// 		check::<u16>();
// 		check::<u32>();
// 		check::<u64>();
// 		check::<u128>();
//
// 		check::<i8>();
// 		check::<i16>();
// 		check::<i32>();
// 		check::<i64>();
// 		check::<i128>();
//
// 		check::<usize>();
// 		check::<isize>();
// 	}
//
// 	#[test]
// 	fn vec_size() {
// 		let mut vec = Vec::<i32>::new();
// 		// ??? lol
// 		let base_vec_usage = <(*const i32, usize, usize)>::calculate_memory_usage(&(std::ptr::NonNull::dangling().as_ptr(), 0, 0));
//
// 		assert_eq!(vec.calculate_memory_usage(), base_vec_usage);
// 		assert_eq!(vec.calculate_values_usage(), base_vec_usage);
//
// 		vec.reserve(32);
//
// 		let mem_use = vec.calculate_memory_usage();
// 		let val_use = vec.calculate_values_usage();
// 		assert!(mem_use >= base_vec_usage + (32 * i32::MEMORY_USAGE));
// 		assert_eq!(val_use, base_vec_usage);
//
// 		vec.extend([1, 2, 3, 4, 5, 6, 7, 8]);
// 		assert_eq!(vec.calculate_values_usage(), base_vec_usage + (8 * i32::MEMORY_USAGE));
// 		assert_eq!(vec.calculate_memory_usage(), mem_use);
// 	}
// }
