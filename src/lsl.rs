#![allow(dead_code, reason = "wip")]
#![allow(
	clippy::missing_inline_in_public_items,
	reason = "performance not highest priority (for now?), ignoring inline attrs for code simplicity"
)]

extern crate hashbrown;
extern crate wiwiwiwiwiwiwiwiwiwi;
use crate::prelude::*;

use crate::chain::GenericChainConversion as _;
use self::ident::{ Ident, IdentIncrementer };
use self::private::{ SealedStruct, SealedTrait };

pub use wiwiwiwiwiwiwiwiwiwi::state;

mod ident;

fn script(f: impl FnOnce(ScriptCx)) -> Script {
	Script::new()
		.into_generic_chain()
		.with_inner(|s| f(s.cx()))
		.into_inner()
}

/// LSL primitive `integer` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_integer { __private: () }

impl Type for primitive_integer {}

impl SealedTrait for primitive_integer {}

/// LSL primitive `float` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_float { __private: () }

impl Type for primitive_float {}

impl SealedTrait for primitive_float {}

/// LSL primitive `string` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_string { __private: () }

impl Type for primitive_string {}

impl SealedTrait for primitive_string {}

/// LSL primitive `key` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_key { __private: () }

impl Type for primitive_key {}

impl SealedTrait for primitive_key {}

/// LSL primitive `vector` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_vector { __private: () }

impl Type for primitive_vector {}

impl SealedTrait for primitive_vector {}

/// LSL primitive `rotation` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_rotation { __private: () }

impl Type for primitive_rotation {}

impl SealedTrait for primitive_rotation {}

/// LSL primitive `list` type
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_list { __private: () }

impl Type for primitive_list {}

impl SealedTrait for primitive_list {}

/// LSL primitive `boolean` type
///
/// Note: LSL does not actually have a `boolean` type, where all "`boolean`"
/// values are represented by an integer. This `boolean` is represented
/// underneath by an integer, so should work just as if LSL always had
/// booleans to begin with.
#[expect(non_camel_case_types, reason = "lsl primitive")]
struct primitive_boolean { __private: () }

impl Type for primitive_boolean {}

impl SealedTrait for primitive_boolean {}

// #[expect(non_camel_case_types, reason = "lsl primitive")]
// struct prim_unit { __private: () }

struct VarContainer<T>
where
	T: Type
{
	__something: PhantomData<T>
}

impl<T> VarContainer<T>
where
	T: Type
{}

#[expect(non_camel_case_types, reason = "lsl primitive")]
type integer = VarContainer<primitive_integer>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type float = VarContainer<primitive_float>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type string = VarContainer<primitive_string>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type key = VarContainer<primitive_key>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type vector = VarContainer<primitive_vector>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type rotation = VarContainer<primitive_rotation>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type list = VarContainer<primitive_list>;

#[expect(non_camel_case_types, reason = "lsl primitive")]
type boolean = VarContainer<primitive_boolean>;


struct Script {
	ident_incrementer: IdentIncrementer
}

impl Script {
	fn new() -> Self {
		Self {
			ident_incrementer: IdentIncrementer::new()
		}
	}

	fn cx(&self) -> ScriptCx {
		ScriptCx { script: self }
	}
}

struct ScriptCx<'h> {
	script: &'h Script
}

pub trait Type {
	#[doc(hidden)]
	fn __assert_dyn_compat(&self, _: &dyn Type, _: SealedStruct) {}
}

pub trait Var<T>
where
	T: Type
{
	#[doc(hidden)]
	fn __assert_dyn_compat(&self, _: &dyn Var<T>, _: SealedStruct) {}
}

pub trait Expr<T>
where
	Self: ExprDyn,
	T: Type
{
	#[doc(hidden)]
	fn __assert_dyn_compat(&self, _: &dyn Expr<T>, _: SealedStruct) {}

	fn into_dyn(self) -> Box<dyn ExprDyn>
	where
		Self: Sized + 'static
	{
		Box::new(self)
	}
}

pub trait ExprDyn {}

mod private {
	pub struct SealedStruct {
		__private: ()
	}

	pub trait SealedTrait {}
}
