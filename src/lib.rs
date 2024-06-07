#![cfg_attr(docsrs, feature(doc_cfg))]

#![doc = include_str!("../README.md")]
#![doc = ""]
#![cfg_attr(all(not(docsrs), feature = "all-unstable"), doc = "## Enabled features (`all-unstable` is enabled)")]
#![cfg_attr(all(not(docsrs), feature = "all", not(feature = "all-unstable")), doc = "## Enabled features (`all` is enabled)")]
#![cfg_attr(all(not(docsrs), not(feature = "all"), not(feature = "all-unstable")), doc = "## Enabled features")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "## Enabled features (docs.rs)")]
#![cfg_attr(kiwingay, doc = "## Enabled features (wiwi.kiwin.gay)")]
#![doc = ""]

#![cfg_attr(all(docsrs, not(kiwingay)), doc = "On docs.rs, the feature `all` is enabled. This means that all stable features,")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "and only stable features, will appear in these docs. These include:")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]

#![cfg_attr(kiwingay, doc = "On wiwi.kiwin.gay, as many features as possible are enabled. This means ")]
#![cfg_attr(kiwingay, doc = "everything, including unstable features! These include:")]
#![cfg_attr(kiwingay, doc = "")]

#![cfg_attr(all(feature = "auth-unstable"),         doc = "- `auth` (unstable)"        )]
#![cfg_attr(all(feature = "bigint-unstable"),       doc = "- `bigint` (unstable)"      )]
#![cfg_attr(all(feature = "bitstream-unstable"),    doc = "- `bitstream` (unstable)"   )]
#![cfg_attr(all(feature = "chainer-unstable"),      doc = "- `chainer` (unstable)"     )]
#![cfg_attr(all(feature = "clock-timer"),           doc = "- `clock-timer`"            )]
#![cfg_attr(all(feature = "debounce"),              doc = "- `debounce`"               )]
#![cfg_attr(all(feature = "defer-unstable"),        doc = "- `defer` (unstable)"       )]
#![cfg_attr(all(feature = "h"),                     doc = "- `h`"                      )]
#![cfg_attr(all(feature = "hex"),                   doc = "- `hex`"                    )]
#![cfg_attr(all(feature = "id-unstable"),           doc = "- `id` (unstable)"          )]
#![cfg_attr(all(feature = "iter-unstable"),         doc = "- `iter` (unstable)"        )]
#![cfg_attr(all(feature = "lazy-wrap"),             doc = "- `lazy-wrap`"              )]
#![cfg_attr(all(feature = "lsl-unstable"),          doc = "- `lsl` (unstable)"         )]
#![cfg_attr(all(feature = "memory-usage-unstable"), doc = "- `memory-usage` (unstable)")]
#![cfg_attr(all(feature = "minesweeper-unstable"),  doc = "- `minesweeper` (unstable)" )]
#![cfg_attr(all(feature = "nominal"),               doc = "- `nominal`"                )]
#![cfg_attr(all(feature = "path-unstable"),         doc = "- `path` (unstable)"        )]
#![cfg_attr(all(feature = "serialiser-unstable"),   doc = "- `serialiser` (unstable)"  )]
#![cfg_attr(all(feature = "string-pool-unstable"),  doc = "- `string-pool` (unstable)" )]
#![cfg_attr(all(feature = "sudoku-unstable"),       doc = "- `sudoku` (unstable)"      )]
#![cfg_attr(all(feature = "to-maybeuninit"),        doc = "- `to-maybeuninit`"         )]
#![cfg_attr(all(feature = "z85"),                   doc = "- `z85`"                    )]

#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "If you would like to see documentation for unstable features, please use")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "`cargo doc` to build them locally (or alternatively `cargo doc --open` to")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "open the locally built docs in your default browser). Running `cargo doc`")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "will by default also generate documentation for all crates that yours depends")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "on, so no need to clone the `wiwi` repository just for unstable feature docs.")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "Unstable documentation for the in-development version of wiwi is available at [wiwi.kiwin.gay].")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "")]
#![cfg_attr(all(docsrs, not(kiwingay)), doc = "[wiwi.kiwin.gay]: https://wiwi.kiwin.gay")]

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

