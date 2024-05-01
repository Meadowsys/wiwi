use super::*;

exported_match_macro!(smallint_positive_range: 0x00..=0x7f);
exported_match_macro!(smallint_negative_range: 0xc0..=0xff);
exported_match_macro!(int_range: 0x80..=0x9f);

pub const FLOAT16: u8 = 0xa2;
pub const FLOAT32: u8 = 0xa3;
pub const FLOAT64: u8 = 0xa4;
pub const FLOAT128: u8 = 0xa5;
pub const FLOAT256: u8 = 0xa6;

pub struct NumberMarker {
	inner: NumberMarkerInner
}

pub enum NumberMarkerInner {
	SmallIntPositive { marker: u8 },
	SmallIntNegative { marker: u8 },
	Unsigned { byte_count: u8  },
	Signed { byte_count: u8 },
	// Float16,
	Float32,
	Float64,
	// Float128,
	// Float256,
}

impl NumberMarker {
	pub fn to_marker(&self) -> u8 {
		use NumberMarkerInner::*;
		match &**self {
			SmallIntPositive { marker } => {
				debug_assert!(smallint_positive_range!().contains(marker));
				*marker
			}
			SmallIntNegative { marker } => {
				debug_assert!(smallint_negative_range!().contains(marker));
				*marker
			}
			Unsigned { byte_count } => {
				debug_assert!((1..=16).contains(byte_count));
				let marker = ((*byte_count - 1) << 1) | 0b100_00000;
				debug_assert!(int_range!().contains(&marker));
				marker
			}
			Signed { byte_count } => {
				debug_assert!((1..=16).contains(byte_count));
				let marker = ((*byte_count - 1) << 1) | 0b100_00001;
				debug_assert!(int_range!().contains(&marker));
				marker
			}
			// Float16 => { FLOAT16 }
			Float32 => { FLOAT32 }
			Float64 => { FLOAT64 }
			// Float128 => { FLOAT128 }
			// Float256 => { FLOAT256 }
		}
	}
}

marker_impls!(NumberMarker NumberMarkerInner Number);
