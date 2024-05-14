#![doc = include_str!("../README.md")]

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

use cfg_if::cfg_if;

pub mod prelude;

// TODO: expose this?
#[cfg(any(
	feature = "base16",
	feature = "base32",
	feature = "base64",
	feature = "hex",
	feature = "z85"
))]
mod encoding_utils;

#[cfg(feature = "auth")]
pub mod auth;
unstable_feature!("auth");

// #[cfg(feature = "base16")]
// pub mod base16;

// #[cfg(feature = "base32")]
// pub mod base32;

// #[cfg(feature = "base64")]
// pub mod base64;

#[cfg(feature = "chaining")]
pub mod chaining;
unstable_feature!("chaining");

#[cfg(feature = "clock-timer")]
pub mod clock_timer;

#[cfg(feature = "debounce")]
pub mod debounce;
feature_cfg_compile_check!("debounce-dyn-fn", cfg of "debounce");

#[cfg(feature = "h")]
pub mod h;

#[cfg(feature = "hex")]
pub mod hex;

#[cfg(feature = "id")]
pub mod id;
unstable_feature!("id");

#[cfg(feature = "lazy-wrap")]
pub mod lazy_wrap;

#[cfg(feature = "lsl")]
pub mod lsl;
unstable_feature!("lsl");

#[cfg(feature = "serialiser")]
pub mod serialiser;
unstable_feature!("serialiser");

#[cfg(feature = "string-pool")]
pub mod string_pool;

// #[cfg(feature = "ws")]
// pub mod ws;

#[cfg(feature = "z85")]
pub mod z85;

// misc other checks

// macros and stuff

/// has to be run in this module and not in the feature modules themselves
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

macro_rules! unstable_feature {
	($feature:literal) => {
		#[cfg(all(
			not(feature = "unstable"),
			feature = $feature
		))]
		compile_error!(concat!("`", $feature, "` is an unstable feature, and you must have the `unstable` feature enabled to use it"));
	}
}
use unstable_feature;
