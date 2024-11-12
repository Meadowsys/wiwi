use crate::prelude_std::*;

macro_rules! from {
	{
		$losslessness:literal

		$(#[$trait_meta:meta])*
		trait $trait_name:ident

		$(#[$fn_meta:meta])*
		fn $fn_name:ident($type_name:ident)
	} => {
		#[doc = concat!(
			"Trait for number types that can be converted from [`",
			stringify!($type_name),
			"`], ",
			$losslessness
		)]
		#[doc = ""]
		$(#[$trait_meta])*
		pub trait $trait_name: private::Sealed {
			#[doc = concat!("Convert from a [`", stringify!($type_name), "`] value")]
			#[doc = ""]
			$(#[$fn_meta])*
			fn $fn_name(value: $type_name) -> Self;
		}
	};

	{
		impl $trait_name:ident::$fn_name:ident($type_name:ident)
		$(
			$(#[$meta:meta])*
			$impl_type:ident
		)*
	} => {
		$(
			$(#[$meta])*
			impl $trait_name for $impl_type {
				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn $fn_name(value: $type_name) -> $impl_type { value as _ }
			}
		)*
	};
}

macro_rules! into {
	{
		$losslessness:literal

		$(#[$trait_meta:meta])*
		trait $trait_name:ident

		$(#[$fn_meta:meta])*
		fn $fn_name:ident() -> $type_name:ident
	} => {
		#[doc = concat!(
			"Trait for number types that can be converted into [`",
			stringify!($type_name),
			"`], ",
			$losslessness
		)]
		#[doc = ""]
		$(#[$trait_meta])*
		pub trait $trait_name: private::Sealed {
			#[doc = concat!("Convert into a [`", stringify!($type_name), "`] value")]
			#[doc = ""]
			$(#[$fn_meta])*
			fn $fn_name(self) -> $type_name;
		}
	};

	{
		impl $trait_name:ident::$fn_name:ident() -> $type_name:ident
		$(
			$(#[$meta:meta])*
			$impl_type:ident
		)*
	} => {
		$(
			$(#[$meta])*
			impl $trait_name for $impl_type {
				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn $fn_name(self) -> $type_name { self as _ }
			}
		)*
	};
}

from! { "losslessly" trait FromU8 fn from_u8(u8) }
from! { "losslessly" trait FromU16 fn from_u16(u16) }
from! { "losslessly" trait FromU32 fn from_u32(u32) }
from! { "losslessly" trait FromU64 fn from_u64(u64) }
from! { "losslessly" trait FromU128 fn from_u128(u128) }
from! { "losslessly" trait FromUsize fn from_usize(usize) }
from! { "losslessly" trait FromI8 fn from_i8(i8) }
from! { "losslessly" trait FromI16 fn from_i16(i16) }
from! { "losslessly" trait FromI32 fn from_i32(i32) }
from! { "losslessly" trait FromI64 fn from_i64(i64) }
from! { "losslessly" trait FromI128 fn from_i128(i128) }
from! { "losslessly" trait FromIsize fn from_isize(isize) }
from! { "losslessly" trait FromF32 fn from_f32(f32) }
from! { "losslessly" trait FromF64 fn from_f64(f64) }
from! { "losslessly" trait FromBool fn from_bool(bool) }

from! { "potentially lossily" trait FromU8Lossy fn from_u8_lossy(u8) }
from! { "potentially lossily" trait FromU16Lossy fn from_u16_lossy(u16) }
from! { "potentially lossily" trait FromU32Lossy fn from_u32_lossy(u32) }
from! { "potentially lossily" trait FromU64Lossy fn from_u64_lossy(u64) }
from! { "potentially lossily" trait FromU128Lossy fn from_u128_lossy(u128) }
from! { "potentially lossily" trait FromUsizeLossy fn from_usize_lossy(usize) }
from! { "potentially lossily" trait FromI8Lossy fn from_i8_lossy(i8) }
from! { "potentially lossily" trait FromI16Lossy fn from_i16_lossy(i16) }
from! { "potentially lossily" trait FromI32Lossy fn from_i32_lossy(i32) }
from! { "potentially lossily" trait FromI64Lossy fn from_i64_lossy(i64) }
from! { "potentially lossily" trait FromI128Lossy fn from_i128_lossy(i128) }
from! { "potentially lossily" trait FromIsizeLossy fn from_isize_lossy(isize) }
from! { "potentially lossily" trait FromF32Lossy fn from_f32_lossy(f32) }
from! { "potentially lossily" trait FromF64Lossy fn from_f64_lossy(f64) }
from! { "potentially lossily" trait FromBoolLossy fn from_bool_lossy(bool) }

into! { "losslessly" trait IntoU8 fn into_u8() -> u8 }
into! { "losslessly" trait IntoU16 fn into_u16() -> u16 }
into! { "losslessly" trait IntoU32 fn into_u32() -> u32 }
into! { "losslessly" trait IntoU64 fn into_u64() -> u64 }
into! { "losslessly" trait IntoU128 fn into_u128() -> u128 }
into! { "losslessly" trait IntoUsize fn into_usize() -> usize }
into! { "losslessly" trait IntoI8 fn into_i8() -> i8 }
into! { "losslessly" trait IntoI16 fn into_i16() -> i16 }
into! { "losslessly" trait IntoI32 fn into_i32() -> i32 }
into! { "losslessly" trait IntoI64 fn into_i64() -> i64 }
into! { "losslessly" trait IntoI128 fn into_i128() -> i128 }
into! { "losslessly" trait IntoIsize fn into_isize() -> isize }
into! { "losslessly" trait IntoF32 fn into_f32() -> f32 }
into! { "losslessly" trait IntoF64 fn into_f64() -> f64 }

into! { "potentially lossily" trait IntoU8Lossy fn into_u8_lossy() -> u8 }
into! { "potentially lossily" trait IntoU16Lossy fn into_u16_lossy() -> u16 }
into! { "potentially lossily" trait IntoU32Lossy fn into_u32_lossy() -> u32 }
into! { "potentially lossily" trait IntoU64Lossy fn into_u64_lossy() -> u64 }
into! { "potentially lossily" trait IntoU128Lossy fn into_u128_lossy() -> u128 }
into! { "potentially lossily" trait IntoUsizeLossy fn into_usize_lossy() -> usize }
into! { "potentially lossily" trait IntoI8Lossy fn into_i8_lossy() -> i8 }
into! { "potentially lossily" trait IntoI16Lossy fn into_i16_lossy() -> i16 }
into! { "potentially lossily" trait IntoI32Lossy fn into_i32_lossy() -> i32 }
into! { "potentially lossily" trait IntoI64Lossy fn into_i64_lossy() -> i64 }
into! { "potentially lossily" trait IntoI128Lossy fn into_i128_lossy() -> i128 }
into! { "potentially lossily" trait IntoIsizeLossy fn into_isize_lossy() -> isize }
into! { "potentially lossily" trait IntoF32Lossy fn into_f32_lossy() -> f32 }
into! { "potentially lossily" trait IntoF64Lossy fn into_f64_lossy() -> f64 }

from! {
	impl FromU8::from_u8(u8)
	u8
	u16 u32 u64 u128 usize
	i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU16::from_u16(u16)
	u16
	u32 u64 u128 usize
	i32 i64 i128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	isize
	f32 f64
}

from! {
	impl FromU32::from_u32(u32)
	u32
	u64 u128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	usize
	i64 i128
	#[cfg(target_pointer_width = "64")]
	isize
	f64
}

from! {
	impl FromU64::from_u64(u64)
	u64
	u128
	#[cfg(target_pointer_width = "64")]
	usize
	i128
}

from! {
	impl FromU128::from_u128(u128)
	u128
}

from! {
	impl FromUsize::from_usize(usize)
	#[cfg(target_pointer_width = "16")]
	u16
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	u32
	u64 u128 usize
	#[cfg(target_pointer_width = "16")]
	i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	i64
	i128
	#[cfg(target_pointer_width = "16")]
	f32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	f64
}

from! {
	impl FromI8::from_i8(i8)
	i8
	i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI16::from_i16(i16)
	i16
	i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI32::from_i32(i32)
	i32
	i64 i128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	isize
	f64
}

from! {
	impl FromI64::from_i64(i64)
	i64
	i128
	#[cfg(target_pointer_width = "64")]
	isize
}

from! {
	impl FromI128::from_i128(i128)
	i128
}

from! {
	impl FromIsize::from_isize(isize)
	#[cfg(target_pointer_width = "16")]
	i16
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	i32
	i64 i128 isize
	#[cfg(target_pointer_width = "16")]
	f32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	f64
}

from! {
	impl FromF32::from_f32(f32)
	f32 f64
}

from! {
	impl FromF64::from_f64(f64)
	f64
}

from! {
	impl FromBool::from_bool(bool)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl FromBool for f32 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool(value: bool) -> f32 { value as u32 as _ }
}

impl FromBool for f64 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool(value: bool) -> f64 { value as u64 as _ }
}

from! {
	impl FromU8Lossy::from_u8_lossy(u8)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU16Lossy::from_u16_lossy(u16)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU32Lossy::from_u32_lossy(u32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU64Lossy::from_u64_lossy(u64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU128Lossy::from_u128_lossy(u128)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromUsizeLossy::from_usize_lossy(usize)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI8Lossy::from_i8_lossy(i8)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI16Lossy::from_i16_lossy(i16)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI32Lossy::from_i32_lossy(i32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI64Lossy::from_i64_lossy(i64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI128Lossy::from_i128_lossy(i128)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromIsizeLossy::from_isize_lossy(isize)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromF32Lossy::from_f32_lossy(f32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromF64Lossy::from_f64_lossy(f64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromBoolLossy::from_bool_lossy(bool)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl FromBoolLossy for f32 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool_lossy(value: bool) -> f32 { value as u32 as _ }
}

impl FromBoolLossy for f64 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool_lossy(value: bool) -> f64 { value as u64 as _ }
}

into! {
	impl IntoU8::into_u8() -> u8
	u8
	bool
}

into! {
	impl IntoU16::into_u16() -> u16
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	bool
}

into! {
	impl IntoU32::into_u32() -> u32
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	bool
}

into! {
	impl IntoU64::into_u64() -> u64
	u8 u16 u32 u64 usize
	bool
}

into! {
	impl IntoU128::into_u128() -> u128
	u8 u16 u32 u64 u128 usize
	bool
}

into! {
	impl IntoUsize::into_usize() -> usize
	u8 u16
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u32
	#[cfg(target_pointer_width = "64")]
	u64
	usize
	bool
}

into! {
	impl IntoI8::into_i8() -> i8
	i8
	bool
}

into! {
	impl IntoI16::into_i16() -> i16
	u8
	i8 i16
	#[cfg(target_pointer_width = "16")]
	isize
	bool
}

into! {
	impl IntoI32::into_i32() -> i32
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	i8 i16 i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	isize
	bool
}

into! {
	impl IntoI64::into_i64() -> i64
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	i8 i16 i32 i64 isize
	bool
}

into! {
	impl IntoI128::into_i128() -> i128
	u8 u16 u32 u64 usize
	i8 i16 i32 i64 i128 isize
	bool
}

into! {
	impl IntoIsize::into_isize() -> isize
	u8
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u16
	#[cfg(target_pointer_width = "64")]
	u32
	i8 i16
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	i32
	#[cfg(target_pointer_width = "64")]
	i64
	isize
	bool
}

into! {
	impl IntoF32::into_f32() -> f32
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	i8 i16
	#[cfg(target_pointer_width = "16")]
	isize
	f32
}

impl IntoF32 for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	fn into_f32(self) -> f32 { self as u32 as _ }
}

into! {
	impl IntoF64::into_f64() -> f64
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	i8 i16 i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	isize
	f32 f64
}

impl IntoF64 for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn into_f64(self) -> f64 { self as u64 as _ }
}

into! {
	impl IntoU8Lossy::into_u8_lossy() -> u8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU16Lossy::into_u16_lossy() -> u16
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU32Lossy::into_u32_lossy() -> u32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU64Lossy::into_u64_lossy() -> u64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU128Lossy::into_u128_lossy() -> u128
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoUsizeLossy::into_usize_lossy() -> usize
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI8Lossy::into_i8_lossy() -> i8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI16Lossy::into_i16_lossy() -> i16
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI32Lossy::into_i32_lossy() -> i32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI64Lossy::into_i64_lossy() -> i64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI128Lossy::into_i128_lossy() -> i128
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoIsizeLossy::into_isize_lossy() -> isize
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoF32Lossy::into_f32_lossy() -> f32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

impl IntoF32Lossy for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn into_f32_lossy(self) -> f32 { self as u32 as _ }
}

into! {
	impl IntoF64Lossy::into_f64_lossy() -> f64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

impl IntoF64Lossy for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	fn into_f64_lossy(self) -> f64 { self as u64 as _ }
}

macro_rules! op_trait {
	{
		$(#[$trait_meta:meta])*
		trait $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(checked) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Option<Self::Output>;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(overflowing) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> (Self::Output, bool);
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(saturating) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(unchecked) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			unsafe fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(wrapping) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R: private::Sealed>: private::Sealed {
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		impl
		$(
			$lhs:ident: $lhs_ty:ident {
				$(
					$rhs:ident: $rhs_ty:ident {
						$(
							$(#[$cfg:meta])*
							$op:ident -> $output_ty:ident { $($body:tt)+ }
						)*
					}
				)*
			}
		)*
	} => {
		$(
			$(
				$(
					op_trait! {
						impl(helper) $op
						$(#[$cfg])*
						$lhs: $lhs_ty, $rhs: $rhs_ty -> $output_ty { $($body)+ }
					}
				)*
			)*
		)*
	};

	{ impl(helper) add $($stuff:tt)+ } => { op_trait! { impl(normal) Add::add $($stuff)+ } };

	{
		impl(normal) $trait_name:ident::$fn_name:ident
		$(#[$cfg:meta])*
		$lhs:ident: $lhs_ty:ident, $rhs:ident: $rhs_ty:ident -> $output_ty:ident { $($body:tt)+ }
	} => {
		$(#[$cfg])*
		impl $trait_name<$rhs_ty> for $lhs_ty {
			type Output = $output_ty;

			#[inline(always)]
			fn $fn_name(self, rhs: $rhs_ty) -> $output_ty {
				#[inline(always)]
				fn inner($lhs: $lhs_ty, $rhs: $rhs_ty) -> $output_ty {
					$($body)+
				}

				inner(self, rhs)
			}
		}
	};
}

op_trait! { trait Add fn add() }
op_trait! { trait Sub fn sub() }
op_trait! { trait Mul fn mul() }
op_trait! { trait Div fn div() }
op_trait! { trait Neg fn neg() }

op_trait! { trait(checked) AddChecked fn add_checked() }
op_trait! { trait(checked) SubChecked fn sub_checked() }
op_trait! { trait(checked) MulChecked fn mul_checked() }
op_trait! { trait(checked) DivChecked fn div_checked() }
op_trait! { trait(checked) NegChecked fn neg_checked() }

op_trait! { trait(overflowing) AddOverflowing fn add_overflowing() }
op_trait! { trait(overflowing) SubOverflowing fn sub_overflowing() }
op_trait! { trait(overflowing) MulOverflowing fn mul_overflowing() }
op_trait! { trait(overflowing) DivOverflowing fn div_overflowing() }
op_trait! { trait(overflowing) NegOverflowing fn neg_overflowing() }

op_trait! { trait(saturating) AddSaturating fn add_saturating() }
op_trait! { trait(saturating) SubSaturating fn sub_saturating() }
op_trait! { trait(saturating) MulSaturating fn mul_saturating() }
op_trait! { trait(saturating) DivSaturating fn div_saturating() }
op_trait! { trait(saturating) NegSaturating fn neg_saturating() }

// todo strict op?
// todo unbounded shifts?

op_trait! { trait(unchecked) AddUnchecked #[expect(clippy::missing_safety_doc, reason = "todo")] fn add_unchecked() }
op_trait! { trait(unchecked) SubUnchecked #[expect(clippy::missing_safety_doc, reason = "todo")] fn sub_unchecked() }
op_trait! { trait(unchecked) MulUnchecked #[expect(clippy::missing_safety_doc, reason = "todo")] fn mul_unchecked() }
op_trait! { trait(unchecked) DivUnchecked #[expect(clippy::missing_safety_doc, reason = "todo")] fn div_unchecked() }
op_trait! { trait(unchecked) NegUnchecked #[expect(clippy::missing_safety_doc, reason = "todo")] fn neg_unchecked() }

op_trait! { trait(wrapping) AddWrapping fn add_wrapping() }
op_trait! { trait(wrapping) SubWrapping fn sub_wrapping() }
op_trait! { trait(wrapping) MulWrapping fn mul_wrapping() }
op_trait! { trait(wrapping) DivWrapping fn div_wrapping() }
op_trait! { trait(wrapping) NegWrapping fn neg_wrapping() }

op_trait! {
	impl

	lhs: u8 {
		rhs: u8 {
			add -> u8 { lhs + rhs }
		}
		rhs: u16 {
			add -> u16 { lhs.into_u16() + rhs }
		}
		rhs: u32 {
			add -> u32 { lhs.into_u32() + rhs }
		}
		rhs: u64 {
			add -> u64 { lhs.into_u64() + rhs }
		}
		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
		}
		rhs: usize {
			add -> usize { lhs.into_usize() + rhs }
		}
		rhs: i8 {
			add -> i8 { lhs.into_i8_lossy() + rhs }
		}
		rhs: i16 {
			add -> i16 { lhs.into_i16() + rhs }
		}
		rhs: i32 {
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: u16 {
		rhs: u8 {
			add -> u16 { lhs + rhs.into_u16() }
		}
		rhs: u16 {
			add -> u16 { lhs + rhs }
		}
		rhs: u32 {
			add -> u32 { lhs.into_u32() + rhs }
		}
		rhs: u64 {
			add -> u64 { lhs.into_u64() + rhs }
		}
		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
		}
		rhs: usize {
			add -> usize { lhs.into_usize() + rhs }
		}
		rhs: i8 {
			add -> i16 { lhs.into_i16_lossy() + rhs.into_i16() }
		}
		rhs: i16 {
			add -> i16 { lhs.into_i16_lossy() + rhs }
		}
		rhs: i32 {
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "16")]
			add -> isize { lhs.into_isize_lossy() + rhs }
		}
		rhs: isize {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: u32 {
		rhs: u8 {
			add -> u32 { lhs + rhs.into_u32() }
		}
		rhs: u16 {
			add -> u32 { lhs + rhs.into_u32() }
		}
		rhs: u32 {
			add -> u32 { lhs + rhs }
		}
		rhs: u64 {
			add -> u64 { lhs.into_u64() + rhs }
		}
		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "16")]
			add -> u32 { lhs + rhs.into_u32() }
		}
		rhs: usize {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> usize { lhs.into_usize() + rhs }
		}
		rhs: i8 {
			add -> i32 { lhs.into_i32_lossy() + rhs.into_i32() }
		}
		rhs: i16 {
			add -> i32 { lhs.into_i32_lossy() + rhs.into_i32() }
		}
		rhs: i32 {
			add -> i32 { lhs.into_i32_lossy() + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs.into_i32_lossy() + rhs.into_i32() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "32")]
			add -> isize { lhs.into_isize_lossy() + rhs }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32_lossy() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: u64 {
		rhs: u8 {
			add -> u64 { lhs + rhs.into_u64() }
		}
		rhs: u16 {
			add -> u64 { lhs + rhs.into_u64() }
		}
		rhs: u32 {
			add -> u64 { lhs + rhs.into_u64() }
		}
		rhs: u64 {
			add -> u64 { lhs + rhs }
		}
		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
		}
		rhs: usize {
			add -> u64 { lhs + rhs.into_u64() }
		}
		rhs: i8 {
			add -> i64 { lhs.into_i64_lossy() + rhs.into_i64() }
		}
		rhs: i16 {
			add -> i64 { lhs.into_i64_lossy() + rhs.into_i64() }
		}
		rhs: i32 {
			add -> i64 { lhs.into_i64_lossy() + rhs.into_i64() }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64_lossy() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> i64 { lhs.into_i64_lossy() + rhs.into_i64() }
		}
		rhs: f32 {
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: u128 {
		rhs: u8 {
			add -> u128 { lhs + rhs.into_u128() }
		}
		rhs: u16 {
			add -> u128 { lhs + rhs.into_u128() }
		}
		rhs: u32 {
			add -> u128 { lhs + rhs.into_u128() }
		}
		rhs: u64 {
			add -> u128 { lhs + rhs.into_u128() }
		}
		rhs: u128 {
			add -> u128 { lhs + rhs }
		}
		rhs: usize {
			add -> u128 { lhs + rhs.into_u128() }
		}
		rhs: i8 {
			add -> i128 { lhs.into_i128_lossy() + rhs.into_i128() }
		}
		rhs: i16 {
			add -> i128 { lhs.into_i128_lossy() + rhs.into_i128() }
		}
		rhs: i32 {
			add -> i128 { lhs.into_i128_lossy() + rhs.into_i128() }
		}
		rhs: i64 {
			add -> i128 { lhs.into_i128_lossy() + rhs.into_i128() }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128_lossy() + rhs }
		}
		rhs: isize {
			add -> i128 { lhs.into_i128_lossy() + rhs.into_i128() }
		}
		rhs: f32 {
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: usize {
		rhs: u8 {
			add -> usize { lhs + rhs.into_usize() }
		}
		rhs: u16 {
			add -> usize { lhs + rhs.into_usize() }
		}
		rhs: u32 {
			#[cfg(target_pointer_width = "16")]
			add -> u32 { lhs.into_u32() + rhs }
		}
		rhs: u32 {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> usize { lhs + rhs.into_usize() }
		}
		rhs: u64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> u64 { lhs.into_u64() + rhs }
		}
		rhs: u64 {
			#[cfg(target_pointer_width = "64")]
			add -> usize { lhs + rhs.into_usize() }
		}
		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
		}
		rhs: usize {
			add -> usize { lhs + rhs }
		}
		rhs: i8 {
			add -> isize { lhs.into_isize_lossy() + rhs.into_isize() }
		}
		rhs: i16 {
			add -> isize { lhs.into_isize_lossy() + rhs.into_isize() }
		}
		rhs: i32 {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i32 {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs.into_isize_lossy() + rhs.into_isize() }
		}
		rhs: i64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i64 {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs.into_isize_lossy() + rhs.into_isize() }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> isize { lhs.into_isize_lossy() + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "16")]
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "32")]
			add -> f32 { lhs.into_f32_lossy() + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> f64 { lhs.into_f64() + rhs }
		}
		rhs: f64 {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: i8 {
		rhs: u8 {
			add -> i8 { lhs + rhs.into_i8_lossy() }
		}
		rhs: u16 {
			add -> i16 { lhs.into_i16() + rhs.into_i16_lossy() }
		}
		rhs: u32 {
			add -> i32 { lhs.into_i32() + rhs.into_i32_lossy() }
		}
		rhs: u64 {
			add -> i64 { lhs.into_i64() + rhs.into_i64_lossy() }
		}
		rhs: u128 {
			add -> i128 { lhs.into_i128() + rhs.into_i128_lossy() }
		}
		rhs: usize {
			add -> isize { lhs.into_isize() + rhs.into_isize_lossy() }
		}
		rhs: i8 {
			add -> i8 { lhs + rhs }
		}
		rhs: i16 {
			add -> i16 { lhs.into_i16() + rhs }
		}
		rhs: i32 {
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: i16 {
		rhs: u8 {
			add -> i16 { lhs + rhs.into_i16() }
		}
		rhs: u16 {
			add -> i16 { lhs + rhs.into_i16_lossy() }
		}
		rhs: u32 {
			add -> i32 { lhs.into_i32() + rhs.into_i32_lossy() }
		}
		rhs: u64 {
			add -> i64 { lhs.into_i64() + rhs.into_i64_lossy() }
		}
		rhs: u128 {
			add -> i128 { lhs.into_i128() + rhs.into_i128_lossy() }
		}
		rhs: usize {
			add -> isize { lhs.into_isize() + rhs.into_isize_lossy() }
		}
		rhs: i8 {
			add -> i16 { lhs + rhs.into_i16() }
		}
		rhs: i16 {
			add -> i16 { lhs + rhs }
		}
		rhs: i32 {
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: i32 {
		rhs: u8 {
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: u16 {
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: u32 {
			add -> i32 { lhs + rhs.into_i32_lossy() }
		}
		rhs: u64 {
			add -> i64 { lhs.into_i64() + rhs.into_i64_lossy() }
		}
		rhs: u128 {
			add -> i128 { lhs.into_i128() + rhs.into_i128_lossy() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: usize {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs.into_isize() + rhs.into_isize_lossy() }
		}
		rhs: i8 {
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: i16 {
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: i32 {
			add -> i32 { lhs + rhs }
		}
		rhs: i64 {
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs + rhs.into_i32() }
		}
		rhs: isize {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f32 { lhs.into_f32_lossy() + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: i64 {
		rhs: u8 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: u16 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: u32 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: u64 {
			add -> i64 { lhs + rhs.into_i64_lossy() }
		}
		rhs: u128 {
			add -> i128 { lhs.into_i128() + rhs.into_i128_lossy() }
		}
		rhs: usize {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs.into_isize() + rhs.into_isize_lossy()}
		}
		rhs: i8 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: i16 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: i32 {
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: i64 {
			add -> i64 { lhs + rhs }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> i64 { lhs + rhs.into_i64() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs.into_isize() + rhs }
		}
		rhs: f32 {
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: i128 {
		rhs: u8 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: u16 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: u32 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: u64 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: u128 {
			add -> i128 { lhs + rhs.into_i128_lossy() }
		}
		rhs: usize {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: i8 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: i16 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: i32 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: i64 {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: i128 {
			add -> i128 { lhs + rhs }
		}
		rhs: isize {
			add -> i128 { lhs + rhs.into_i128() }
		}
		rhs: f32 {
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: isize {
		rhs: u8 {
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: u16 {
			#[cfg(target_pointer_width = "16")]
			add -> isize { lhs + rhs.into_isize_lossy() }
		}
		rhs: u16 {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: u32 {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs.into_i32() + rhs.into_i32_lossy() }
		}
		rhs: u32 {
			#[cfg(target_pointer_width = "32")]
			add -> isize { lhs + rhs.into_isize_lossy() }
		}
		rhs: u32 {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: u64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> i64 { lhs.into_isize() + rhs.into_i64_lossy() }
		}
		rhs: u64 {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs + rhs.into_isize_lossy() }
		}
		rhs: u128 {
			add -> i128 { lhs.into_i128() + rhs.into_i128_lossy() }
		}
		rhs: usize {
			add -> isize { lhs + rhs.into_isize_lossy() }
		}
		rhs: i8 {
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: i16 {
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: i32 {
			#[cfg(target_pointer_width = "16")]
			add -> i32 { lhs.into_i32() + rhs }
		}
		rhs: i32 {
			#[cfg(any(
				target_pointer_width = "32",
				target_pointer_width = "64"
			))]
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: i64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> i64 { lhs.into_i64() + rhs }
		}
		rhs: i64 {
			#[cfg(target_pointer_width = "64")]
			add -> isize { lhs + rhs.into_isize() }
		}
		rhs: i128 {
			add -> i128 { lhs.into_i128() + rhs }
		}
		rhs: isize {
			add -> isize { lhs + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "16")]
			add -> f32 { lhs.into_f32() + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "32")]
			add -> f32 { lhs.into_f32_lossy() + rhs }
		}
		rhs: f32 {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64_lossy() + rhs.into_f64() }
		}
		rhs: f64 {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> f64 { lhs.into_f64() + rhs }
		}
		rhs: f64 {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64_lossy() + rhs }
		}
	}

	lhs: f32 {
		rhs: u8 {
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: u16 {
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: u32 {
			add -> f32 { lhs + rhs.into_f32_lossy() }
		}
		rhs: u64 {
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: u128 {
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "16")]
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "32")]
			add -> f32 { lhs + rhs.into_f32_lossy() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: i8 {
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: i16 {
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: i32 {
			add -> f32 { lhs + rhs.into_f32_lossy() }
		}
		rhs: i64 {
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: i128 {
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "16")]
			add -> f32 { lhs + rhs.into_f32() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "32")]
			add -> f32 { lhs + rhs.into_f32_lossy() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs.into_f64() + rhs.into_f64_lossy() }
		}
		rhs: f32 {
			add -> f32 { lhs + rhs }
		}
		rhs: f64 {
			add -> f64 { lhs.into_f64() + rhs }
		}
	}

	lhs: f64 {
		rhs: u8 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: u16 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: u32 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: u64 {
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: u128 {
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: usize {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: usize {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: i8 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: i16 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: i32 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: i64 {
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: i128 {
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: isize {
			#[cfg(any(
				target_pointer_width = "16",
				target_pointer_width = "32"
			))]
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: isize {
			#[cfg(target_pointer_width = "64")]
			add -> f64 { lhs + rhs.into_f64_lossy() }
		}
		rhs: f32 {
			add -> f64 { lhs + rhs.into_f64() }
		}
		rhs: f64 {
			add -> f64 { lhs + rhs }
		}
	}
}

/// notouchie
mod private {
	use super::*;

	/// notouchie
	pub trait Sealed: Sized {}

	macro_rules! impl_sealed {
		{ $($type:ident)* } => {
			$(
				impl Sealed for $type {}
			)*
		}
	}

	impl_sealed! {
		u8 u16 u32 u64 u128 usize
		i8 i16 i32 i64 i128 isize
		f32 f64
		bool
	}

	impl<T: Sealed> Sealed for &T {}
	impl<T: Sealed> Sealed for &mut T {}
}
