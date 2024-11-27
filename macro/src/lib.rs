// keep synced with main lib.rs
// TODO: review these
#![allow(
	clippy::unnecessary_lazy_evaluations,
	reason = "uwu"
)]
#![warn(
	// missing_docs,
	unconditional_recursion,
	unexpected_cfgs,
	unfulfilled_lint_expectations,
	unsafe_op_in_unsafe_fn,
	unused_unsafe,
	clippy::allow_attributes,
	clippy::allow_attributes_without_reason,
	clippy::as_conversions,
	clippy::empty_docs,
	clippy::inline_always,
	clippy::items_after_statements,
	clippy::items_after_test_module,
	// clippy::missing_docs_in_private_items
	clippy::missing_inline_in_public_items,
	clippy::missing_safety_doc,
	clippy::mixed_attributes_style,
	clippy::multiple_unsafe_ops_per_block,
	clippy::trait_duplication_in_bounds,
	clippy::undocumented_unsafe_blocks,
	clippy::unnecessary_safety_comment,
	reason = "uwu"
)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod builder;

#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
	builder::builder(attr, parse_macro_input!(item)).into()
}

// #[cfg(feature = "memory-usage")]
// mod memory_usage;

// #[cfg(feature = "memory-usage")]
// #[proc_macro_derive(MemoryUsage, attributes(wiwi))]
// pub fn memory_usage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	memory_usage::macro_impl(input)
// }

// #[cfg(feature = "int")]
// mod int;

// /// Internal macro only, do not use
// #[cfg(feature = "int")]
// #[doc(hidden)]
// #[proc_macro]
// pub fn define_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	int::macro_impl(input)
// }

// #[cfg(feature = "serialiser-binary")]
// mod serialiser_binary;

// #[cfg(feature = "serialiser-binary")]
// #[proc_macro_derive(Serialise, attributes(wiwi))]
// pub fn serialise_binary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	serialiser_binary::macro_impl(input)
// }

// #[cfg(feature = "unicode")]
// mod unicode;
