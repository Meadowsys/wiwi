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
	// this doesn't need to be a "base"
	// float_base
	signedness
	array_conversions

	from_lossless
	into_lossless
	from_lossy
	into_lossy

	widening
	narrowing

	count_bits

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
	rem_checked

	shl_regular
	shl_checked

	shr_regular
	shr_checked

	// pow_regular
	// pow_int
	// pow_float

	neg_regular
	neg_checked

	not_regular

	and_regular

	or_regular

	xor_regular
}

// TODO: ilog/2/10 sum(?) product(?)
// TODO: f16 f128
