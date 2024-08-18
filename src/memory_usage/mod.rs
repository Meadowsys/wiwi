// pub use wiwiwiwiwi::MemoryUsage;

// todo: write doc comments on the _impl fns themselves_ describing the logic inside

/// Trait for types that can calculate their actual total memory usage, not
/// just the stack usage (ie. not just [`size_of`])
///
/// # Definition of "heap"
///
/// For the purposes of this trait, and its [`mem_use_heap`] and
/// [`mem_use_heap_incl_extra_capacity`] methods... what counts as the "heap" is
/// just "anything that's not on the stack". For example, the value a reference
/// is pointing to is considered part of the "heap" (even if it may be just a
/// reference to a local value), and the contents of string literals (`&'static str`),
/// despite actually being embedded somewhere in the binary (and the str
/// reference just points to that), also count towards the "heap".
///
/// [`mem_use_heap`]: MemoryUsage::mem_use_heap
/// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
pub trait MemoryUsage {
	/// Gets the stack usage of this value
	///
	/// For [`Sized`] types, using [`size_of`] will almost always be correct.
	fn mem_use_stack(&self) -> usize;

	/// Gets the heap usage of this value
	///
	/// For simple values like [`u64`], this should be 0, as it does not
	/// dynamically allocate. For values like [`Box`] or collections like
	/// [`Vec`], this should be calculated.
	///
	/// For values that potentially allocate extra capacity, like [`Vec`], this
	/// method is for the actual memory being utilised (ie. that contain data).
	/// For vec this would be just its len.
	///
	/// Also see [`mem_use_heap_incl_extra_capacity`].
	///
	/// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
	fn mem_use_heap(&self) -> usize;

	/// Gets the heap usage of this value, including "excess capacity" and other
	/// related extra allocated memory
	///
	/// For values that potentially allocate extra capacity, like [`Vec`], this
	/// method is for _all_ memory allocated, including "excess capacity". In other
	/// words, _all_ memory that is currently allocated. For vec this would be its
	/// `capacity`.
	///
	/// Also see [`mem_use_heap`].
	///
	/// [`mem_use_heap`]: MemoryUsage::mem_use_heap
	fn mem_use_heap_incl_extra_capacity(&self) -> usize;

	/// Gets the total memory usage of this value, including the stack usage
	/// and the heap usage
	///
	/// This should be equivalent to calling [`mem_use_stack`] and [`mem_use_heap`]
	/// and adding them up.
	///
	/// [`mem_use_stack`]: MemoryUsage::mem_use_stack
	/// [`mem_use_heap`]: MemoryUsage::mem_use_heap
	fn mem_use_total(&self) -> usize;

	/// Gets the total memory usage of this value, including the stack usage,
	/// the heap usage, and the "excess capacity" if applicable.
	///
	/// This should be equivalent to calling [`mem_use_stack`] and
	/// [`mem_use_heap_incl_extra_capacity`] and adding them up.
	///
	/// [`mem_use_stack`]: MemoryUsage::mem_use_stack
	/// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
	fn mem_use_total_incl_extra_capacity(&self) -> usize;
}

/// Trait for types that can know all of their memory usage at compile time
///
/// [`mem_use_static`] should be a const fn... but rust doesn't allow
/// const fn in traits for now, so it doesn't really do anything extra over
/// [`MemoryUsage`]. If/when const traits / const fn in traits are implemented /
/// stabilised, this will become a const trait / const fn, and then
/// [`mem_use_static`] will be usable in const contexts.
///
/// [`mem_use_static`]: MemoryUsageStatic::mem_use_static
pub trait MemoryUsageStatic: MemoryUsage {
	/// Statically calculates memory usage
	///
	/// This is useful for things like `&'static str` (ie. string literals),
	/// which can calculate their usage in a `const` context (`.len()` on `str` is
	/// `const`).
	///
	/// See the trait docs for information on the possibility of this fn/trait
	/// being `const` in the future.
	fn mem_use_static(&self) -> usize;
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
	const MEM_USE_CONST: usize;
}

fn _assert_mem_use_obj_safe(_: &dyn MemoryUsage) {}
fn _assert_mem_use_static_obj_safe(_: &dyn MemoryUsageStatic) {}
// const is not object safe

