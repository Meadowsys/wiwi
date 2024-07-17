//! Implemetation of [Unicode](https://www.unicode.org)
//!
//! Currently, we are aiming to implement [Unicode 15.1].
//!
//! [Unicode 15.1]: https://www.unicode.org/versions/Unicode15.1.0/

mod _internal;
mod char;
mod utf8;
mod utf16;
mod utf32;

pub use utf8::{ StrUtf8, StringUtf8 };
pub use utf16::{ StrUtf16, StringUtf16 };
pub use utf32::{ StrUtf32, StringUtf32 };
