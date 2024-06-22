pub mod serialise;
pub mod deserialise;

pub mod buffer;
pub mod error;
pub mod marker;
pub mod number;

/// I wrote a macro to help me write macros. I think something might be wrong,
/// potentially maybe?
macro_rules! exported_match_macro {
	($name:ident: $($match:tt)+) => {
		#[macro_export]
		macro_rules! $name {
			() => { $($match)+ }
		}
		pub use $name;
	}
}
use exported_match_macro;