// ...yes, I'm doing it
// (there's another one in prelude, copy/paste/modify for it)
#![cfg_attr(all(
	not(feature = "auth-unstable"),
	not(feature = "bigint-unstable"),
	not(feature = "bitstream-unstable"),
	not(feature = "chainer-unstable"),
	not(feature = "clock-timer"),
	not(feature = "debounce"),
	not(feature = "defer-unstable"),
	not(feature = "h"),
	not(feature = "hex"),
	not(feature = "id-unstable"),
	not(feature = "iter-unstable"),
	not(feature = "lazy-wrap"),
	not(feature = "lsl-unstable"),
	not(feature = "memory-usage-unstable"),
	not(feature = "minesweeper-unstable"),
	not(feature = "nominal"),
	not(feature = "path-unstable"),
	not(feature = "serialiser-unstable"),
	not(feature = "string-pool-unstable"),
	not(feature = "sudoku-unstable"),
	not(feature = "to-maybeuninit"),
	not(feature = "z85"),
	not(docsrs)
), doc = "No features enabled! (you should probably enable something, otherwise this crate does nothing :p)")]

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::upper_case_acronyms)]

// TODO: remove when more finished
#![allow(
	dead_code,
	unused_imports,
	unused_macros,
	unused_mut,
	unused_variables
)]

mod _internal;
pub mod prelude;

#[cfg(feature = "auth-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "auth-unstable")))]
pub mod auth;

#[cfg(feature = "bigint-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint-unstable")))]
pub mod bigint;

// #[cfg(feature = "base16")]
// pub mod base16;

// #[cfg(feature = "base32")]
// pub mod base32;

// #[cfg(feature = "base64")]
// pub mod base64;

#[cfg(feature = "bitstream-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
pub mod bitstream;

#[cfg(feature = "chainer-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "chainer-unstable")))]
pub mod chainer;

#[cfg(feature = "clock-timer")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock-timer")))]
pub mod clock_timer;

#[cfg(feature = "debounce")]
#[cfg_attr(docsrs, doc(cfg(feature = "debounce")))]
pub mod debounce;

#[cfg(feature = "defer-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "defer-unstable")))]
pub mod defer;

#[cfg(feature = "h")]
#[cfg_attr(docsrs, doc(cfg(feature = "h")))]
pub mod h;

#[cfg(feature = "hex")]
#[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
pub mod hex;

#[cfg(feature = "id-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-unstable")))]
pub mod id;

#[cfg(feature = "iter-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "iter-unstable")))]
pub mod iter;

#[cfg(feature = "lazy-wrap")]
#[cfg_attr(docsrs, doc(cfg(feature = "lazy-wrap")))]
pub mod lazy_wrap;

#[cfg(feature = "lsl-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "lsl-unstable")))]
pub mod lsl;

#[cfg(feature = "memory-usage-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory-usage-unstable")))]
pub mod memory_usage;

#[cfg(feature = "minesweeper-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "minesweeper-unstable")))]
pub mod minesweeper;

#[cfg(feature = "nominal")]
#[cfg_attr(docsrs, doc(cfg(feature = "nominal")))]
pub mod nominal;

#[cfg(feature = "path-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "path-unstable")))]
pub mod path;

#[cfg(feature = "serialiser-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "serialiser-unstable")))]
pub mod serialiser;

#[cfg(feature = "string-pool-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "string-pool-unstable")))]
pub mod string_pool;

#[cfg(feature = "sudoku-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "sudoku-unstable")))]
pub mod sudoku;

#[cfg(feature = "to-maybeuninit")]
#[cfg_attr(docsrs, doc(cfg(feature = "to-maybeuninit")))]
pub mod to_maybeuninit;

// #[cfg(feature = "ws")]
// pub mod ws;

#[cfg(feature = "z85")]
#[cfg_attr(docsrs, doc(cfg(feature = "z85")))]
pub mod z85;
