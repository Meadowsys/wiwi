//! Prelude module, exporting commonly-used types from enabled features

// ----- start autogenerated region (see gen-features script) -----

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
	not(feature = "rand"),
	not(feature = "serialiser-binary-unstable"),
	not(feature = "serialiser-text-unstable"),
	not(feature = "string-pool-unstable"),
	not(feature = "sudoku-unstable"),
	not(feature = "unicode-unstable"),
	not(feature = "with-cloned"),
	not(feature = "z85")
), doc = "(... of which there are none lol)")]

// ----- end autogenerated region -----

// TODO auth prelude
// #[cfg(feature = "auth-unstable")]

// #[cfg(feature = "chainer-unstable")]
// pub use crate::chainer::{
// 	ArrayChain,
// 	SliceBoxedChain,
// 	SliceMutChain,
// 	SliceRefChain,
// 	VecChain
// };

#[cfg(feature = "clock-timer")]
pub use crate::clock_timer::{
	ClockTimer,
	Tick,
	chrono::{
		DateTime,
		Local,
		TimeDelta
	}
};

#[cfg(feature = "debounce")]
pub use crate::debounce::{
	debounce,
	debounce_immediate,
	debounce_with_rt,
	debounce_immediate_with_rt
};

#[cfg(feature = "h")]
pub use crate::h::{ h, H };

#[cfg(feature = "hex")]
pub use crate::hex::{
	encode_hex,
	encode_hex_upper,
	decode_hex
};

#[cfg(feature = "id-unstable")]
pub use crate::id::{
	IDGenerator,
	GeneratedID
};

#[cfg(feature = "lazy-wrap")]
pub use crate::lazy_wrap::{
	LazyWrap,
	LazyWrapState
};

// TODO: lsl prelude
// TODO: serialiser prelude

#[cfg(feature = "rand")]
pub use crate::rand::{
	Randomisable,
	ThreadLocalChaCha8Rng,
	ThreadLocalChaCha12Rng,
	ThreadLocalChaCha20Rng
};

#[cfg(feature = "string-pool-unstable")]
pub use crate::string_pool::String;

#[cfg(feature = "with-cloned")]
pub use crate::with_cloned::with_cloned;

#[cfg(feature = "z85")]
pub use crate::z85::{
	encode_z85,
	decode_z85
};
