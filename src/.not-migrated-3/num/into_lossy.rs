use super::Base;

macro_rules! decl_into_lossy {
	{
		$(
			trait: $trait_name:ident
			fn: $fn_name:ident -> $into:ident
		)*
		// trait: $($trait_name:ident)*
	} => {
		$(
			/// Potentially lossy conversion into
			#[doc = concat!("`", stringify!($into), "`")]
			///
			/// This trait is implemented for all number types. This is not a guaranteed
			/// lossless conversion. It's the same as an `as` cast, but in trait form.
			pub trait $trait_name: Base {
				/// converts `self` into
				#[doc = concat!("`", stringify!($into), "`,")]
				/// potentially lossily
				fn $fn_name(self) -> $into;
			}

			decl_into_lossy! {
				@impl
				trait: $trait_name
				fn: $fn_name -> $into

				u8 u16 u32 u64 u128 usize
				i8 i16 i32 i64 i128 isize
				f32 f64
			}
		)*
	};

	{
		@impl
		trait: $trait_name:ident
		fn: $fn_name:ident -> $into:ident
		$($num:ident)*
	} => {
		$(
			impl $trait_name for $num {
				#[expect(clippy::as_conversions)]
				#[inline(always)]
				fn $fn_name(self) -> $into { self as _ }
			}
		)*
	};
}

decl_into_lossy! {
	trait: IntoU8Lossy
	fn: into_u8_lossy -> u8

	trait: IntoU16Lossy
	fn: into_u16_lossy -> u16

	trait: IntoU32Lossy
	fn: into_u32_lossy -> u32

	trait: IntoU64Lossy
	fn: into_u64_lossy -> u64

	trait: IntoU128Lossy
	fn: into_u128_lossy -> u128

	trait: IntoUsizeLossy
	fn: into_usize_lossy -> usize

	trait: IntoI8Lossy
	fn: into_i8_lossy -> i8

	trait: IntoI16Lossy
	fn: into_i16_lossy -> i16

	trait: IntoI32Lossy
	fn: into_i32_lossy -> i32

	trait: IntoI64Lossy
	fn: into_i64_lossy -> i64

	trait: IntoI128Lossy
	fn: into_i128_lossy -> i128

	trait: IntoIsizeLossy
	fn: into_isize_lossy -> isize

	trait: IntoF32Lossy
	fn: into_f32_lossy -> f32

	trait: IntoF64Lossy
	fn: into_f64_lossy -> f64
}
