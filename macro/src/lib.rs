// TODO: remove when more finished
#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

#[cfg(feature = "memory-usage")]
mod memory_usage;

#[cfg(feature = "memory-usage")]
#[proc_macro_derive(MemoryUsage, attributes(wiwi))]
pub fn memory_usage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	memory_usage::macro_impl(input)
}

#[cfg(feature = "int")]
mod int;

/// Internal macro only, do not use
#[cfg(feature = "int")]
#[doc(hidden)]
#[proc_macro]
pub fn define_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	int::macro_impl(input)
}

#[cfg(feature = "serialiser-binary")]
mod serialiser_binary;

#[cfg(feature = "serialiser-binary")]
#[proc_macro_derive(Serialise, attributes(wiwi))]
pub fn serialise_binary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	serialiser_binary::macro_impl(input)
}

#[cfg(feature = "unicode")]
mod unicode;
