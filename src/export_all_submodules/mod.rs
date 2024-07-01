#[macro_export]
macro_rules! export_all_submodules {
	{ $($mod:ident)* } => {
		$(
			mod $mod;
			pub use self::$mod::*;
		)*
	}
}
pub use export_all_submodules;
