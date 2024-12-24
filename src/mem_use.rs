use crate::prelude::*;
use self::private::SealedStruct;

pub trait MemUse {
	/// Unsure the use case, but let's check that these are dyn compatible for now?
	#[doc(hidden)]
	#[inline]
	fn __assert_dyn_compat(&self, _: &dyn MemUse, _: SealedStruct) {}

	/// Calculates total memory usage, including inline and indirect,
	#[inline]
	fn mem_use(&self) -> usize {
		self.mem_use_inline() + self.mem_use_indirect()
	}

	/// Calculates inline memory usage (ex. heap usage of a [`Vec`], always 24
	/// bytes on 64-bit architextures)
	fn mem_use_inline(&self) -> usize;

	/// Calculates indirect memory usage (ex. heap usage of a [`Vec`])
	fn mem_use_indirect(&self) -> usize;
}

pub trait MemUseStatic
where
	Self: MemUse
{
	/// Unsure the use case, but let's check that these are dyn compatible for now?
	#[doc(hidden)]
	#[inline]
	fn __assert_dyn_compat(&self, _: &dyn MemUseStatic, _: SealedStruct) {}

	/// Calculates the static memory usage for this value, including inline
	/// and indirect usage if applicable
	fn mem_use_static(&self) -> usize;
}

pub trait MemUseConst
where
	Self: Sized + MemUseStatic
{
	/// The constant memory usage for this type
	const MEM_USE: usize;
}

macro_rules! stack_only_impl {
	{ [$($generics:tt)*] $($ty:tt)* } => {
		impl<$($generics)*> MemUse for $($ty)* {
			#[inline]
			fn mem_use(&self) -> usize {
				size_of::<$($ty)*>()
			}

			#[inline]
			fn mem_use_inline(&self) -> usize {
				size_of::<$($ty)*>()
			}

			#[inline]
			fn mem_use_indirect(&self) -> usize {
				0
			}
		}

		impl<$($generics)*> MemUseStatic for $($ty)* {
			#[inline]
			fn mem_use_static(&self) -> usize {
				size_of::<$($ty)*>()
			}
		}

		impl<$($generics)*> MemUseConst for $($ty)* {
			const MEM_USE: usize = size_of::<$($ty)*>();
		}
	};

	{ $($ty:tt)* } => { stack_only_impl! { [] $($ty)* } };
}

stack_only_impl! { () }
stack_only_impl! { bool }
stack_only_impl! { char }
stack_only_impl! { u8 }
stack_only_impl! { u16 }
stack_only_impl! { u32 }
stack_only_impl! { u64 }
stack_only_impl! { u128 }
stack_only_impl! { usize }
stack_only_impl! { i8 }
stack_only_impl! { i16 }
stack_only_impl! { i32 }
stack_only_impl! { i64 }
stack_only_impl! { i128 }
stack_only_impl! { isize }
// stack_only_impl! { f16 }
stack_only_impl! { f32 }
stack_only_impl! { f64 }
// stack_only_impl! { f128 }
stack_only_impl! { [T: ?Sized] *const T }
stack_only_impl! { [T: ?Sized] *mut T }
stack_only_impl! { NonZero<u8> }
stack_only_impl! { NonZero<u16> }
stack_only_impl! { NonZero<u32> }
stack_only_impl! { NonZero<u64> }
stack_only_impl! { NonZero<u128> }
stack_only_impl! { NonZero<usize> }
stack_only_impl! { NonZero<i8> }
stack_only_impl! { NonZero<i16> }
stack_only_impl! { NonZero<i32> }
stack_only_impl! { NonZero<i64> }
stack_only_impl! { NonZero<i128> }
stack_only_impl! { NonZero<isize> }
stack_only_impl! { Saturating<u8> }
stack_only_impl! { Saturating<u16> }
stack_only_impl! { Saturating<u32> }
stack_only_impl! { Saturating<u64> }
stack_only_impl! { Saturating<u128> }
stack_only_impl! { Saturating<usize> }
stack_only_impl! { Saturating<i8> }
stack_only_impl! { Saturating<i16> }
stack_only_impl! { Saturating<i32> }
stack_only_impl! { Saturating<i64> }
stack_only_impl! { Saturating<i128> }
stack_only_impl! { Saturating<isize> }
stack_only_impl! { Wrapping<u8> }
stack_only_impl! { Wrapping<u16> }
stack_only_impl! { Wrapping<u32> }
stack_only_impl! { Wrapping<u64> }
stack_only_impl! { Wrapping<u128> }
stack_only_impl! { Wrapping<usize> }
stack_only_impl! { Wrapping<i8> }
stack_only_impl! { Wrapping<i16> }
stack_only_impl! { Wrapping<i32> }
stack_only_impl! { Wrapping<i64> }
stack_only_impl! { Wrapping<i128> }
stack_only_impl! { Wrapping<isize> }
stack_only_impl! { AtomicBool }
stack_only_impl! { AtomicU8 }
stack_only_impl! { AtomicU16 }
stack_only_impl! { AtomicU32 }
stack_only_impl! { AtomicU64 }
// stack_only_impl! { AtomicU128 }
stack_only_impl! { AtomicUsize }
stack_only_impl! { AtomicI8 }
stack_only_impl! { AtomicI16 }
stack_only_impl! { AtomicI32 }
stack_only_impl! { AtomicI64 }
// stack_only_impl! { AtomicI128 }
stack_only_impl! { AtomicIsize }
stack_only_impl! { [T] AtomicPtr<T> }
stack_only_impl! { [T: ?Sized] PhantomData<T> }
stack_only_impl! { PhantomPinned }

