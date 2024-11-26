#![no_implicit_prelude]

// keep synced with macro lib.rs
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

#![cfg_attr(docsrs, feature(doc_cfg))]

#![cfg_attr(all(docsrs, kiwingay), doc = "")]
#![cfg_attr(
	all(docsrs, kiwingay),
	doc = concat!(
		"These docs have been built from commit [",
		env!("KIWINGAY_DEPLOY_COMMIT_SHORT"),
		"](https://github.com/meadowsys/wiwi/commit/",
		env!("KIWINGAY_DEPLOY_COMMIT"),
		")."
	)
)]

#[cfg(target_pointer_width = "16")]
compile_error!("16-bit platforms are not supported yet (but please do file an issue if for whatever reason you do need it, I would be happy to add support!)");

#[cfg(any(doc, docsrs, kiwingay))]
#[doc = include_str!("../CHANGELOG.md")]
pub mod _changelog {}

pub mod prelude_std;

pub mod builder;
pub mod chain;
pub mod clock_timer;
pub mod encoding;
pub mod filetypes;
// pub mod function;
pub mod lazy_wrap;
pub mod macro_util;
pub mod nominal;
pub mod num;
pub mod parser;
pub mod rc;
pub mod serialiser;
pub mod string;
pub mod tuple;
pub mod vh;

// #[cfg(feature = "aoc-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "aoc-unstable")))]
// /// Utilities specific for writing solutions for [Advent of Code](https://adventofcode.com)
// #[doc = ""]
// #[doc = include_str!("./aoc/README.md")]
// pub mod aoc;

// #[cfg(feature = "augment-panic-hook")]
// #[cfg_attr(docsrs, doc(cfg(feature = "augment-panic-hook")))]
// /// Conveniently augment the panic hook (instead of replacing it), running some of your code before calling the existing one
// #[doc = ""]
// #[doc = include_str!("./augment_panic_hook/README.md")]
// pub mod augment_panic_hook;

// #[cfg(feature = "auth-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "auth-unstable")))]
// /// Lower-level(ish) utilities for writing an authentication system, in which the client password is never sent over the wire
// #[doc = ""]
// #[doc = include_str!("./auth/README.md")]
// pub mod auth;

// #[cfg(feature = "bitstream-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
// /// Encoder and decoder for a stream of bits
// #[doc = ""]
// #[doc = include_str!("./bitstream/README.md")]
// pub mod bitstream;

// #[cfg(feature = "chainer-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "chainer-unstable")))]
// /// Wrappers around common structs that provide chaining APIs (take ownership, do operation, then return ownership back with the new value)
// #[doc = ""]
// #[doc = include_str!("./chainer/README.md")]
// pub mod chainer;

// #[cfg(feature = "cli-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "cli-unstable")))]
// /// CLI arguments parser
// #[doc = ""]
// #[doc = include_str!("./cli/README.md")]
// pub mod cli;

// #[cfg(feature = "clock-timer")]
// #[cfg_attr(docsrs, doc(cfg(feature = "clock-timer")))]
// /// An interval tracking clock, yielding ticks at specified intervals and doing so for a specified period of time
// #[doc = ""]
// #[doc = include_str!("./clock_timer/README.md")]
// pub mod clock_timer;

// #[cfg(feature = "debounce")]
// #[cfg_attr(docsrs, doc(cfg(feature = "debounce")))]
// /// Delay calling a function until a specified period of time has passed since the last time it was called
// #[doc = ""]
// #[doc = include_str!("./debounce/README.md")]
// pub mod debounce;

// #[cfg(feature = "defer-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "defer-unstable")))]
// /// Defer running code until the end of the current scope or until something gets dropped
// #[doc = ""]
// #[doc = include_str!("./defer/README.md")]
// pub mod defer;

// #[cfg(feature = "export-all-submodules")]
// #[cfg_attr(docsrs, doc(cfg(feature = "export-all-submodules")))]
// /// A convenience macro for declaring many private modules, then reexporting everything within them using a glob use statement
// #[doc = ""]
// #[doc = include_str!("./export_all_submodules/README.md")]
// pub mod export_all_submodules;

// #[cfg(feature = "h")]
// #[cfg_attr(docsrs, doc(cfg(feature = "h")))]
// /// h
// #[doc = ""]
// #[doc = include_str!("./h/README.md")]
// pub mod h;

// #[cfg(feature = "hex")]
// #[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
// /// Fast hex encoder and decoder for both upper hex and lower hex
// #[doc = ""]
// #[doc = include_str!("./hex/README.md")]
// pub mod hex;

// #[cfg(feature = "id-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "id-unstable")))]
// /// ID generators that guarantee uniqueness and ordering, and of various output sizes
// #[doc = ""]
// #[doc = include_str!("./id/README.md")]
// pub mod id;

// #[cfg(feature = "int-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "int-unstable")))]
// /// Bigints (ex. u384) and uneven sized ints (ex. u15)
// #[doc = ""]
// #[doc = include_str!("./int/README.md")]
// pub mod int;

