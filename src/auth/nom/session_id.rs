use super::*;

pub struct SessionID {
	// TODO: u64?
	inner: [u8; 32]
}

pub fn generate() -> SessionID {
	SessionID { inner: util::rand_array() }
}
