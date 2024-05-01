use super::*;

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
				debug_assert!((0x00..=0x7f).contains(marker));
				*marker
			}
			SmallIntNegative { marker } => {
				debug_assert!((0xc0..=0xff).contains(marker));
				*marker
			}
			Unsigned { byte_count } => {
				debug_assert!((1..=16).contains(byte_count));
				((*byte_count - 1) << 1) | 0b100_00000
			}
			Signed { byte_count } => {
				debug_assert!((1..=16).contains(byte_count));
				((*byte_count - 1) << 1) | 0b100_00001
			}
			// Float16 => { 0xa2 }
			Float32 => { 0xa3 }
			Float64 => { 0xa4 }
			// Float128 => { 0xa5 }
			// Float256 => { 0xa6 }
		}
	}
}

marker_impls!(NumberMarker NumberMarkerInner Number);
