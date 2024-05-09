use super::*;

/// Something that is able to identify a user. Usually an email or username.
/// Store directly in the database without modification
pub struct UserID {
	inner: String
}

/// User's password. Never gets sent over the network to the server.
pub struct UserPassword {
	inner: String
}

pub fn user_id(id: String) -> UserID {
	UserID { inner: id }
}

pub fn user_password(pw: String) -> UserPassword {
	UserPassword { inner: pw }
}

/// Details input by user in eg. signup or signin form
pub struct UserDetailsInput {
	pub user_id: UserID,
	pub user_password: UserPassword
}

impl UserID {}

impl UserPassword {
	pub(crate) fn as_bytes(&self) -> &[u8] {
		self.inner.as_bytes()
	}
}
