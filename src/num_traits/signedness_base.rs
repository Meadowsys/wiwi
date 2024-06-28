use super::Base;

/// Additional base for signed integers with an unsigned variant
pub trait IntSignedBase: Base {
	type Unsigned: IntUnsignedBase<Signed = Self>;

	fn cast_unsigned(self) -> Self::Unsigned;
}

/// Additional base for unsigned integers with a signed variant
pub trait IntUnsignedBase: Base {
	type Signed: IntSignedBase<Unsigned = Self>;

	fn cast_signed(self) -> Self::Signed;
}

macro_rules! impl_signed {
	{ $signed:ident $unsigned:ident } => {
		impl IntSignedBase for $signed {
			type Unsigned = $unsigned;

			#[inline(always)]
			fn cast_unsigned(self) -> $unsigned { self as _ }
		}

		impl IntUnsignedBase for $unsigned {
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
