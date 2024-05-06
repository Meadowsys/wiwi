use super::*;
use hashbrown::HashMap;

pub struct Script {
	pub(in crate::lsl) vars: HashMap<u64, Box<dyn var::VarTrait>>,
	pub(in crate::lsl) default_state: Option<state::StateContainer>,
	pub(in crate::lsl) states: HashMap<u64, state::StateContainer>,
	pub(in crate::lsl) _st: SingleThreadMarker
}

pub fn script(f: impl FnOnce()) -> Script {
	enter();
	f();
	exit()
}

fn enter() {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		1.. => { panic!("cannot start script within another script") }
		0 => {
			ctx.push(Ctx::Script {
				ctx: Script::new()
			})
		}
	})
}

fn exit() -> script::Script {
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

impl VarDeclarable for script::Script {
	fn declare_var(&mut self, var: Box<dyn var::VarTrait>) {
		let prev = self.vars.insert(var.id(), var);
		assert!(prev.is_none())
	}
}

impl Script {
	fn new() -> Self {
		Self {
			vars: HashMap::with_capacity(32),
			default_state: None,
			states: HashMap::with_capacity(8),
			_st: PhantomData
		}
	}

	pub fn print_stdout(&self) {
		let rendered = render::render(self)
			.expect("failed to render script");
		println!("{rendered}");
	}
}
