#![deny(unconditional_recursion)]

use std::cmp::{ Eq, Ord, PartialEq, PartialOrd };
use std::fmt::{ Debug, Display };
use std::hash::Hash;
use std::iter::{ Sum, Product };
use std::ops;

mod base;
pub use base::Base;

mod widening;
pub use widening::Widening;

mod array_conversions;
pub use array_conversions::ArrayConversions;
mod from_lossless;
pub use from_lossless::*;
mod into_lossless;
pub use into_lossless::*;

mod add_regular;
pub use add_regular::Add;
// mod add_checked;
// pub use add_checked::AddChecked;
// mod add_unchecked;
// pub use add_unchecked::AddUnchecked;
// mod add_strict;
// pub use add_strict::AddStrict;
mod add_overflowing;
pub use add_overflowing::AddOverflowing;
// mod add_saturating;
// pub use add_saturating::AddSaturating;
// mod add_wrapping;
// pub use add_wrapping::AddWrapping;
mod add_carrying;
pub use add_carrying::AddCarrying;

mod sub_regular;
pub use sub_regular::Sub;
mod sub_overflowing;
pub use sub_overflowing::SubOverflowing;
mod sub_borrowing;
pub use sub_borrowing::SubBorrowing;

mod mul_regular;
pub use mul_regular::Mul;
mod mul_unchecked;
pub use mul_unchecked::MulUnchecked;
mod mul_overflowing;
pub use mul_overflowing::MulOverflowing;
mod mul_widening;
pub use mul_widening::MulWidening;

mod div_regular;
pub use div_regular::Div;
mod div_int;
pub use div_int::DivInt;
mod div_float;
pub use div_float::DivFloat;
mod div_overflowing;
pub use div_overflowing::DivOverflowing;

// mod rem_regular;
// pub use rem_regular::Rem;
// mod shl_regular;
// pub use shl_regular::Shl;
// mod shr_regular;
// pub use shr_regular::Shr;
// mod pow_regular;
// pub use pow_regular::Pow;
// mod neg_regular;
// pub use neg_regular::Neg;
// mod not_regular;
// pub use not_regular::Not;
// mod and_regular;
// pub use and_regular::And;
// mod or_regular;
// pub use or_regular::Or;
// mod xor_regular;
// pub use xor_regular::Xor;

// TODO: ilog/2/10 sum(?) product(?)
// TODO: f16 f128