/// Provides an impl of [`stack_usage`](MemoryUsage::stack_usage) using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsage`].
macro_rules! mem_use_stack_size_of_impl {
	() => {
		#[inline]
		fn mem_use_stack(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`mem_use_heap`](MemoryUsage::mem_use_heap) and
/// [`mem_use_heap_incl_extra_capacity`](MemoryUsage::mem_use_heap_incl_extra_capacity)
/// for types that never allocate onto the heap, returning just 0
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsage`].
macro_rules! mem_use_heap_zero_impl {
	() => {
		#[inline]
		fn mem_use_heap(&self) -> usize {
			0
		}

		// we don't actually need to override this... but doing so means one can't
		// use this macro, and then override `heap_usage_with_extra_capacity` with
		// something else, which is just wrong
		#[inline]
		fn mem_use_heap_incl_extra_capacity(&self) -> usize {
			0
		}
	}
}

macro_rules! mem_use_total_default_impl {
	() => {
		#[inline]
		fn mem_use_total(&self) -> usize {
			self.mem_use_stack() + self.mem_use_heap()
		}
	}
}

macro_rules! mem_use_total_incl_extra_capacity_default_impl {
	() => {
		#[inline]
		fn mem_use_total_incl_extra_capacity(&self) -> usize {
			self.mem_use_stack() + self.mem_use_heap_incl_extra_capacity()
		}
	}
}

/// Provides an impl of [`memory_usage_static`](MemoryUsageStatic::memory_usage_static)
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsageStatic`].
macro_rules! mem_use_static_size_of_impl {
	() => {
		#[inline]
		fn mem_use_static(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MEMORY_USAGE`](MemoryUsageConst::MEMORY_USAGE)
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the trait [`MemoryUsageConst`].\
macro_rules! mem_use_const_size_of_impl {
	() => {
		const MEM_USE_CONST: usize = ::std::mem::size_of::<Self>();
	}
}

/// Implements [`MemoryUsage`], [`MemoryUsageStatic`], and [`MemoryUsageConst`]
/// for types that only live on the stack (stack usage is using [`size_of`],
/// and heap usage is 0)
///
/// Use by invoking in a standalone place. This macro generates the three
/// entire trait implementations
macro_rules! stack_only_impl {
	{ $($type:ident)* } => {
		$(
			impl MemoryUsage for $type {
				mem_use_stack_size_of_impl!();
				mem_use_heap_zero_impl!();
				mem_use_total_default_impl!();
				mem_use_total_incl_extra_capacity_default_impl!();
			}

			impl MemoryUsageStatic for $type {
				mem_use_static_size_of_impl!();
			}

			impl MemoryUsageConst for $type {
				mem_use_const_size_of_impl!();
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

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use_total(self)
	}

	#[inline]
	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
		T::mem_use_total_incl_extra_capacity(self)
	}

	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h T>() + T::mem_use_static(self)
	}
}

impl<'h, T: ?Sized + MemoryUsageConst> MemoryUsageConst for &'h T {
	const MEM_USE_CONST: usize = size_of::<&'h T>() + T::MEM_USE_CONST;
}

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h mut T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use_total(self)
	}

	#[inline]
	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
		T::mem_use_total_incl_extra_capacity(self)
	}

	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h mut T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h mut T>() + T::mem_use_static(self)
	}
}

impl<'h, T: ?Sized + MemoryUsageConst> MemoryUsageConst for &'h mut T {
	const MEM_USE_CONST: usize = size_of::<&'h mut T>() + T::MEM_USE_CONST;
}

impl<T: MemoryUsage, const N: usize> MemoryUsage for [T; N] {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap)
			.sum()
	}

	#[inline]
	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap_incl_extra_capacity)
			.sum()
	}

	#[inline]
	fn mem_use_total(&self) -> usize {
		self.iter()
			.map(T::mem_use_total)
			.sum()
	}

	#[inline]
	fn mem_use_total_incl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_total_incl_extra_capacity)
			.sum()
	}
}

impl<T: MemoryUsageStatic, const N: usize> MemoryUsageStatic for [T; N] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		// while loop instead of for loop, for loops aren't available in const
		// fn (yet?),getting ready to make this a const fn
		let mut usage = 0;

		let mut i = 0;
		while i < N {
			usage += T::mem_use_static(&self[i]);
			i += 1;
		}

		usage
	}
}

impl<T: MemoryUsageConst, const N: usize> MemoryUsageConst for [T; N] {
	const MEM_USE_CONST: usize = T::MEM_USE_CONST * N;
}

impl<T: MemoryUsage> MemoryUsage for [T] {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		0
	}

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.iter()
			.map(T::mem_use_total)
			.sum()
	}

	#[inline]
	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_total_incl_extra_capacity)
			.sum()
	}

	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

impl<T: MemoryUsageStatic> MemoryUsageStatic for [T] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		self.iter()
			.map(T::mem_use_static)
			.sum()
	}
}

impl<T: ?Sized> MemoryUsage for *const T {
	mem_use_stack_size_of_impl!();
	mem_use_heap_zero_impl!();
	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

impl<T: ?Sized> MemoryUsageStatic for *const T {
	mem_use_static_size_of_impl!();
}

impl<T: ?Sized> MemoryUsageConst for *const T {
	mem_use_const_size_of_impl!();
}

impl<T: ?Sized> MemoryUsage for *mut T {
	mem_use_stack_size_of_impl!();
	mem_use_heap_zero_impl!();
	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

impl<T: ?Sized> MemoryUsageStatic for *mut T {
	mem_use_static_size_of_impl!();
}

impl<T: ?Sized> MemoryUsageConst for *mut T {
	mem_use_const_size_of_impl!();
}

impl<T: MemoryUsage> MemoryUsage for Vec<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.as_slice().mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
		let extra_cap = self.capacity() - self.len();
		let mem_extra_cap = extra_cap * size_of::<T>();

		<[T]>::mem_use_total(self) + mem_extra_cap
	}

	mem_use_total_default_impl!();
	mem_use_total_incl_extra_capacity_default_impl!();
}

// TODO: do we impl static or const for vec?

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
