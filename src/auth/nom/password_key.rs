use super::*;

/// Made by hashing a user's password with the password salt. Used for
/// encrypting/decrypting the secret key. Not sent to the server; only sent after
/// hashing again (as password verifier).
pub struct PasswordKey {
	pub(in crate::auth) inner: kdf::Kdf
}

impl PasswordKey {
	pub(in crate::auth) fn new(
		pw: &user_details::UserPassword,
		salt: &UserPasswordSalt
	) -> Result<Self> {
		let inner = kdf::Kdf::hash_and_salt(pw.inner.as_bytes(), &salt.inner)?;
		Ok(Self { inner })
	}
}

/// Salt used in hashing a user's password. Sent to the server as is for storage.
pub struct UserPasswordSalt {
	pub(in crate::auth) inner: salt::Salt
}

impl UserPasswordSalt {
	pub(in crate::auth) fn generate() -> Self {
		let inner = salt::Salt::generate();
		Self { inner }
	}
}
