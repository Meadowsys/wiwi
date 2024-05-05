use super::*;
use chrono::Utc;

pub struct DateTime {
	pub(in crate::auth) inner: chrono::DateTime<Utc>
}

impl DateTime {
	pub(in crate::auth) fn now() -> Self {
		Self { inner: Utc::now() }
	}
}
