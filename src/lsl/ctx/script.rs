use super::*;
use hashbrown::HashMap;

pub struct Script {
	// vars: HashMap<u64, >
	default_state: Option<state::State>,
	states: HashMap<u64, state::State>,
	_st: SingleThreadMarker
}

pub(super) fn new() -> Script {
	Script {
		default_state: None,
		states: HashMap::with_capacity(8),
		_st: PhantomData
	}
}
