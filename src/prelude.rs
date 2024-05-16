//! Prelude module, exporting types from enabled features

// TODO auth prelude
// #[cfg(feature = "auth-unstable")]

#[cfg(feature = "chaining-unstable")]
pub use crate::chaining::{
	ArrayChain,
	SliceBoxedChain,
	SliceMutChain,
	SliceRefChain,
	VecChain
};

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
pub use crate::h::h;

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

#[cfg(feature = "string-pool-unstable")]
pub use crate::string_pool::String;

#[cfg(feature = "z85")]
pub use crate::z85::{
	encode_z85,
	decode_z85
};
