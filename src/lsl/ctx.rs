use super::*;
use std::cell::RefCell;

thread_local! {
	pub(super) static CONTEXT: RefCell<Vec<Context>> = {
		RefCell::new(Vec::with_capacity(16))
	}
}

pub(super) enum Context {
	Script {
		ctx: script::Script
	},
	State {
		id: Option<u64>,
		ctx: state::StateContainer
	}
}

macro_rules! check_variant {
	($var:ident, $variant:ident, $($fields:ident,)+ $msg:literal) => {
		if let Self::$variant { $($fields,)+ .. } = $var {
			($($fields),+)
		} else {
			panic!($msg);
		}
	}
}

impl Context {
	pub fn borrow_script_context(&mut self) -> &mut script::Script {
		check_variant!(self, Script, ctx, "expected script context")
	}

	pub fn unwrap_script_ctx(self) -> script::Script {
		check_variant!(self, Script, ctx, "expected script context")
	}

	pub fn unwrap_state_ctx(self) -> (Option<u64>, state::StateContainer) {
		check_variant!(self, State, id, ctx, "expected state context")
	}
}
