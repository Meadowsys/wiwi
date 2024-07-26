// TODO: this should be removed
#![allow(dead_code)]

mod error;
mod marker;

macro_rules! consts {
	{
		@impl(const)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub const $name: $($type)+ = $value;
			)*
		}
	};

	{
		@impl(static)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $value:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub static $name: $($type)+ = $value;
			)*
		}
	};

	{
		const type u8
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts! {
			@impl(const)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $value, u8))*
		}
	};

	{
		static type &'static str
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $value:expr)*
	} => {
		consts! {
			@impl(static)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $value, &'static str))*
		}
	};
}
use consts;
