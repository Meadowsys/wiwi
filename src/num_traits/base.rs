use std::mem::size_of;

pub trait Base: Sized {
	const MIN: Self;
	const MAX: Self;
	const ZERO: Self;
	const ONE: Self;
	const BITS: usize = size_of::<Self>();
	const BYTES: usize = size_of::<Self>();
}
