use super::*;
use hashbrown::HashMap;

pub struct Script {
	pub(super) default_state: Option<state::StateContainer>,
	pub(super) states: HashMap<u64, state::StateContainer>,
	_st: SingleThreadMarker
}

pub(super) fn enter_script_ctx() {
	ctx::CONTEXT.with_borrow_mut(|ctx| {
		assert!(ctx.is_empty(), "script in script");
		ctx.push(ctx::Context::Script {
			ctx: Script {
				default_state: None,
				states: HashMap::new(),
				_st: PhantomData
			}
		})
	})
}

pub(super) fn exit_script_ctx() -> Script {
	ctx::CONTEXT.with_borrow_mut(|ctx| {
		assert!(ctx.len() <= 1, "exiting script context while in sub context");
		ctx.pop()
			.expect("nonexistent script context")
			.unwrap_script_ctx()
	})
}
