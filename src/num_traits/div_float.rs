use super::Div;

/// Floating-point division
pub trait DivFloat: Div {
	#[inline]
	fn div_float(self, rhs: Self) -> Self {
		self / rhs
	}
}

macro_rules! impl_num_trait_div_float {
	{ $($num:ident)* } => {
		$(
			impl DivFloat for $num {}
		)*
	}
}

impl_num_trait_div_float! {
	f32 f64
}
