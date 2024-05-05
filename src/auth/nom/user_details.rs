use super::*;

/// Something that is able to identify a user. Usually an email or username.
/// Store directly in the database without modification
pub struct UserID {
	pub(in crate::auth) inner: String
}

/// User's password. Never gets sent over the network to the server.
pub struct UserPassword {
	pub(in crate::auth) inner: String
}

impl UserPassword {
	pub fn new(pw: String) -> Self {
		Self { inner: pw }
	}
}
