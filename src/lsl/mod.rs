use std::marker::PhantomData;

pub mod ctx;
pub mod script;
pub mod state;
pub mod util;

type SingleThreadMarker = PhantomData<*const ()>;

pub fn new_script(f: impl FnOnce()) -> script::Script {
	script::enter_script_ctx();
	f();
	script::exit_script_ctx()
}
