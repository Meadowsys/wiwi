#![doc = include_str!("../README.md")]

#![doc = ""]
#![cfg_attr(
	all(feature = "all-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "## Enabled features (`all-unstable` is enabled)"
)]
#![cfg_attr(
	all(feature = "all", not(feature = "all-unstable"), not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "## Enabled features (`all` is enabled)"
)]
#![cfg_attr(
	all(not(feature = "all"), not(feature = "all-unstable"), not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "## Enabled features"
)]
#![cfg_attr(
	feature = "docs-rs-notouchie-dont-use-this",
	doc = "## Enabled features (docs.rs)"
)]
#![doc = ""]

#![cfg_attr(
	feature = "docs-rs-notouchie-dont-use-this",
	doc = "On docs.rs, the feature `all` is enabled. This means that all stable features, and only stable features, will appear in these docs."
)]
// TODO: "these include: <move below list to here>"
// and make note of "if there are any unstable features listed here, that is bug"
#![cfg_attr(feature = "docs-rs-notouchie-dont-use-this", doc = "")]
#![cfg_attr(
	feature = "docs-rs-notouchie-dont-use-this",
	doc = "If you would like to see documentation for unstable features, please use `cargo doc` to build them locally (or alternatively `cargo doc --open` to open the locally built docs in your default browser)."
)]

#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "auth-unstable"),        doc = "- `auth` (unstable)"       )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "chainer-unstable"),     doc = "- `chainer` (unstable)"    )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "clock-timer"),          doc = "- `clock-timer`"           )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "debounce"),             doc = "- `debounce`"              )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "h"),                    doc = "- `h`"                     )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "hex"),                  doc = "- `hex`"                   )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "id-unstable"),          doc = "- `id` (unstable)"         )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "iter-unstable"),        doc = "- `iter` (unstable)"       )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "lazy-wrap"),            doc = "- `lazy-wrap`"             )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "lsl-unstable"),         doc = "- `lsl` (unstable)"        )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "path-unstable"),        doc = "- `path` (unstable)"       )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "serialiser-unstable"),  doc = "- `serialiser` (unstable)" )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "string-pool-unstable"), doc = "- `string-pool` (unstable)")]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "sudoku-unstable"),      doc = "- `sudoku` (unstable)"     )]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "superstring-unstable"), doc = "- `superstring` (unstable)")]
#![cfg_attr(all(not(feature = "docs-rs-notouchie-dont-use-this"), feature = "z85"),                  doc = "- `z85`"                   )]

// ...yes, I'm doing it
// (there's another one in prelude, copy/paste/modify for it)
#![cfg_attr(all(
	not(feature = "auth-unstable"),
	not(feature = "chainer-unstable"),
	not(feature = "clock-timer"),
	not(feature = "debounce"),
	not(feature = "h"),
	not(feature = "hex"),
	not(feature = "id-unstable"),
	not(feature = "iter-unstable"),
	not(feature = "lazy-wrap"),
	not(feature = "lsl-unstable"),
	not(feature = "path-unstable"),
	not(feature = "serialiser-unstable"),
	not(feature = "string-pool-unstable"),
	not(feature = "sudoku-unstable"),
	not(feature = "superstring-unstable"),
	not(feature = "z85"),
	not(feature = "docs-rs-notouchie-dont-use-this")
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

pub mod prelude;

// TODO: expose this?
#[cfg(any(
	// feature = "base16",
	// feature = "base32",
	// feature = "base64",
	feature = "hex",
	feature = "z85"
))]
mod encoding_utils;

#[cfg(feature = "auth-unstable")]
pub mod auth;

// #[cfg(feature = "base16")]
// pub mod base16;

// #[cfg(feature = "base32")]
// pub mod base32;

// #[cfg(feature = "base64")]
// pub mod base64;

#[cfg(feature = "chainer-unstable")]
pub mod chainer;

#[cfg(feature = "clock-timer")]
pub mod clock_timer;

#[cfg(feature = "debounce")]
pub mod debounce;
feature_cfg_compile_check!("debounce-dyn-fn", cfg of "debounce");

#[cfg(feature = "h")]
pub mod h;

#[cfg(feature = "hex")]
pub mod hex;

#[cfg(feature = "id-unstable")]
pub mod id;

#[cfg(feature = "iter-unstable")]
pub mod iter;

#[cfg(feature = "lazy-wrap")]
pub mod lazy_wrap;

#[cfg(feature = "lsl-unstable")]
pub mod lsl;

#[cfg(feature = "path-unstable")]
pub mod path;

#[cfg(feature = "serialiser-unstable")]
pub mod serialiser;

#[cfg(feature = "string-pool-unstable")]
pub mod string_pool;

#[cfg(feature = "sudoku-unstable")]
pub mod sudoku;

#[cfg(feature = "superstring-unstable")]
pub mod superstring;

// #[cfg(feature = "ws")]
// pub mod ws;

#[cfg(feature = "z85")]
pub mod z85;

/// has to be run in root module and not in the feature modules themselves
/// because then, if this *should have* triggered an error, it won't because
/// the feature is off and module excluded from compilation lol
macro_rules! feature_cfg_compile_check {
	($cfgname:literal, cfg of $featname:literal) => {
		#[cfg(all(
			feature = $cfgname,
			not(feature = $featname)
		))]
		compile_error!(concat!("`", $cfgname, "` is a configuration feature of `", $featname, "`, and does nothing when enabled on its own"));
	}
}
use feature_cfg_compile_check;
