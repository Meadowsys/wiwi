/// Something that is able to identify a user. Usually an email or username.
/// Store directly in the database without modification
pub struct UserId {
	user_id: String
}

impl UserId {
	pub fn from_string(user_id: String) -> Self {
		Self { user_id }
	}
}
