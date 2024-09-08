use super::Base;

macro_rules! decl_into_lossless {
	{
		trait_name: $trait_name:ident
		fn_name: $fn_name:ident
		into: $into:ident
		$($(#[$meta:meta])* $from:ident)*
	} => {
		/// Lossless conversion into
		#[doc = concat!("`", stringify!($into), "`")]
		///
		/// That is, this trait is implemented for a number type, when it can be
		/// converted into a
		#[doc = concat!("`", stringify!($into), "`")]
		/// while guaranteeing absolutely no losses for all possible values. This
		/// is not the same as an `as` cast!
		pub trait $trait_name: Base {
			/// Losslessly converts `self` into
			#[doc = concat!("`", stringify!($into), "`")]
			fn $fn_name(self) -> $into;
		}

		$(
			$(#[$meta])*
			impl $trait_name for $from {
				#[inline(always)]
				fn $fn_name(self) -> $into { self as _ }
			}
		)*
	}
}

decl_into_lossless! {
	trait_name: IntoU8Lossless
	fn_name: into_u8
	into: u8
	u8
}

decl_into_lossless! {
	trait_name: IntoU16Lossless
	fn_name: into_u16
	into: u16
	u8 u16
	#[cfg(target_pointer_width = "16")] usize
}

decl_into_lossless! {
	trait_name: IntoU32Lossless
	fn_name: into_u32
	into: u32
	u8 u16 u32
	#[cfg(not(target_pointer_width = "64"))] usize
}

decl_into_lossless! {
	trait_name: IntoU64Lossless
	fn_name: into_u64
	into: u64
	u8 u16 u32 u64 usize
}

decl_into_lossless! {
	trait_name: IntoU128Lossless
	fn_name: into_u128
	into: u128
	u8 u16 u32 u64 u128 usize
}

decl_into_lossless! {
	trait_name: IntoUsizeLossless
	fn_name: into_usize
	into: usize
	u8 u16
	#[cfg(not(target_pointer_width = "16"))] u32
	#[cfg(target_pointer_width = "64")] u64
	usize
}

decl_into_lossless! {
	trait_name: IntoI8Lossless
	fn_name: into_i8
	into: i8
	i8
}

decl_into_lossless! {
	trait_name: IntoI16Lossless
	fn_name: into_i16
	into: i16
	u8
	i8 i16
	#[cfg(target_pointer_width = "16")] isize
}

decl_into_lossless! {
	trait_name: IntoI32Lossless
	fn_name: into_i32
	into: i32
	u8 u16
	#[cfg(target_pointer_width = "16")] usize
	i8 i16 i32
	#[cfg(not(target_pointer_width = "64"))] isize
}

decl_into_lossless! {
	trait_name: IntoI64Lossless
	fn_name: into_i64
	into: i64
	u8 u16 u32
	i8 i16 i32 i64 isize
}

decl_into_lossless! {
	trait_name: IntoI128Lossless
	fn_name: into_i128
	into: i128
	u8 u16 u32 u64 usize
	i8 i16 i32 i64 i128 isize
}

decl_into_lossless! {
	trait_name: IntoIsizeLossless
	fn_name: into_isize
	into: isize
	u8
	#[cfg(not(target_pointer_width = "16"))] u16
	#[cfg(target_pointer_width = "64")] u32
	i8 i16
	#[cfg(not(target_pointer_width = "16"))] i32
	#[cfg(target_pointer_width = "64")] i64
	isize
}

decl_into_lossless! {
	trait_name: IntoF32Lossless
	fn_name: into_f32
	into: f32
	u8 u16
	#[cfg(target_pointer_width = "16")] usize
	i8 i16
	#[cfg(target_pointer_width = "16")] isize
	f32
}

decl_into_lossless! {
	trait_name: IntoF64Lossless
	fn_name: into_f64
	into: f64
	u8 u16 u32
	#[cfg(not(target_pointer_width = "64"))] usize
	i8 i16 i32
	#[cfg(not(target_pointer_width = "64"))] isize
	f32 f64
}
