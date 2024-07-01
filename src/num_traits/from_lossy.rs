use super::Base;

macro_rules! decl_from_lossy {
	{
		$(
			trait: $trait_name:ident
			fn: $fn_name:ident
			from: $from:ident
		)*
		// trait: $($trait_name:ident)*
	} => {
		$(
			/// Potentially lossy conversion from
			#[doc = concat!("`", stringify!($from), "`")]
			///
			/// This trait is implemented for all number types. This is not a guaranteed
			/// lossless conversion. It's the same as an `as` cast, but in trait form.
			pub trait $trait_name: Base {
				/// Converts the provided
				#[doc = concat!("`", stringify!($from), "`")]
				/// into Self, potentially lossily
				fn $fn_name(val: $from) -> Self;
			}

			decl_from_lossy! {
				@impl
				trait: $trait_name
				fn: $fn_name
				from: $from

				u8 u16 u32 u64 u128 usize
				i8 i16 i32 i64 i128 isize
				f32 f64
			}
		)*
	};

	{
		@impl
		trait: $trait_name:ident
		fn: $fn_name:ident
		from: $from:ident
		$($num:ident)*
	} => {
		$(
			impl $trait_name for $num {
				/// Converts the provided
				#[doc = concat!("`", stringify!($from), "`")]
				/// into
				#[doc = concat!("`", stringify!($num), "`,")]
				/// potentially lossily
				#[inline(always)]
				fn $fn_name(val: $from) -> $num { val as _ }
			}
		)*
	};
}

decl_from_lossy! {
	trait: FromU8Lossy
	fn: from_u8_lossy
	from: u8

	trait: FromU16Lossy
	fn: from_u16_lossy
	from: u16

	trait: FromU32Lossy
	fn: from_u32_lossy
	from: u32

	trait: FromU64Lossy
	fn: from_u64_lossy
	from: u64

	trait: FromU128Lossy
	fn: from_u128_lossy
	from: u128

	trait: FromUsizeLossy
	fn: from_usize_lossy
	from: usize

	trait: FromI8Lossy
	fn: from_i8_lossy
	from: i8

	trait: FromI16Lossy
	fn: from_i16_lossy
	from: i16

	trait: FromI32Lossy
	fn: from_i32_lossy
	from: i32

	trait: FromI64Lossy
	fn: from_i64_lossy
	from: i64

	trait: FromI128Lossy
	fn: from_i128_lossy
	from: i128

	trait: FromIsizeLossy
	fn: from_isize_lossy
	from: isize

	trait: FromF32Lossy
	fn: from_f32_lossy
	from: f32

	trait: FromF64Lossy
	fn: from_f64_lossy
	from: f64
}
