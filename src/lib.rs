#![cfg_attr(feature = "nightly", feature(cfg_overflow_checks))]

#![cfg_attr(feature = "omega-tuples-of-doom", recursion_limit = "256")]

// TODO: review these
#![warn(
	// TODO: these following four should be deny,
	// once all unsafe blocks are commented/fixed
	unsafe_op_in_unsafe_fn,
	clippy::missing_safety_doc,
	clippy::multiple_unsafe_ops_per_block,
	clippy::undocumented_unsafe_blocks
)]
#![deny(
	unexpected_cfgs,
	clippy::mixed_attributes_style
)]

#![cfg_attr(docsrs, feature(doc_cfg))]

#![doc = include_str!("../README.md")]
#![doc = ""]
#![doc = "[wiwi.kiwin.gay]: https://wiwi.kiwin.gay"]
#![doc = ""]
#![cfg_attr(all(not(docsrs), feature = "all-unstable"), doc = "## Enabled features (`all-unstable` is enabled)")]
#![cfg_attr(all(not(docsrs), feature = "all", not(feature = "all-unstable")), doc = "## Enabled features (`all` is enabled)")]
#![cfg_attr(all(not(docsrs), not(feature = "all"), not(feature = "all-unstable")), doc = "## Enabled features")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "## Enabled features (docs.rs)")]
#![cfg_attr(kiwingay, doc = "## Enabled features (wiwi.kiwin.gay)")]
#![doc = ""]

#![cfg_attr(all(docsrs, not(kiwingay)), doc = "On docs.rs, the feature `all` is enabled. This means that all stable features,")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "and only stable features, will appear in these docs.")]
#![cfg_attr(kiwingay, doc = "On [wiwi.kiwin.gay], as many features as possible are enabled. This means")]
#![cfg_attr(kiwingay, doc = "everything, including unstable features!")]
#![cfg_attr(any(docsrs, kiwingay), doc = "")]

// ----- start autogenerated region (doc cfgs) (see gen-features script) -----

#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "aoc-unstable"), doc = "- `aoc` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "augment-panic-hook"), doc = "- `augment-panic-hook`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "auth-unstable"), doc = "- `auth` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "bitstream-unstable"), doc = "- `bitstream` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "chainer-unstable"), doc = "- `chainer` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "cli-unstable"), doc = "- `cli` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "clock-timer"), doc = "- `clock-timer`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "debounce"), doc = "- `debounce`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "defer-unstable"), doc = "- `defer` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "export-all-submodules"), doc = "- `export-all-submodules`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "gpg-unstable"), doc = "- `gpg` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "h"), doc = "- `h`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "hex"), doc = "- `hex`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "id-unstable"), doc = "- `id` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "int-unstable"), doc = "- `int` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "iter-unstable"), doc = "- `iter` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "lazy-wrap"), doc = "- `lazy-wrap`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "libassuan-unstable"), doc = "- `libassuan` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "libgpg-error-unstable"), doc = "- `libgpg-error` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "lsl-unstable"), doc = "- `lsl` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "mcu-unstable"), doc = "- `mcu` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "memory-usage-unstable"), doc = "- `memory-usage` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "minesweeper-unstable"), doc = "- `minesweeper` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "nominal"), doc = "- `nominal`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "num-traits-unstable"), doc = "- `num-traits` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "path-unstable"), doc = "- `path` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "ptr-unstable"), doc = "- `ptr` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "rand"), doc = "- `rand`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "serialiser-binary-unstable"), doc = "- `serialiser-binary` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "serialiser-text-unstable"), doc = "- `serialiser-text` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "string-pool-unstable"), doc = "- `string-pool` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "sudoku-unstable"), doc = "- `sudoku` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "unicode-unstable"), doc = "- `unicode` (unstable)")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "with-cloned"), doc = "- `with-cloned`")]
#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = "z85"), doc = "- `z85`")]

#![cfg_attr(all(
	not(feature = "aoc-unstable"),
	not(feature = "augment-panic-hook"),
	not(feature = "auth-unstable"),
	not(feature = "bitstream-unstable"),
	not(feature = "chainer-unstable"),
	not(feature = "cli-unstable"),
	not(feature = "clock-timer"),
	not(feature = "debounce"),
	not(feature = "defer-unstable"),
	not(feature = "export-all-submodules"),
	not(feature = "gpg-unstable"),
	not(feature = "h"),
	not(feature = "hex"),
	not(feature = "id-unstable"),
	not(feature = "int-unstable"),
	not(feature = "iter-unstable"),
	not(feature = "lazy-wrap"),
	not(feature = "libassuan-unstable"),
	not(feature = "libgpg-error-unstable"),
	not(feature = "lsl-unstable"),
	not(feature = "mcu-unstable"),
	not(feature = "memory-usage-unstable"),
	not(feature = "minesweeper-unstable"),
	not(feature = "nominal"),
	not(feature = "num-traits-unstable"),
	not(feature = "path-unstable"),
	not(feature = "ptr-unstable"),
	not(feature = "rand"),
	not(feature = "serialiser-binary-unstable"),
	not(feature = "serialiser-text-unstable"),
	not(feature = "string-pool-unstable"),
	not(feature = "sudoku-unstable"),
	not(feature = "unicode-unstable"),
	not(feature = "with-cloned"),
	not(feature = "z85"),
	not(docsrs)
), doc = "No features enabled! (you should probably enable something, otherwise this crate does nothing :p)")]

