use super::*;

/// Verification token (ex. email verification token)
pub struct VerificationToken {
	bytes: [u8; 32],
	timestamp: datetime::DateTime
}

pub fn generate() -> VerificationToken {
	let bytes = util::rand_array();
	let timestamp = datetime::now();
	VerificationToken { bytes, timestamp }
}

impl VerificationToken {
	pub fn as_bytes(&self) -> &[u8; 32] {
		&self.bytes
	}
}
