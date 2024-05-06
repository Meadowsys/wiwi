use std::marker::PhantomData;

// // mod consts;
// mod ctx;
// mod render;
// mod ty;
// mod util;
// mod val;
// mod var;

// pub use ctx::script::*;
// pub use ctx::state::*;
// pub use var::*;

type SingleThreadMarker = PhantomData<*const ()>;

pub mod callback_api;
