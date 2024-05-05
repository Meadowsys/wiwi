use super::*;

/// Generated token used for verification (of course you can do your own
/// verification if you'd like)
pub struct VerificationToken {
	pub(in crate::auth) inner: [u8; 32]
}

impl VerificationToken {
	pub fn generate() -> Self {
		let inner = util::rand_array();
		Self { inner }
	}
}

/// Last time a verification token was generated. Use this to implement
/// TTL on in-progress account creations stored in the DB. For example, saying
/// users must verify before 6 hours passes, otherwise the unverified data is
/// deleted and users have to start over.
pub struct LastVerificationTokenGenerationTime {
	pub(in crate::auth) inner: datetime::DateTime
}

impl LastVerificationTokenGenerationTime {
	pub(in crate::auth) fn now() -> Self {
		let inner = datetime::DateTime::now();
		Self { inner }
	}
}
