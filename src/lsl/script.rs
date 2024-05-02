use super::*;
use hashbrown::HashMap;
use std::fmt;

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

impl fmt::Display for Script {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { default_state, states, _st: _ } = self;

		// TODO: need to detect... empty states then add a dummy thing

		if let Some(state) = default_state {
			writeln!(f, "default {{")?;
			// TODO do something about this lol
			writeln!(f, "}}")?;
		} else {
			// TODO: do something about this
			// default_state.unwrapordefault()? then like the one in the bottom,
			// insert a default hook if its empty somehow
		}

		for (id, state) in states {
			writeln!(f, "state state_{id:x} {{")?;
			// TODO do something about this lol
			// might need fn abstracting to write body of script
			writeln!(f, "}}")?;
		}
		Ok(())
	}
}
