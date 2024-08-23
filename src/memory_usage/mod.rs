// pub use wiwiwiwiwi::MemoryUsage;

// todo: write doc comments on the _impl fns themselves_ describing the logic inside

/// Trait for types that can report its current total memory usage
pub trait MemoryUsage {
	/// Gets the amount of memory this value uses on the "stack"
	///
	/// "Stack" here is defined vaguely as the amount of memory that a value
	/// itself takes up, excluding any indirections. Self-explanatorily, for
	/// sized values, this is the same as [`size_of`], (ie. `u64` would return
	/// 8 (bytes), and `&str` would return 16 (bytes) on a 64-bit platform)).
	/// However... this trait and method can be implemented on unsized types,
	/// such as `[T]` (not `&[T]`). For `[T]` this would be the len of the slice
	/// multiplied by the size of T (so does not include the reference size).
	fn mem_use_stack(&self) -> usize;

	/// Gets the amount of memory this value uses on the "heap", excluding extra
	/// if applicable
	///
	/// For example, a [`Vec`] would be using its `capacity`, rather than its `len`.
	///
	/// "Heap" here is defined vaguely as the amount of memory that a value itself
	/// holds, via indirections. So this would include the contents for a [`Box`],
	/// including its total usage.
	fn mem_use_heap(&self) -> usize;

	/// Gets the amount of memory this value uses on the "heap", excluding extra
	/// if applicable
	///
	/// For example, a [`Vec`] would be using its `len`, rather than its `capacity`.
	///
	/// See [`mem_use_heap`](MemoryUsage::mem_use_heap) for information on what
	/// "heap" means here.
	fn mem_use_heap_excl_extra_capacity(&self) -> usize;

	/// Gets the total amount of memory this value has allocated, including extra
	/// if applicable
	///
	/// This is the same as adding the values from
	/// [`mem_use_stack`](MemoryUsage::mem_use_stack) and
	/// [`mem_use_heap`](MemoryUsage::mem_use_heap) together, but can be
	/// overridden if there are more efficient ways to calculate this for a
	/// specific type.
	#[inline]
	fn mem_use(&self) -> usize {
		self.mem_use_stack() + self.mem_use_heap()
	}

	/// Gets the total amount of memory this value is using, excluding extra
	/// if applicable
	///
	/// This is the same as adding the values from
	/// [`mem_use_stack`](MemoryUsage::mem_use_stack) and
	/// [`mem_use_heap`](MemoryUsage::mem_use_heap_excl_extra_capacity) together,
	/// but can be overridden if there are more efficient ways to calculate this
	/// for a specific type.
	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		self.mem_use_stack() + self.mem_use_heap_excl_extra_capacity()
	}

	/// Tells a value to remove its extra unused memory, if applicable and possible
	///
	/// This will recurse into inner types, causing all structs and nested
	/// structs to deallocate as much as possible.
	///
	/// For types where this is not applicable (ex. simple number types), this
	/// will just silently do nothing. There is a default implementation provided
	/// that is a noop.
	#[inline]
	fn shrink_extra(&mut self) {}
}

/// Trait for types that can report its total memory usage in a static context
///
/// Currently this trait is kinda... useless, but it'll become more useful once
/// traits can be `const`.
pub trait MemoryUsageStatic: MemoryUsage {
	/// Gets the static memory usage for this value
	fn mem_use_static(&self) -> usize;
}

/// Trait for types that have the same size no matter the value
///
/// For example, number types: [`u64`] will always be size 8, etc.
pub trait MemoryUsageConst: Sized + MemoryUsageStatic {
	/// The constant memory usage for this type
	const MEM_USE_CONST: usize;
}

// if possible I think `MemoryUsage` and `MemoryUsageStatic` can be
// trait objects... I'm not sure for what use case exactly, but still
const _: &dyn MemoryUsage = &0u8;
const _: &dyn MemoryUsageStatic = &0u8;
// const is not object safe

