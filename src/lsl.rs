#![allow(dead_code, reason = "wip")]
#![allow(
	clippy::missing_inline_in_public_items,
	reason = "performance not highest priority (for now?), ignoring inline attrs for code simplicity"
)]

extern crate hashbrown;
extern crate wiwiwiwiwiwiwiwiwiwi;
use crate::prelude::*;

use self::private::{ SealedStruct, SealedTrait };

pub use wiwiwiwiwiwiwiwiwiwi::state;

/// LSL primitive `integer` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_integer { __private: () }

impl Type for prim_integer {}

impl SealedTrait for prim_integer {}

/// LSL primitive `float` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_float { __private: () }

impl Type for prim_float {}

impl SealedTrait for prim_float {}

/// LSL primitive `string` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_string { __private: () }

impl Type for prim_string {}

impl SealedTrait for prim_string {}

/// LSL primitive `key` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_key { __private: () }

impl Type for prim_key {}

impl SealedTrait for prim_key {}

/// LSL primitive `vector` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_vector { __private: () }

impl Type for prim_vector {}

impl SealedTrait for prim_vector {}

/// LSL primitive `rotation` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_rotation { __private: () }

impl Type for prim_rotation {}

impl SealedTrait for prim_rotation {}

/// LSL primitive `list` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_list { __private: () }

impl Type for prim_list {}

impl SealedTrait for prim_list {}

/// LSL primitive `boolean` type
///
/// Note: LSL does not actually have a `boolean` type, where all "`boolean`"
/// values are represented by an integer. This `boolean` is represented
/// underneath by an integer, so should work just as if LSL always had
/// booleans to begin with.
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct prim_boolean { __private: () }

impl Type for prim_boolean {}

impl SealedTrait for prim_boolean {}

// #[expect(non_camel_case_types, reason = "lsl primitive")]
// struct prim_unit { __private: () }

pub trait Type {
	#[doc(hidden)]
	fn __assert_obj_safe(_: &dyn Type, _: SealedStruct)
	where
		Self: Sized
	{}
}

mod private {
	pub struct SealedStruct {
		__private: ()
	}

	pub trait SealedTrait {}
}
