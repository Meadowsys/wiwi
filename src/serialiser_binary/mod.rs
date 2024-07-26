// TODO: this should be removed
#![allow(dead_code)]

mod error;
mod marker;

macro_rules! consts {
	{
		@impl(const) $((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(
			$(#[$meta])*
			pub const $name: $($type)+ = $value;
		)*
	};

	{
		@impl(static) $((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(
			$(#[$meta])*
			pub static $name: $($type)+ = $value;
		)*
	};

	{
		const type u8
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts!(@impl(const) $(($(#[$meta])* $name, $value, u8))*);
	};

	{
		static type &'static str
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts!(@impl(static) $(($(#[$meta])* $name, $value, &'static str))*);
	};
}
use consts;
