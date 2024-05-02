use std::{ error, fmt, ops::Deref };

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
	Str { msg: &'static str },
	String { msg: String }
}

impl Deref for Error {
	type Target = str;
	fn deref(&self) -> &str {
		match self {
			Self::Str { msg } => { msg }
			Self::String { msg } => { msg }
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self)
	}
}

impl error::Error for Error {}

#[inline]
pub const fn error_str(msg: &'static str) -> Error {
	Error::Str { msg }
}

#[inline]
pub fn error_string<S: Into<String>>(msg: S) -> Error {
	Error::String { msg: msg.into() }
}

#[inline]
pub const fn err_str<T>(msg: &'static str) -> Result<T> {
	Err(error_str(msg))
}

#[inline]
pub fn err_string<T, S: Into<String>>(msg: S) -> Result<T> {
	Err(error_string(msg))
}

#[inline]
pub const fn error_eof() -> Error {
	error_str("unexpected EOF")
}

#[inline]
pub const fn err_eof<T>() -> Result<T> {
	Err(error_eof())
}

pub trait OptionExt<T>: Sized {
	fn err_str(self, msg: &'static str) -> Result<T>;

	fn err_string<S: Into<String>>(self, msg: S) -> Result<T>;

	fn err_eof(self) -> Result<T>;

	fn err_f<S, F>(self, f: F) -> Result<T>
	where
		F: FnOnce() -> S,
		S: Into<String>;
}

impl<T> OptionExt<T> for Option<T> {
	fn err_str(self, msg: &'static str) -> Result<T> {
		self.ok_or_else(|| error_str(msg))
	}

	fn err_string<S: Into<String>>(self, msg: S) -> Result<T> {
		self.ok_or_else(|| error_string(msg))
	}

	#[inline]
	fn err_eof(self) -> Result<T> {
		self.ok_or_else(error_eof)
	}

	#[inline]
	fn err_f<S, F>(self, f: F) -> Result<T>
	where
		F: FnOnce() -> S,
		S: Into<String>
	{
		self.ok_or_else(|| error_string(f()))
	}
}

pub trait ResultExt<T> {
	fn convert_err(self) -> Result<T>;
}

impl<T, E: ToString> ResultExt<T> for Result<T, E> {
	#[inline]
	fn convert_err(self) -> Result<T> {
		self.map_err(|e| error_string(e.to_string()))
	}
}
