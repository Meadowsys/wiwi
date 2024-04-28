use ::std::{ error, fmt };

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(String);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl error::Error for Error {}

#[inline]
pub fn error<S: Into<String>>(s: S) -> Error {
	Error(s.into())
}

#[inline]
pub fn err<T, S: Into<String>>(s: S) -> Result<T> {
	Err(error(s))
}

#[inline]
pub fn error_eof() -> Error {
	error("unexpected EOF")
}

#[inline]
pub fn err_eof<T>() -> Result<T> {
	Err(error_eof())
}

pub trait OptionExt<T>: Sized {
	fn err<S: Into<String>>(self, s: S) -> Result<T>;

	fn err_eof(self) -> Result<T>;

	fn err_f<S, F>(self, f: F) -> Result<T>
	where
		F: FnOnce() -> S,
		S: Into<String>;
}

impl<T> OptionExt<T> for Option<T> {
	#[inline]
	fn err<S: Into<String>>(self, s: S) -> Result<T> {
		self.ok_or_else(|| error(s))
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
		self.ok_or_else(|| error(f()))
	}
}

pub trait ResultExt<T> {
	fn convert_err(self) -> Result<T>;
}

impl<T, E: ToString> ResultExt<T> for Result<T, E> {
	#[inline]
	fn convert_err(self) -> Result<T> {
		self.map_err(|e| error(e.to_string()))
	}
}
