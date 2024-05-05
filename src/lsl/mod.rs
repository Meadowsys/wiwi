use std::marker::PhantomData;

// mod consts;
mod ctx;
// mod var;
// mod script;
// mod script_render;
// mod state;
mod ty;
mod util;
mod val;

pub use ctx::script::Script;

type SingleThreadMarker = PhantomData<*const ()>;

pub fn script(f: impl FnOnce()) -> Script {
	ctx::enter_script();
	f();
	ctx::exit_script()
}
