use super::*;

/// Generated token used for email verification (of course you can do your own
/// verification if you'd like)
pub struct EmailVerificationToken {
	pub(in crate::auth) inner: [u8; 32]
}

impl EmailVerificationToken {
	pub fn generate() -> Self {
		let inner = util::rand_array();
		Self { inner }
	}
}

/// Last time an email verification token was generated. Use this to implement
/// TTL on in-progress account creations stored in the DB. For example, saying
/// users must verify their email before 6 hours passes, otherwise the unverified
/// data is deleted and users have to start over.
pub struct LastEmailVerificationTokenGenerationTime {
	pub(in crate::auth) inner: datetime::DateTime
}

impl LastEmailVerificationTokenGenerationTime {
	pub(in crate::auth) fn now() -> Self {
		let inner = datetime::DateTime::now();
		Self { inner }
	}
}
