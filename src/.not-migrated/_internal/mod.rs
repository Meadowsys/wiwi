//! Internal implementation details and stuff

// TODO: expose this?
#[cfg(any(
	// feature = "base16",
	// feature = "base32",
	// feature = "base64",
	feature = "hex",
	feature = "z85"
))]
pub mod encoding_utils;
