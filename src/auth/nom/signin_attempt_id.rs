use super::*;

#[derive(Clone)]
pub struct SigninAttemptID {
	// TODO: u64?
	inner: [u8; 32]
}

pub fn generate() -> SigninAttemptID {
	SigninAttemptID { inner: util::rand_array() }
}