impl<T> MemUse for &T
where
	T: ?Sized + MemUse
{
	#[inline]
	fn mem_use_inline(&self) -> usize {
		size_of::<Self>()
	}

	#[inline]
	fn mem_use_indirect(&self) -> usize {
		T::mem_use(self)
	}
}

impl<T> MemUseStatic for &T
where
	T: ?Sized + MemUseStatic
{
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<Self>() + T::mem_use_static(self)
	}
}

impl<T> MemUseConst for &T
where
	T: MemUseConst
{
	const MEM_USE: usize = size_of::<Self>() + T::MEM_USE;
}

impl<T> MemUse for &mut T
where
	T: ?Sized + MemUse
{
	#[inline]
	fn mem_use_inline(&self) -> usize {
		size_of::<Self>()
	}

	#[inline]
	fn mem_use_indirect(&self) -> usize {
		T::mem_use(self)
	}
}

impl<T> MemUseStatic for &mut T
where
	T: ?Sized + MemUseStatic
{
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<Self>() + T::mem_use_static(self)
	}
}

impl<T> MemUseConst for &mut T
where
	T: MemUseConst
{
	const MEM_USE: usize = size_of::<Self>() + T::MEM_USE;
}

impl<T> MemUse for [T]
where
	T: MemUse
{
	#[inline]
	fn mem_use(&self) -> usize {
		self.iter().map(T::mem_use).sum()
	}

	#[inline]
	fn mem_use_inline(&self) -> usize {
		size_of::<T>() + self.len()
	}

	#[inline]
	fn mem_use_indirect(&self) -> usize {
		self.iter().map(T::mem_use_indirect).sum()
	}
}

impl<T> MemUseStatic for [T]
where
	T: MemUseStatic
{
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

impl MemUse for str {
	#[inline]
	fn mem_use(&self) -> usize {
		self.len()
	}

	#[inline]
	fn mem_use_inline(&self) -> usize {
		self.len()
	}

	#[inline]
	fn mem_use_indirect(&self) -> usize {
		0
	}
}

impl MemUseStatic for str {
	#[inline]
	fn mem_use_static(&self) -> usize {
		self.len()
	}
}

// TODO: Option, tuples, Result, Vec, String, [T; N], the ranges, cells, path, osstr, cstr, rc, arc, types in this crate
// TODO: go through std's list of types lol

mod private {
	pub struct SealedStruct {
		__private: ()
	}
}
