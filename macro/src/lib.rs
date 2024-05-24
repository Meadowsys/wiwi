// TODO: remove when more finished
#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

#[cfg(feature = "memory-usage-unstable")]
mod memory_usage;

#[cfg(feature = "memory-usage-unstable")]
#[proc_macro_derive(MemoryUsage, attributes(wiwi))]
pub fn memory_usage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	memory_usage::macro_impl(input)
}