// #[cfg(feature = "iter-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "iter-unstable")))]
// /// Iterator utilities
// #[doc = ""]
// #[doc = include_str!("./iter/README.md")]
// pub mod iter;

// #[cfg(feature = "lazy-wrap")]
// #[cfg_attr(docsrs, doc(cfg(feature = "lazy-wrap")))]
// /// Wrappers around a lazily initialised value, and its initialisation function, supporting usage in static variables
// #[doc = ""]
// #[doc = include_str!("./lazy_wrap/README.md")]
// pub mod lazy_wrap;

// #[cfg(feature = "lsl-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "lsl-unstable")))]
// /// Experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol
// #[doc = ""]
// #[doc = include_str!("./lsl/README.md")]
// pub mod lsl;

// #[cfg(feature = "mcu-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "mcu-unstable")))]
// /// [Material colour utilities](https://github.com/material-foundation/material-color-utilities)
// #[doc = ""]
// #[doc = include_str!("./mcu/README.md")]
// pub mod mcu;

// #[cfg(feature = "memory-usage-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "memory-usage-unstable")))]
// /// Calculate actual memory usage of values, including derive macro (not the same as `size_of::<T>()`)
// #[doc = ""]
// #[doc = include_str!("./memory_usage/README.md")]
// pub mod memory_usage;

// #[cfg(feature = "minesweeper-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "minesweeper-unstable")))]
// /// Core logic components for minesweeper games of arbitrary size
// #[doc = ""]
// #[doc = include_str!("./minesweeper/README.md")]
// pub mod minesweeper;

// #[cfg(feature = "nominal")]
// #[cfg_attr(docsrs, doc(cfg(feature = "nominal")))]
// /// Generic newtype wrappers, for increased type safety through Rust's nominal type system
// #[doc = ""]
// #[doc = include_str!("./nominal/README.md")]
// pub mod nominal;

// #[cfg(feature = "num-traits-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "num-traits-unstable")))]
// /// More traits for numbers and their various functionality, including things like [`MulWidening`]
// #[doc = ""]
// #[doc = include_str!("./num_traits/README.md")]
// pub mod num_traits;

// #[cfg(feature = "path-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "path-unstable")))]
// /// UTF-8 only path manipulation utilities written from scratch
// #[doc = ""]
// #[doc = include_str!("./path/README.md")]
// pub mod path;

// #[cfg(feature = "rand")]
// #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
// /// Random number generators and utilities
// #[doc = ""]
// #[doc = include_str!("./rand/README.md")]
// pub mod rand;

// #[cfg(feature = "serialiser-binary-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "serialiser-binary-unstable")))]
// /// Self describing and stable binary format/serialiser, aiming for small output size
// #[doc = ""]
// #[doc = include_str!("./serialiser_binary/README.md")]
// pub mod serialiser_binary;

// #[cfg(feature = "serialiser-binary-2-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "serialiser-binary-2-unstable")))]
// /// Self describing and stable binary format/serialiser, aiming for small output size (v2)
// #[doc = ""]
// #[doc = include_str!("./serialiser_binary_2/README.md")]
// pub mod serialiser_binary_2;

// #[cfg(feature = "serialiser-text-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "serialiser-text-unstable")))]
// /// Self describing and stable text format/serialiser, aiming to be easy to read and write by hand
// #[doc = ""]
// #[doc = include_str!("./serialiser_text/README.md")]
// pub mod serialiser_text;

// #[cfg(feature = "string-pool-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "string-pool-unstable")))]
// /// Global immutable string pool and String type (à la Java)
// #[doc = ""]
// #[doc = include_str!("./string_pool/README.md")]
// pub mod string_pool;

// #[cfg(feature = "sudoku-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "sudoku-unstable")))]
// /// Sudoku related... stuff
// #[doc = ""]
// #[doc = include_str!("./sudoku/README.md")]
// pub mod sudoku;

// #[cfg(feature = "unicode-unstable")]
// #[cfg_attr(docsrs, doc(cfg(feature = "unicode-unstable")))]
// /// Implementation of the [Unicode](https://home.unicode.org) standard, including UTF-8, UTF-16, and UTF-32 strings
// #[doc = ""]
// #[doc = include_str!("./unicode/README.md")]
// pub mod unicode;

// #[cfg(feature = "with-cloned")]
// #[cfg_attr(docsrs, doc(cfg(feature = "with-cloned")))]
// /// Convenience macro for the clone-and-move pattern (yes, we're calling it that :p)
// #[doc = ""]
// #[doc = include_str!("./with_cloned/README.md")]
// pub mod with_cloned;

// #[cfg(feature = "z85")]
// #[cfg_attr(docsrs, doc(cfg(feature = "z85")))]
// /// Fast encoder and decoder for [ZeroMQ](https://zeromq.org)'s [Z85](https://rfc.zeromq.org/spec/32) format
// #[doc = ""]
// #[doc = include_str!("./z85/README.md")]
// pub mod z85;
