use super::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct State {
	id: u64,
	_st: SingleThreadMarker
}

pub fn declare_state() -> State {
	let id = util::next_id();
	State { id, _st: PhantomData }
}

pub fn state_default(f: impl FnOnce()) {
	_state(None, f)
}

pub fn state(state: State, f: impl FnOnce()) {
	_state(Some(state), f)
}

fn _state(state: Option<State>, f: impl FnOnce()) {
	enter_state(state);
	f();
	exit_state(state);
}

pub(super) fn enter_state(state: Option<State>) {
	ctx::CONTEXT.with_borrow_mut(|ctx| {
		assert!(!ctx.is_empty(), "cannot enter state outside of script context");
		assert!(ctx.len() == 1, "cannot enter state in sub context");

		ctx.push(ctx::Context::State {
			id: state.map(|s| s.id),
			ctx: StateContainer {
			}
		})
	});
}

pub(super) fn exit_state(state: Option<State>) {
	ctx::CONTEXT.with_borrow_mut(|ctx| {
		assert!(!ctx.is_empty(), "cannot exit state outside of script context");
		assert!(ctx.len() != 1, "cannot exit state outside of state context");
		assert!(ctx.len() == 2, "cannot exit state in sub context");

		let (id, container) = ctx.pop()
			.unwrap()
			.unwrap_state_ctx();
		let script = ctx.last_mut()
			.unwrap()
			.borrow_script_context();

		let state = match (state, id) {
			(Some(state), Some(id)) if state.id == id => {
				script.states.entry(id).or_default()
			}

			(None, None) => {
				script.default_state.get_or_insert_with(Default::default)
			}

			_ => { panic!("cannot exit unmatched state") }
		};
		// TODO: update entry somehow lol
	});
}

#[derive(Default)]
pub(super) struct StateContainer {
	// events
}