/// Provides an impl of [`MemoryUsage::mem_use_stack`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsage`].
macro_rules! mem_use_stack_size_of_impl {
	() => {
		#[inline]
		fn mem_use_stack(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MemoryUsage::mem_use_heap`] and
/// [`MemoryUsage::mem_use_heap_excl_extra_capacity`]
/// for types that never allocate onto the heap, returning just 0
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsage`].
macro_rules! mem_use_heap_zero_impl {
	() => {
		#[inline]
		fn mem_use_heap(&self) -> usize {
			0
		}

		#[inline]
		fn mem_use_heap_excl_extra_capacity(&self) -> usize {
			0
		}
	}
}

/// Provides an impl of [`MemoryUsageStatic::mem_use_static`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsageStatic`].
macro_rules! mem_use_static_size_of_impl {
	() => {
		#[inline]
		fn mem_use_static(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MemoryUsageConst::MEM_USE_CONST`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsageConst`].
macro_rules! mem_use_const_size_of_impl {
	() => {
		const MEM_USE_CONST: usize = ::std::mem::size_of::<Self>();
	}
}

/// Implements [`MemoryUsage`], [`MemoryUsageStatic`], and [`MemoryUsageConst`]
/// for types that only live on the stack (stack usage is using [`size_of`],
/// and heap usage is 0)
macro_rules! stack_only_impl {
	{ [$($generics:tt)*] $($type:tt)+ } => {
		impl<$($generics)*> MemoryUsage for $($type)+ {
			mem_use_stack_size_of_impl!();
			mem_use_heap_zero_impl!();
		}

		impl<$($generics)*> MemoryUsageStatic for $($type)+ {
			mem_use_static_size_of_impl!();
		}

		impl<$($generics)*> MemoryUsageConst for $($type)+ {
			mem_use_const_size_of_impl!();
		}
	}
}

stack_only_impl!([] ());

stack_only_impl!([] bool);
stack_only_impl!([] char);

stack_only_impl!([] u8);
stack_only_impl!([] u16);
stack_only_impl!([] u32);
stack_only_impl!([] u64);
stack_only_impl!([] u128);
stack_only_impl!([] usize);

stack_only_impl!([] i8);
stack_only_impl!([] i16);
stack_only_impl!([] i32);
stack_only_impl!([] i64);
stack_only_impl!([] i128);
stack_only_impl!([] isize);

// stack_only_impl!([] f16);
stack_only_impl!([] f32);
stack_only_impl!([] f64);
// stack_only_impl!([] f128);

stack_only_impl!([T: ?Sized] *const T);
stack_only_impl!([T: ?Sized] *mut T);

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use(*self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_excl_extra_capacity(*self)
	}

	// what to do about shrink extra? should we panic? or is no op fine?
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h T>() + T::mem_use_static(*self)
	}
}

impl<'h, T: MemoryUsageConst> MemoryUsageConst for &'h T {
	const MEM_USE_CONST: usize = size_of::<&'h T>() + T::MEM_USE_CONST;
}

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h mut T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use(*self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_excl_extra_capacity(*self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(*self)
	}
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h mut T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h mut T>() + T::mem_use_static(*self)
	}
}

impl<'h, T: MemoryUsageConst> MemoryUsageConst for &'h mut T {
	const MEM_USE_CONST: usize = size_of::<&'h mut T>() + T::MEM_USE_CONST;
}

macro_rules! tuple_impl {
	{ $($params:ident)* } => {
		tuple_impl! { @impl [$($params)*] [] }
	};

	{ @impl [] [$($done:ident)*] } => {
		// done
	};

	{ @impl [$first:ident $($remainder:ident)*] [$($done:ident)*] } => {
		tuple_impl! { @impl_real $($done)* $first }
		tuple_impl! { @impl [$($remainder)*] [$($done)* $first] }
	};

	{ @impl_real $($params:ident)* } => {
		impl<$($params: MemoryUsage,)*> MemoryUsage for ($($params,)*) {
			mem_use_stack_size_of_impl!();

			#[inline]
			fn mem_use_heap(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_heap
					$($params)*
				}
			}

			#[inline]
			fn mem_use_heap_excl_extra_capacity(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_heap_excl_extra_capacity
					$($params)*
				}
			}

			#[inline]
			fn mem_use(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use
					$($params)*
				}
			}

			#[inline]
			fn mem_use_excl_extra_capacity(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_excl_extra_capacity
					$($params)*
				}
			}
		}
	};

	{
		@impl_fn
		$self:ident
		$fn_name:ident
		$($params:ident)*
	} => {
		{
			#[allow(non_snake_case)]
			let ($($params,)*) = $self;
			let mut val = 0;
			$(val += $params.$fn_name();)*
			val
		}
	};
}

