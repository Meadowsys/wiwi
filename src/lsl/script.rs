use super::*;
use hashbrown::HashMap;
use std::fmt;

pub struct Script {
	pub(super) global_vars: HashMap<u64, Box<dyn var::VarTrait>>,
	pub(super) default_state: Option<state::StateContainer>,
	pub(super) states: HashMap<u64, state::StateContainer>,
	_st: SingleThreadMarker
}

pub(super) fn enter_script_ctx() {
	ctx::CONTEXT.with_borrow_mut(|ctx| {
		assert!(ctx.is_empty(), "script in script");
		ctx.push(ctx::Context::Script {
			ctx: Script {
				global_vars: HashMap::with_capacity(16),
				default_state: None,
				states: HashMap::with_capacity(8),
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
		let Self { global_vars, default_state, states, _st: _ } = self;

		for (id, var) in global_vars {
			let ty_ident = var.get_ty().type_keyword();
			write!(f, "{ty_ident} var_{id}")?;

			if var.has_init() {
				var::VAR_INIT_VALUES.with_borrow(|init_val_map| {
					let val = init_val_map.get(id)
						.expect("invalid state (var says it has an init value, but can't find one)");
					write!(f, " = {val}")
				})?;
			}

			writeln!(f, ";")?;
		}

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
