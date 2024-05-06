use super::*;
use hashbrown::HashMap;

#[derive(Clone, Copy)]
pub struct State {
	id: u64,
	_st: SingleThreadMarker
}

pub fn declare_state() -> State {
	let id = util::next_id().get();
	let _st = PhantomData;

	// I don't think you can declare states anywhere other than root
	ctx::assert_in_script_root();

	State { id, _st }
}

pub fn state(state: State, f: impl FnOnce()) {
	enter(Some(state));
	f();
	exit(Some(state));
}

pub fn state_default(f: impl FnOnce()) {
	enter(None);
	f();
	exit(None);
}

pub fn new_state(f: impl FnOnce()) -> State {
	let state = declare_state();

	enter(Some(state));
	f();
	exit(Some(state));

	state
}

pub(in crate::lsl) struct StateContainer {
	events: HashMap<u64, event::Event>,
	_st: SingleThreadMarker
}

fn enter(state: Option<State>) {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		0 => { panic!("cannot enter state context outside of script context") }
		2.. => { panic!("cannot enter state while in sub context") }
		1 => {
			ctx.push(ctx::Ctx::State {
				id: state.map(|s| s.id),
				ctx: StateContainer::new(),
			})
		}
	})
}

fn exit(state: Option<State>) {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		..=1 => { panic!("cannot exit non-existent state context") }
		3.. => { panic!("cannot exit state context while in sub context") }
		2 => {
			let (id, container) = ctx.pop()
				.unwrap()
				.unwrap_state_ctx();
			let script = ctx.last_mut()
				.unwrap()
				.borrow_script_ctx();

			let state = match (state, id) {
				(Some(state), Some(id)) if state.id == id => {
					script.states.entry(id).or_insert_with(StateContainer::new)
				}
				(None, None) => {
					script.default_state.get_or_insert_with(StateContainer::new)
				}
				_ => { panic!("cannot exit unmatched state") }
			};

			// TODO: entry updating when we have stuff to put into it heh
		}
	})
}

impl StateContainer {
	fn new() -> Self {
		Self {
			events: HashMap::with_capacity(8),
			_st: PhantomData
		}
	}
}
