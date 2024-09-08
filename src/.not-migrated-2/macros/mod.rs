extern crate core;
pub use core::{
	assert,
	assert_eq,
	assert_ne,

	debug_assert,
	debug_assert_eq,
	debug_assert_ne,

	// cfg,
	// file,
	// line,
	// column,

	// compile_error,

	// concat,
	// env,
	// format_args,
	// include,
	// include_bytes,
	// include_str,
	// matches,
	// module_path,
	// option_env,
	// stringify,
	// todo
};

// #[macro_export]
// macro_rules! panic {
// 	($($stuff:tt)*) => {
// 		core::panic!($($stuff)*)
// 	}
// }
// pub use panic;
