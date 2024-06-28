/// Signed integers with an unsigned variant
pub trait IntSigned {
	type Unsigned: IntUnsigned<Signed = Self>;

	fn cast_unsigned(self) -> Self::Unsigned;
}

/// Unsigned integers with a signed variant
pub trait IntUnsigned {
	type Signed: IntSigned<Unsigned = Self>;

	fn cast_signed(self) -> Self::Signed;
}

macro_rules! impl_signed {
	{ $signed:ident $unsigned:ident } => {
		impl IntSigned for $signed {
			type Unsigned = $unsigned;

			#[inline(always)]
			fn cast_unsigned(self) -> $unsigned { self as _ }
		}

		impl IntUnsigned for $unsigned {
			type Signed = $signed;

			#[inline(always)]
			fn cast_signed(self) -> $signed { self as _ }
		}
	}
}

impl_signed! { i8 u8 }
impl_signed! { i16 u16 }
impl_signed! { i32 u32 }
impl_signed! { i64 u64 }
impl_signed! { i128 u128 }
impl_signed! { isize usize }
