use super::*;

pub struct SigningChallenge {
	inner: [u8; 512]
}

pub fn generate() -> SigningChallenge {
	SigningChallenge { inner: util::rand_array() }
}

impl SigningChallenge {
	pub(crate) fn as_bytes(&self) -> &[u8; 512] {
		&self.inner
	}
}
