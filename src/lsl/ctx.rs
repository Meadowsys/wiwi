use super::*;
use std::cell::RefCell;

pub(in crate::lsl) mod script;
pub(in crate::lsl) mod state;

thread_local! {
	static CONTEXT: RefCell<Vec<Ctx>> = {
		RefCell::new(Vec::with_capacity(16))
	}
}

pub enum Ctx {
	/// Root of script
	Script {
		ctx: script::Script
	},
	/// Root of state (ex. root of a `default {}` block,
	/// or `state examplestate {}` block)
	State {},
	/// Inside an event declaration in a state (ex. attach)
	Event {},
	/// Inside a function declaration
	Function {}
	// TODO: if/else statements etc
}

pub fn enter_script() {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		1.. => { panic!("cannot start script within another script") }
		0 => {
			ctx.push(Ctx::Script {
				ctx: script::new()
			})
		}
	})
}

pub fn exit_script() -> script::Script {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		0 => { panic!("cannot exit non-existent script context") }
		2.. => { panic!("cannot exit script context while in sub context") }
		1 => {
			ctx.pop()
				.unwrap()
				.unwrap_script_ctx()
		}
	})
}

impl Ctx {
	pub fn unwrap_script_ctx(self) -> script::Script {
		single_variant_check!(self, Script, ctx, "expected script context")
	}
}

macro_rules! single_variant_check {
	($var:ident, $variant:ident, $($fields:ident,)* $msg:literal) => {
		#[allow(clippy::unused_unit)]
		if let Self::$variant { $($fields,)* .. } = $var {
			($($fields),*)
		} else {
			panic!($msg);
		}
	}
}
use single_variant_check;
