/// Convenience macro that declares many private submodules, and glob reexports
/// all items from them
///
/// See [module docs](self) for more info
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
