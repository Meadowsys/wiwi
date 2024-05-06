use super::*;
use hashbrown::HashMap;

pub struct Script {
	pub(in crate::lsl) vars: HashMap<u64, Box<dyn var::VarTrait>>,
	pub(in crate::lsl) default_state: Option<state::State>,
	pub(in crate::lsl) states: HashMap<u64, state::State>,
	pub(in crate::lsl) _st: SingleThreadMarker
}

pub(super) fn new() -> Script {
	Script {
		vars: HashMap::with_capacity(32),
		default_state: None,
		states: HashMap::with_capacity(8),
		_st: PhantomData
	}
}

impl VarDeclarable for script::Script {
	fn declare_var(&mut self, var: Box<dyn var::VarTrait>) {
		let prev = self.vars.insert(var.id(), var);
		assert!(prev.is_none())
	}
}

impl Script {
	pub fn print_stdout(&self) {
		let rendered = render::render(self)
			.expect("failed to render script");
		println!("{rendered}");
	}
}
