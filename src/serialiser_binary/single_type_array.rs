use super::internal_prelude::*;
use super::USizeSerialiser;
use super::number::{
	get_byte_count_signed_le,
	get_byte_count_unsigned_le,
	get_marker_for_signed,
	get_marker_for_unsigned
};
use std::{ hint, slice };
