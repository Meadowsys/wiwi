#![deny(unconditional_recursion)]

//! Number operations exposed as traits, enabling code to be written genericly
//! over number types
//!
//! Think the [`Add`](std::ops::Add) trait, except for things like
//! [overflowing addition](AddOverflowing) as well

use std::cmp::{ Eq, Ord, PartialEq, PartialOrd };
use std::fmt::{ Debug, Display };
use std::hash::Hash;
use std::iter::{ Sum, Product };
use std::ops;

macro_rules! decl_mod {
	{ $($mod_name:ident)* } => {
		$(
			mod $mod_name;
			pub use $mod_name::*;
		)*
	}
}

decl_mod! {
	base

	widening
	array_conversions
	from_lossless
	into_lossless
	signedness

	add_regular
	add_checked
	// add_unchecked
	// add_strict
	add_overflowing
	// add_saturating
	// add_wrapping
	add_carrying

	sub_regular
	sub_checked
	sub_overflowing
	sub_borrowing

	mul_regular
	mul_checked
	mul_unchecked
	mul_overflowing
	mul_widening

	div_regular
	div_checked
	div_int
	div_float
	div_overflowing

	rem_regular

	shl_regular

	shr_regular

	// pow_regular
	// pow_int
	// pow_float

	neg_regular

	not_regular

	and_regular

	// or_regular
	// xor_regular
}

// TODO: ilog/2/10 sum(?) product(?)
// TODO: f16 f128
