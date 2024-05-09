use super::*;

/// Hashed version of the password verifier (server stores this)
pub struct HashedPasswordVerifier {
	pub(crate) inner: kdf::Kdf
}

pub struct HashedPasswordVerifierSalt {
	inner: salt::Salt
}

pub(crate) fn new(
	pw_verifier: &password_verifier::PasswordVerifier
) -> Result<(HashedPasswordVerifier, HashedPasswordVerifierSalt)> {
	let salt = generate_salt();
	with_salt(pw_verifier, &salt).map(|hpv| (hpv, salt))
}

pub(crate) fn with_salt(
	pw_verifier: &password_verifier::PasswordVerifier,
	salt: &HashedPasswordVerifierSalt
) -> Result<HashedPasswordVerifier> {
	let inner = kdf::hash(pw_verifier.as_bytes(), &salt.inner)?;
	Ok(HashedPasswordVerifier { inner })
}

pub(crate) fn generate_salt() -> HashedPasswordVerifierSalt {
	let inner = salt::generate();
	HashedPasswordVerifierSalt { inner }
}

// impl HashedPasswordVerifier {
// 	pub(crate) fn new(
// 		pw_verifier: &PasswordVerifier,
// 		salt: &HashedPasswordVerifierSalt
// 	) -> Result<Self> {
// 		let inner = kdf::Kdf::hash_and_salt(
// 			pw_verifier.inner.as_bytes(),
// 			&salt.inner
// 		)?;
// 		Ok(Self { inner })
// 	}
// }

// /// Salt used in hashing the password verifier, stored as is
// /// alongside the hash itself
// pub struct HashedPasswordVerifierSalt {
// 	pub(crate) inner: salt::Salt
// }

// impl HashedPasswordVerifierSalt {
// 	pub(crate) fn generate() -> Self {
// 		let inner = salt::Salt::generate();
// 		Self { inner }
// 	}
// }
