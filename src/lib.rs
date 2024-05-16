#![doc = include_str!("../README.md")]

#![cfg_attr(not(feature = "docs-rs-notouchie-dont-use-this"), doc = "")]
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
#![cfg_attr(not(feature = "docs-rs-notouchie-dont-use-this"), doc = "")]

#![cfg_attr(
	all(feature = "auth-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `auth` (unstable)"
)]
#![cfg_attr(
	all(feature = "chaining-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `chaining` (unstable)"
)]
#![cfg_attr(
	all(feature = "clock-timer", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `clock-timer`"
)]
#![cfg_attr(
	all(feature = "debounce", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `debounce`"
)]
#![cfg_attr(
	all(feature = "h", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `h`"
)]
#![cfg_attr(
	all(feature = "hex", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `hex`"
)]
#![cfg_attr(
	all(feature = "id-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `id` (unstable)"
)]
#![cfg_attr(
	all(feature = "lazy-wrap", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `lazy-wrap`"
)]
#![cfg_attr(
	all(feature = "lsl-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `lsl` (unstable)"
)]
#![cfg_attr(
	all(feature = "serialiser-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `serialiser` (unstable)"
)]
#![cfg_attr(
	all(feature = "string-pool-unstable", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `string-pool` (unstable)"
)]
#![cfg_attr(
	all(feature = "z85", not(feature = "docs-rs-notouchie-dont-use-this")),
	doc = "- `z85`"
)]

// ...yes, I'm doing it
#![cfg_attr(all(
	not(feature = "auth-unstable"),
	not(feature = "chaining-unstable"),
	not(feature = "clock-timer"),
	not(feature = "debounce"),
	not(feature = "h"),
	not(feature = "hex"),
	not(feature = "id-unstable"),
	not(feature = "lazy-wrap"),
	not(feature = "lsl-unstable"),
	not(feature = "serialiser-unstable"),
	not(feature = "string-pool-unstable"),
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

#[cfg(feature = "chaining-unstable")]
pub mod chaining;

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

#[cfg(feature = "lazy-wrap")]
pub mod lazy_wrap;

#[cfg(feature = "lsl-unstable")]
pub mod lsl;

#[cfg(feature = "serialiser-unstable")]
pub mod serialiser;

#[cfg(feature = "string-pool-unstable")]
pub mod string_pool;

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
