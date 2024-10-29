use crate::prelude_std::*;

macro_rules! decl_from {
	{
		lossless
		$($stuff:tt)+
	} => {
		decl_from! {
			$($stuff)+
		}
	};

	{
		lossy
		$($stuff:tt)+
	} => {
		decl_from! {
			$($stuff)+
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident
		type $from:ident
		$(
			$(#[$into_meta:meta])*
			$into:ident
		)*
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name: private::Sealed {
			$(#[$fn_meta])*
			fn $fn_name(val: $from) -> Self;
		}

		$(
			impl $trait_name for $into {
				$(#[$into_meta])*
				#[expect(clippy::as_conversions)]
				#[inline(always)]
				fn $fn_name(val: $from) -> $into { val as $into }
			}
		)*
	};
}

macro_rules! decl_into {
	{
		lossless
		$($stuff:tt)+
	} => {
		decl_into! {
			$($stuff)+
		}
	};

	{
		lossy
		$($stuff:tt)+
	} => {
		decl_into! {
			$($stuff)+
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident
		type $into:ident
		$(
			$(#[$from_meta:meta])*
			$from:ident
		)*
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name: private::Sealed {
			$(#[$fn_meta])*
			fn $fn_name(self) -> $into;
		}

		$(
			impl $trait_name for $from {
				$(#[$from_meta])*
				#[expect(clippy::as_conversions)]
				#[inline(always)]
				fn $fn_name(self) -> $into { self as $into }
			}
		)*
	};
}

decl_from! {
	lossless
	trait FromU8
	fn from_u8
	type u8
	u8 u16 u32 u64 u128 usize
	i16 i32 i64 i128 isize
	f32 f64
}

decl_from! {
	lossy
	trait FromU8Lossy
	fn from_u8_lossy
	type u8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

decl_into! {
	lossless
	trait IntoU8
	fn into_u8
	type u8
	u8
}

decl_into! {
	lossy
	trait IntoU8Lossy
	fn into_u8_lossy
	type u8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

pub trait Add<R = Self>: private::Sealed {
	type Output;
}

pub trait AddChecked<R = Self>: private::Sealed {
	type Output;
}

pub trait AddUnchecked<R = Self>: private::Sealed {
	type Output;
}

pub trait AddOverflowing<R = Self>: private::Sealed {
	type Output;
}

pub trait Sub<R = Self>: private::Sealed {
	type Output;
}

pub trait SubChecked<R = Self>: private::Sealed {
	type Output;
}

pub trait SubUnchecked<R = Self>: private::Sealed {
	type Output;
}

pub trait SubOverflowing<R = Self>: private::Sealed {
	type Output;
}

pub trait Mul<R = Self>: private::Sealed {
	type Output;
}

pub trait MulChecked<R = Self>: private::Sealed {
	type Output;
}

pub trait MulUnchecked<R = Self>: private::Sealed {
	type Output;
}

pub trait MulOverflowing<R = Self>: private::Sealed {
	type Output;
}

pub trait Div<R = Self>: private::Sealed {
	type Output;
}

pub trait DivChecked<R = Self>: private::Sealed {
	type Output;
}

pub trait DivUnchecked<R = Self>: private::Sealed {
	type Output;
}

pub trait DivOverflowing<R = Self>: private::Sealed {
	type Output;
}

/// notouchie
mod private {
	/// notouchie
	pub trait Sealed {}
}

macro_rules! impl_sealed {
	{ $($type:ident)* } => {
		$(
			impl private::Sealed for $type {}
		)*
	}
}

impl_sealed! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
