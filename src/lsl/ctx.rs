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

pub fn with<R>(f: impl FnOnce(&mut Ctx) -> R) -> R {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		0 => { panic!("not in a script context") }
		2.. => { panic!("cannot get script context from a nested context") }
		1 => {
			let ctx = ctx.last_mut()
				.unwrap();
			f(ctx)
		}
	})
}

impl Ctx {
	pub fn borrow_script_ctx(&mut self) -> &mut script::Script {
		variant_check!(self, "expected script context", (Script, ctx))
	}
	pub fn unwrap_script_ctx(self) -> script::Script {
		variant_check!(self, "expected script context", (Script, ctx))
	}

	pub fn borrow_var_delarable(&mut self) -> &mut dyn VarDeclarable {
		variant_check! {
			self, "expected context where variables can be defined"
			(Script, ctx)
		}
	}
}

pub trait VarDeclarable {
	fn declare_var(&mut self, var: Box<dyn var::VarTrait>);
}

macro_rules! variant_check {
	{
		$var:ident, $msg:literal
		$(($variant:ident $(, $fields:ident)*))*
	} => {
		match $var {
			$(Self::$variant { $($fields,)* .. } => { $($fields),* })*
			_ => { panic!($msg) }
		}
	};

	($var:ident, $msg:literal, $(($variant:ident $(, $fields:ident)*)),*) => {
		variant_check! {
			$var, $msg
			$(($variant $(, $fields)*))*
		}
	}
}
use variant_check;
