use super::Base;

macro_rules! decl_from_lossless {
	{
		trait_name: $trait_name:ident
		fn_name: $fn_name:ident
		from: $from:ident
		$($(#[$meta:meta])* $to:ident)*
	} => {
		/// Lossless conversion from
		#[doc = concat!("`", stringify!($from), "`")]
		///
		/// That is, this trait is implemented for a number type, when you can
		/// convert a
		#[doc = concat!("`", stringify!($from), "`")]
		/// to the number type while guaranteeing absolutely no losses for all
		/// possible values. This is not the same as an `as` cast!
		pub trait $trait_name: Base {
			/// Losslessly converts the provided
			#[doc = concat!("`", stringify!($from), "`")]
			/// into Self
			fn $fn_name(val: $from) -> Self;
		}

		$(
			$(#[$meta])*
			impl $trait_name for $to {
				/// Losslessly converts the provided
				#[doc = concat!("`", stringify!($from), "`")]
				/// into
				#[doc = concat!("`", stringify!($to), "`")]
				#[inline(always)]
				fn $fn_name(val: $from) -> $to { val as _ }
			}
		)*
	}
}

decl_from_lossless! {
	trait_name: FromU8Lossless
	fn_name: from_u8
	from: u8
	u8 u16 u32 u64 u128
	usize
	i16 i32 i64 i128
	isize
	f32 f64
}

decl_from_lossless! {
	trait_name: FromU16Lossless
	fn_name: from_u16
	from: u16
	u16 u32 u64 u128
	usize
	i32 i64 i128
	#[cfg(not(target_pointer_width = "16"))] isize
	f32 f64
}

decl_from_lossless! {
	trait_name: FromU32Lossless
	fn_name: from_u32
	from: u32
	u32 u64 u128
	#[cfg(not(target_pointer_width = "16"))] usize
	i64 i128
	#[cfg(target_pointer_width = "64")] isize
	f64
}

decl_from_lossless! {
	trait_name: FromU64Lossless
	fn_name: from_u64
	from: u64
	u64 u128
	#[cfg(target_pointer_width = "64")] usize
	i128
}

decl_from_lossless! {
	trait_name: FromU128Lossless
	fn_name: from_u128
	from: u128
	u128
}

decl_from_lossless! {
	trait_name: FromUsizeLossless
	fn_name: from_usize
	from: usize
	#[cfg(target_pointer_width = "16")] u16
	#[cfg(not(target_pointer_width = "64"))] u32
	u64 u128 usize
	#[cfg(target_pointer_width = "16")] i32
	#[cfg(not(target_pointer_width = "64"))] i64
	i128
	#[cfg(target_pointer_width = "16")] f32
	#[cfg(not(target_pointer_width = "64"))] f64
}

decl_from_lossless! {
	trait_name: FromI8Lossless
	fn_name: from_i8
	from: i8
	i8 i16 i32 i64 i128 isize
	f32 f64
}

decl_from_lossless! {
	trait_name: FromI16Lossless
	fn_name: from_i16
	from: i16
	i16 i32 i64 i128 isize
	f32 f64
}

decl_from_lossless! {
	trait_name: FromI32Lossless
	fn_name: from_i32
	from: i32
	i32 i64 i128 #[cfg(not(target_pointer_width = "16"))] isize
	f64
}

decl_from_lossless! {
	trait_name: FromI64Lossless
	fn_name: from_i64
	from: i64
	i64 i128 #[cfg(target_pointer_width = "64")] isize
}

decl_from_lossless! {
	trait_name: FromI128Lossless
	fn_name: from_i128
	from: i128
	i128
}

decl_from_lossless! {
	trait_name: FromIsizeLossless
	fn_name: from_isize
	from: isize
	#[cfg(target_pointer_width = "16")] i16
	#[cfg(not(target_pointer_width = "64"))] i32
	i64 i128 isize
	#[cfg(target_pointer_width = "16")] f32
	#[cfg(not(target_pointer_width = "64"))] f64
}

decl_from_lossless! {
	trait_name: FromF32Lossless
	fn_name: from_f32
	from: f32
	f32 f64
}

decl_from_lossless! {
	trait_name: FromF64Lossless
	fn_name: from_f64
	from: f64
	f64
}