#[cfg(all(
	not(feature = "large-tuples"),
	not(feature = "omega-tuples-of-doom")
))]
tuple_impl! {
	T1 T2 T3 T4
	T5 T6 T7 T8
}

#[cfg(all(
	feature = "large-tuples",
	not(feature = "omega-tuples-of-doom")
))]
tuple_impl! {
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
tuple_impl! {
	T1 T2 T3 T4 T5 T6 T7 T8
	T9 T10 T11 T12 T13 T14 T15 T16
	T17 T18 T19 T20 T21 T22 T23 T24
	T25 T26 T27 T28 T29 T30 T31 T32
	T33 T34 T35 T36 T37 T38 T39 T40
	T41 T42 T43 T44 T45 T46 T47 T48
	T49 T50 T51 T52 T53 T54 T55 T56
	T57 T58 T59 T60 T61 T62 T63 T64
	T65 T66 T67 T68 T69 T70 T71 T72
	T73 T74 T75 T76 T77 T78 T79 T80
	T81 T82 T83 T84 T85 T86 T87 T88
	T89 T90 T91 T92 T93 T94 T95 T96
	T97 T98 T99 T100 T101 T102 T103 T104
	T105 T106 T107 T108 T109 T110 T111 T112
	T113 T114 T115 T116 T117 T118 T119 T120
	T121 T122 T123 T124 T125 T126 T127 T128
}

impl<T: MemoryUsage> MemoryUsage for [T] {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		self.iter()
			.map(T::mem_use_stack)
			.sum()
	}

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap)
			.sum()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap_excl_extra_capacity)
			.sum()
	}


	#[inline]
	fn mem_use(&self) -> usize {
		self.iter()
			.map(T::mem_use)
			.sum()
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_excl_extra_capacity)
			.sum()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.iter_mut()
			.for_each(T::shrink_extra)
	}
}

impl<T: MemoryUsageStatic> MemoryUsageStatic for [T] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		let mut i = 0;
		let mut mem_use = 0;

		while i < self.len() {
			mem_use += self[i].mem_use_static();
			i += 1;
		}

		mem_use
	}
}

impl MemoryUsage for str {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		self.len()
	}

	mem_use_heap_zero_impl!();
}

impl MemoryUsageStatic for str {
	#[inline]
	fn mem_use_static(&self) -> usize {
		self.len()
	}
}

impl<T: MemoryUsage, const N: usize> MemoryUsage for [T; N] {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		<[T]>::mem_use_heap(self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use_heap_excl_extra_capacity(self)
	}

	#[inline]
	fn mem_use(&self) -> usize {
		<[T]>::mem_use(self)
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use_excl_extra_capacity(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		<[T]>::shrink_extra(self)
	}
}

impl<T: MemoryUsageStatic, const N: usize> MemoryUsageStatic for [T; N] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		<[T]>::mem_use_static(self)
	}
}

impl<T: MemoryUsageConst, const N: usize> MemoryUsageConst for [T; N] {
	const MEM_USE_CONST: usize = T::MEM_USE_CONST * N;
}

impl<T: ?Sized + MemoryUsage> MemoryUsage for Box<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_heap(self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_heap_excl_extra_capacity(self)
	}

	#[inline]
	fn mem_use(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use(self)
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_excl_extra_capacity(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(self)
	}
}

impl<T: MemoryUsage> MemoryUsage for Vec<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		let full = <[T]>::mem_use(self);
		let empty = (self.capacity() - self.len()) * size_of::<T>();

		full + empty
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.iter_mut()
			.for_each(T::shrink_extra);
		self.shrink_to_fit();
	}
}

