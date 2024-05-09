use super::*;
use chrono::Utc;

pub struct DateTime {
	inner: chrono::DateTime<Utc>
}

pub fn now() -> DateTime {
	DateTime { inner: Utc::now() }
}
