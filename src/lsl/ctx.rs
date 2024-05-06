use super::*;
use std::{ cell::RefCell, num::NonZeroU64 };

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
	State {
		id: Option<u64>,
		ctx: state::StateContainer
	},
	/// Inside an event declaration in a state (ex. attach)
	Event {},
	/// Inside a function declaration
	Function {}
	// TODO: if/else statements etc
}

pub fn with<R>(f: impl FnOnce(&mut Ctx) -> R) -> R {
	CONTEXT.with_borrow_mut(|ctx| {
		let ctx = ctx.last_mut()
			.expect("not in a script context");
		f(ctx)
	})
}

pub fn assert_in_script_root() {
	with(|ctx| {
		// Just want the panic if its not the right variant
		let _ = ctx.borrow_script_ctx();
	})
}

impl Ctx {
	pub fn borrow_script_ctx(&mut self) -> &mut script::Script {
		variant_check!(self, "expected script context", (Script, ctx))
	}

	pub fn unwrap_script_ctx(self) -> script::Script {
		variant_check!(self, "expected script context", (Script, ctx))
	}

	pub fn unwrap_state_ctx(self) -> (Option<u64>, state::StateContainer) {
		variant_check!(self, "expected state context", (State, id, ctx))
	}

	pub fn borrow_var_delarable(&mut self) -> &mut dyn VarDeclarable {
		variant_check! {
			self, "expected context where variables can be declared"
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
			$(Self::$variant { $($fields,)* .. } => { ($($fields),*) })*
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