// ----- end autogenerated region (doc cfgs) -----

#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "If you would like to see documentation for unstable features, please use")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "`cargo doc` to build them locally (or alternatively `cargo doc --open` to")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "open the locally built docs in your default browser). Running `cargo doc`")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "will by default also generate documentation for all crates in your dependency")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "tree, including wiwi, so no need to clone the `wiwi` repository just for")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "unstable feature docs.")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "Unstable documentation for the in-development version of wiwi is available at [wiwi.kiwin.gay].")]

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
compile_error!("16-bit platforms are not supported (but please do file an issue if for whatever reason you do need it!)");

#[cfg(any(doc, docsrs, kiwingay))]
#[doc = include_str!("../CHANGELOG.md")]
pub mod _changelog {}

mod _internal;
pub mod prelude;

// ----- start autogenerated region (lib) (see gen-features script) -----

#[cfg(feature = "aoc-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "aoc-unstable")))]
pub mod aoc;

#[cfg(feature = "augment-panic-hook")]
#[cfg_attr(docsrs, doc(cfg(feature = "augment-panic-hook")))]
pub mod augment_panic_hook;

#[cfg(feature = "auth-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "auth-unstable")))]
pub mod auth;

#[cfg(feature = "bitstream-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
pub mod bitstream;

#[cfg(feature = "chainer-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "chainer-unstable")))]
pub mod chainer;

#[cfg(feature = "cli-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "cli-unstable")))]
pub mod cli;

#[cfg(feature = "clock-timer")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock-timer")))]
pub mod clock_timer;

#[cfg(feature = "debounce")]
#[cfg_attr(docsrs, doc(cfg(feature = "debounce")))]
pub mod debounce;

#[cfg(feature = "defer-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "defer-unstable")))]
pub mod defer;

#[cfg(feature = "export-all-submodules")]
#[cfg_attr(docsrs, doc(cfg(feature = "export-all-submodules")))]
pub mod export_all_submodules;

#[cfg(feature = "gpg-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "gpg-unstable")))]
pub mod gpg;

#[cfg(feature = "h")]
#[cfg_attr(docsrs, doc(cfg(feature = "h")))]
pub mod h;

#[cfg(feature = "hex")]
#[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
pub mod hex;

#[cfg(feature = "id-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-unstable")))]
pub mod id;

#[cfg(feature = "int-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "int-unstable")))]
pub mod int;

#[cfg(feature = "iter-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "iter-unstable")))]
pub mod iter;

#[cfg(feature = "lazy-wrap")]
#[cfg_attr(docsrs, doc(cfg(feature = "lazy-wrap")))]
pub mod lazy_wrap;

#[cfg(feature = "libassuan-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "libassuan-unstable")))]
pub mod libassuan;

#[cfg(feature = "libgpg-error-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "libgpg-error-unstable")))]
pub mod libgpg_error;

#[cfg(feature = "lsl-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "lsl-unstable")))]
pub mod lsl;

#[cfg(feature = "mcu-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "mcu-unstable")))]
pub mod mcu;

#[cfg(feature = "memory-usage-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory-usage-unstable")))]
pub mod memory_usage;

#[cfg(feature = "minesweeper-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "minesweeper-unstable")))]
pub mod minesweeper;

#[cfg(feature = "nominal")]
#[cfg_attr(docsrs, doc(cfg(feature = "nominal")))]
pub mod nominal;

#[cfg(feature = "num-traits-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "num-traits-unstable")))]
pub mod num_traits;

#[cfg(feature = "path-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "path-unstable")))]
pub mod path;

#[cfg(feature = "ptr-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "ptr-unstable")))]
pub mod ptr;

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub mod rand;

#[cfg(feature = "serialiser-binary-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "serialiser-binary-unstable")))]
pub mod serialiser_binary;

#[cfg(feature = "serialiser-text-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "serialiser-text-unstable")))]
pub mod serialiser_text;

#[cfg(feature = "string-pool-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "string-pool-unstable")))]
pub mod string_pool;

#[cfg(feature = "sudoku-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "sudoku-unstable")))]
pub mod sudoku;

#[cfg(feature = "unicode-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unicode-unstable")))]
pub mod unicode;

#[cfg(feature = "with-cloned")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-cloned")))]
pub mod with_cloned;

#[cfg(feature = "z85")]
#[cfg_attr(docsrs, doc(cfg(feature = "z85")))]
pub mod z85;

// ----- end autogenerated region (lib) -----
