//! Implemetation of [Unicode](https://www.unicode.org)
//!
//! Currently, we are aiming to implement [Unicode 15.1].
//!
//! [Unicode 15.1]: https://www.unicode.org/versions/Unicode15.1.0/

mod _internal;
pub mod char;
pub mod utf8;
pub mod utf16;
pub mod utf32;
