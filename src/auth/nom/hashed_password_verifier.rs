use super::*;

/// Hashed version of the password verifier (server stores this)
pub struct HashedPasswordVerifier {
	pub(in crate::auth) inner: kdf::Kdf
}

impl HashedPasswordVerifier {
	pub(in crate::auth) fn new(
		pw_verifier: &PasswordVerifier,
		salt: &HashedPasswordVerifierSalt
	) -> Result<Self> {
		let inner = kdf::Kdf::hash_and_salt(
			pw_verifier.inner.as_bytes(),
			&salt.inner
		)?;
		Ok(Self { inner })
	}
}

/// Salt used in hashing the password verifier, stored as is
/// alongside the hash itself
pub struct HashedPasswordVerifierSalt {
	pub(in crate::auth) inner: salt::Salt
}

impl HashedPasswordVerifierSalt {
	pub(in crate::auth) fn generate() -> Self {
		let inner = salt::Salt::generate();
		Self { inner }
	}
}