impl MemoryUsage for String {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.capacity()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.len()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.shrink_to_fit()
	}
}

// /// Trait for types that can calculate their actual total memory usage, not
// /// just the stack usage (ie. not just [`size_of`])
// ///
// /// # Definition of "heap"
// ///
// /// For the purposes of this trait, and its [`mem_use_heap`] and
// /// [`mem_use_heap_incl_extra_capacity`] methods... what counts as the "heap" is
// /// just "anything that's not on the stack". For example, the value a reference
// /// is pointing to is considered part of the "heap" (even if it may be just a
// /// reference to a local value), and the contents of string literals (`&'static str`),
// /// despite actually being embedded somewhere in the binary (and the str
// /// reference just points to that), also count towards the "heap".
// ///
// /// [`mem_use_heap`]: MemoryUsage::mem_use_heap
// /// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
// pub trait MemoryUsage {
// 	/// Gets the stack usage of this value
// 	///
// 	/// For [`Sized`] types, using [`size_of`] will almost always be correct.
// 	fn mem_use_stack(&self) -> usize;
//
// 	/// Gets the heap usage of this value
// 	///
// 	/// For simple values like [`u64`], this should be 0, as it does not
// 	/// dynamically allocate. For values like [`Box`] or collections like
// 	/// [`Vec`], this should be calculated.
// 	///
// 	/// For values that potentially allocate extra capacity, like [`Vec`], this
// 	/// method is for the actual memory being utilised (ie. that contain data).
// 	/// For vec this would be just its len.
// 	///
// 	/// Also see [`mem_use_heap_incl_extra_capacity`].
// 	///
// 	/// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
// 	fn mem_use_heap(&self) -> usize;
//
// 	/// Gets the heap usage of this value, including "excess capacity" and other
// 	/// related extra allocated memory
// 	///
// 	/// For values that potentially allocate extra capacity, like [`Vec`], this
// 	/// method is for _all_ memory allocated, including "excess capacity". In other
// 	/// words, _all_ memory that is currently allocated. For vec this would be its
// 	/// `capacity`.
// 	///
// 	/// Also see [`mem_use_heap`].
// 	///
// 	/// [`mem_use_heap`]: MemoryUsage::mem_use_heap
// 	fn mem_use_heap_incl_extra_capacity(&self) -> usize;
//
// 	/// Gets the total memory usage of this value, including the stack usage
// 	/// and the heap usage
// 	///
// 	/// This should be equivalent to calling [`mem_use_stack`] and [`mem_use_heap`]
// 	/// and adding them up.
// 	///
// 	/// [`mem_use_stack`]: MemoryUsage::mem_use_stack
// 	/// [`mem_use_heap`]: MemoryUsage::mem_use_heap
// 	fn mem_use_total(&self) -> usize;
//
// 	/// Gets the total memory usage of this value, including the stack usage,
// 	/// the heap usage, and the "excess capacity" if applicable.
// 	///
// 	/// This should be equivalent to calling [`mem_use_stack`] and
// 	/// [`mem_use_heap_incl_extra_capacity`] and adding them up.
// 	///
// 	/// [`mem_use_stack`]: MemoryUsage::mem_use_stack
// 	/// [`mem_use_heap_incl_extra_capacity`]: MemoryUsage::mem_use_heap_incl_extra_capacity
// 	fn mem_use_total_incl_extra_capacity(&self) -> usize;
// }
//
// /// Trait for types that can know all of their memory usage at compile time
// ///
// /// [`mem_use_static`] should be a const fn... but rust doesn't allow
// /// const fn in traits for now, so it doesn't really do anything extra over
// /// [`MemoryUsage`]. If/when const traits / const fn in traits are implemented /
// /// stabilised, this will become a const trait / const fn, and then
// /// [`mem_use_static`] will be usable in const contexts.
// ///
// /// [`mem_use_static`]: MemoryUsageStatic::mem_use_static
// pub trait MemoryUsageStatic: MemoryUsage {
// 	/// Statically calculates memory usage
// 	///
// 	/// This is useful for things like `&'static str` (ie. string literals),
// 	/// which can calculate their usage in a `const` context (`.len()` on `str` is
// 	/// `const`).
// 	///
// 	/// See the trait docs for information on the possibility of this fn/trait
// 	/// being `const` in the future.
// 	fn mem_use_static(&self) -> usize;
// }
//
// /// Trait for types that can provide their memory usage as a `const` value
// ///
// /// This is a little stricter than [`MemoryUsageStatic`], since the provided
// /// `const` would apply to all values of a type. For example, `str` can implement
// /// [`MemoryUsageStatic`] correctly, but not this trait. This is because `.len()`
// /// on str is `const`, but `.len()` requires to be called on an actual str, but
// /// that's not available here.
// pub trait MemoryUsageConst: MemoryUsageStatic {
// 	/// The constant memory usage value
// 	const MEM_USE_CONST: usize;
// }
//
// fn _assert_mem_use_obj_safe(_: &dyn MemoryUsage) {}
// fn _assert_mem_use_static_obj_safe(_: &dyn MemoryUsageStatic) {}
// // const is not object safe
//
// impl<T: ?Sized> MemoryUsage for *const T {
// 	mem_use_stack_size_of_impl!();
// 	mem_use_heap_zero_impl!();
// 	mem_use_total_default_impl!();
// 	mem_use_total_incl_extra_capacity_default_impl!();
// }
//
// impl<T: ?Sized> MemoryUsageStatic for *const T {
// 	mem_use_static_size_of_impl!();
// }
//
// impl<T: ?Sized> MemoryUsageConst for *const T {
// 	mem_use_const_size_of_impl!();
// }
//
// impl<T: ?Sized> MemoryUsage for *mut T {
// 	mem_use_stack_size_of_impl!();
// 	mem_use_heap_zero_impl!();
// 	mem_use_total_default_impl!();
// 	mem_use_total_incl_extra_capacity_default_impl!();
// }
//
// impl<T: ?Sized> MemoryUsageStatic for *mut T {
// 	mem_use_static_size_of_impl!();
// }
//
// impl<T: ?Sized> MemoryUsageConst for *mut T {
// 	mem_use_const_size_of_impl!();
// }
//
// impl<T: MemoryUsage> MemoryUsage for Vec<T> {
// 	mem_use_stack_size_of_impl!();
//
// 	#[inline]
// 	fn mem_use_heap(&self) -> usize {
// 		self.as_slice().mem_use_heap()
// 	}
//
// 	#[inline]
// 	fn mem_use_heap_incl_extra_capacity(&self) -> usize {
// 		let extra_cap = self.capacity() - self.len();
// 		let mem_extra_cap = extra_cap * size_of::<T>();
//
// 		<[T]>::mem_use_total(self) + mem_extra_cap
// 	}
//
// 	mem_use_total_default_impl!();
// 	mem_use_total_incl_extra_capacity_default_impl!();
// }

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

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_stack_only_impl {
		{ $([$test_name:ident $type:ident $val:expr])* } => {
			$(
				#[test]
				fn $test_name() {
					// MemoryUsage
					assert_eq!(<$type>::mem_use_stack(&$val), size_of::<$type>());
					assert_eq!(<$type>::mem_use_heap(&$val), 0);
					assert_eq!(<$type>::mem_use_heap_excl_extra_capacity(&$val), 0);
					assert_eq!(<$type>::mem_use(&$val), size_of::<$type>());

					// MemoryUsageStatic
					assert_eq!(<$type>::mem_use_static(&$val), size_of::<$type>());

					// MemoryUsageConst
					assert_eq!(<$type>::MEM_USE_CONST, size_of::<$type>());
				}
			)*
		}
	}

	/// a
	type Unit = ();

	test_stack_only_impl! {
		[mem_use_unit Unit ()]
		[mem_use_bool bool false]
		[mem_use_char char '0']

		[mem_use_u8 u8 0]
		[mem_use_u16 u16 0]
		[mem_use_u32 u32 0]
		[mem_use_u64 u64 0]
		[mem_use_u128 u128 0]
		[mem_use_usize usize 0]

		[mem_use_i8 i8 0]
		[mem_use_i16 i16 0]
		[mem_use_i32 i32 0]
		[mem_use_i64 i64 0]
		[mem_use_i128 i128 0]
		[mem_use_isize isize 0]

		// [mem_use_f16 f16 0.0]
		[mem_use_f32 f32 0.0]
		[mem_use_f64 f64 0.0]
		// [mem_use_f128 f128 0.0]
	}

	#[test]
	fn mem_use_vec() {
		type TestVec = Vec<i32>;
		let mut vec = TestVec::new();
		let vec_size = size_of::<TestVec>();

		assert_eq!(vec.mem_use(), vec_size);
		assert_eq!(vec.mem_use_excl_extra_capacity(), vec_size);

		vec.reserve(32);

		let mem_use = vec.mem_use();
		let mem_excl = vec.mem_use_excl_extra_capacity();
		assert!(mem_use >= vec_size + (32 * i32::MEM_USE_CONST));
		assert_eq!(mem_excl, vec_size);

		// we allocated 32, then extended with 8, this should not reallocate (8 <= 32)

		vec.extend([1, 2, 3, 4, 5, 6, 7, 8]);
		assert_eq!(vec.mem_use(), mem_use);
		assert_eq!(vec.mem_use_excl_extra_capacity(), vec_size + (8 * i32::MEM_USE_CONST));
	}

	#[test]
	fn shrink_extra_vec() {
		type TestVec = Vec<Vec<i32>>;
		let mut vec = TestVec::from([
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1)
		]);

		assert!(vec.iter().all(|v| v.capacity() > 0), "test setup failed, for some reason");

		vec.shrink_extra();
		// I went in and read std's code, if shrinking
		// to 0, it'll just deallocate
		assert!(vec.iter().all(|v| v.capacity() == 0));
	}

	#[test]
	fn shrink_extra_slice() {
		type TestVec = Vec<Vec<i32>>;
		let mut vec = TestVec::from([
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1)
		]);

		let slice = &mut *vec;

		assert!(slice.iter().all(|v| v.capacity() > 0), "test setup failed, for some reason");

		slice.shrink_extra();
		// I went in and read std's code, if shrinking
		// to 0, it'll just deallocate
		assert!(slice.iter().all(|v| v.capacity() == 0));
	}

	#[test]
	fn shrink_extra_noop() {
		let mut value = 27478;
		let orig_value = value;
		value.shrink_extra();
		assert_eq!(value, orig_value);
		// ...
	}

	#[test]
	fn brief_tuple_impl_mem_use() {
		let tuple = (1u8, 2u32, 4u16, 8u8, 16u64);
		assert_eq!(tuple.mem_use(), 16);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), 16);
		assert_eq!(tuple.mem_use_stack(), 16);
		assert_eq!(tuple.mem_use_heap(), 0);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 0);

		let size_of_usize = size_of::<usize>();

		let tuple = (3usize, &[1u8, 2, 3, 4, 5]);
		assert_eq!(tuple.mem_use(), (size_of_usize * 2) + 5);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), (size_of_usize * 2) + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize * 2);
		assert_eq!(tuple.mem_use_heap(), 5);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);

		let tuple = (3usize, &[1u8, 2, 3, 4, 5] as &[u8]);
		assert_eq!(tuple.mem_use(), (size_of_usize * 3) + 5);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), (size_of_usize * 3) + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize * 3);
		assert_eq!(tuple.mem_use_heap(), 5);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);

		let tuple = (3usize, {
			let mut vec = Vec::with_capacity(8);
			vec.extend_from_slice(&[1u8, 2, 3, 4, 5]);
			vec
		});
		let vec_size = size_of::<Vec<u8>>();
		assert_eq!(tuple.mem_use(), size_of_usize + vec_size + 8);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), size_of_usize + vec_size + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize + vec_size);
		assert_eq!(tuple.mem_use_heap(), 8);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);
	}
}
