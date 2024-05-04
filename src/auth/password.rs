use zeroize::Zeroizing;

/// User's password. Never gets sent over the network.
pub struct Password {
	password: Zeroizing<String>
}

impl Password {
	pub fn from_string(password: String) -> Self {
		let password = password.into();
		Self { password }
	}
}
