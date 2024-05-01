use super::{ buffer::*, error::*, integer::* };
use ::std::ops::Deref;

pub struct Marker {
	inner: MarkerInner
}

pub enum MarkerInner {
	Number {
		marker: NumberMarker
	},
	Bool {
		marker: BoolMarker
	},
	Null {
		marker: NullMarker
	},
	String {
		marker: StringMarker
	},
	Array {
		marker: ArrayMarker
	},
	Map {
		marker: MapMarker
	}
}

impl Marker {
	pub fn from_byte(byte: u8) -> Result<Self> {
		#[deny(unreachable_patterns)]
		match byte {
			num @ 0x00..=0x7f => { NumberMarkerInner::SmallIntPositive { num }.into() }
			num @ 0xc0..=0xff => { NumberMarkerInner::SmallIntNegative { num: num as _ }.into() }

			marker @ 0x80..=0x9f => {
				let byte_count = ((marker & 0b11111) >> 1) + 1;
				if marker & 1 == 0 {
					NumberMarkerInner::Unsigned { byte_count }.into()
				} else {
					NumberMarkerInner::Signed { byte_count }.into()
				}
			}

			0xa0 => { BoolMarkerInner { value: true }.into() }
			0xa1 => { BoolMarkerInner { value: false }.into() }

			// 0xa2 => { NumberMarkerInner::Float16.into() }
			0xa3 => { NumberMarkerInner::Float32.into() }
			0xa4 => { NumberMarkerInner::Float64.into() }
			// 0xa5 => { NumberMarkerInner::Float128.into() }
			// 0xa6 => { NumberMarkerInner::Float256.into() }

			0xa7 => { NullMarkerInner { __private: () }.into() }

			0xa8 => { StringMarkerInner::Variant8.into() }
			0xa9 => { StringMarkerInner::VariantXL.into() }

			0xaa => { ArrayMarkerInner::Variant8.into() }
			0xab => { ArrayMarkerInner::VariantXL.into() }

			0xac => { MapMarkerInner::Variant8.into() }
			0xad => { MapMarkerInner::VariantXL.into() }

			_ => { err("unknown marker") }
		}
	}

	pub fn to_marker(&self) -> u8 {
		use MarkerInner::*;
		match &self.inner {
			Number { marker } => { marker.to_marker() }
			Bool { marker } => { marker.to_marker() }
			Null { marker } => { marker.to_marker() }
			String { marker } => { marker.to_marker() }
			Array { marker } => { marker.to_marker() }
			Map { marker } => { marker.to_marker() }
		}
	}

	extract_marker_ref! {
		expect_number, NumberMarker, Number
		expect_bool, BoolMarker, Bool
		expect_null, NullMarker, Null
		expect_string, StringMarker, String
		expect_array, ArrayMarker, Array
		expect_map, MapMarker, Map
	}
}

pub struct NumberMarker {
	inner: NumberMarkerInner
}

pub enum NumberMarkerInner {
	SmallIntPositive { num: u8 },
	SmallIntNegative { num: i8 },
	Unsigned { byte_count: u8 },
	Signed { byte_count: u8 },
	// Float16,
	Float32,
	Float64,
	// Float128,
	// Float256
}

impl NumberMarker {
	pub fn to_marker(&self) -> u8 {
		use NumberMarkerInner::*;
		match self.inner {
			SmallIntPositive { num } => { num }
			SmallIntNegative { num } => { num as _ }
			Unsigned { byte_count } => {
				debug_assert!(byte_count <= 16);
				((byte_count - 1) << 1) | 0b100_00000
			}
			Signed { byte_count } => {
				debug_assert!(byte_count <= 16);
				((byte_count - 1) << 1) | 0b100_00001
			}
			// Float16 => {}
			Float32 => { 0xa3 }
			Float64 => { 0xa4 }
			// Float128 => {}
			// Float256 => {}
		}
	}

	// pub fn read_rest_of_u8<B: BufferRead>(&self, input: &mut B) -> Result<u8> {
	// 	use NumberMarkerInner::*;
	// 	Ok(match self.inner {
	// 		SmallIntPositive { num } => { num }
	// 		Unsigned { byte_count } if byte_count == 1 => {
	// 			// input.read_byte()?
	// 			// deserialise_rest_of_u8(checked_marker, input)
	// 		}

	// 		Signed { byte_count } if byte_count == 1 => {
	// 			input.read_byte()?
	// 		}
	// 		// SmallIntNegative { num } => { return err("expected u8-compatible int, found") }
	// 	})
	// }
}

pub struct BoolMarker {
	inner: BoolMarkerInner
}

pub struct BoolMarkerInner {
	value: bool
}

impl BoolMarker {
	#[inline]
	pub fn from_bool(value: bool) -> Self {
		let inner = BoolMarkerInner { value };
		Self { inner }
	}

	#[inline]
	pub fn to_bool(&self) -> bool {
		self.inner.value
	}

	#[inline]
	pub fn to_marker(&self) -> u8 {
		match self.inner.value {
			true => { 0xa0 }
			false => { 0xa1 }
		}
	}
}

pub struct NullMarker {
	inner: NullMarkerInner
}

pub struct NullMarkerInner {
	__private: ()
}

impl NullMarker {
	#[inline]
	pub fn to_marker(&self) -> u8 {
		0xa7
	}
}

pub struct StringMarker {
	inner: StringMarkerInner
}

pub enum StringMarkerInner {
	Variant8,
	VariantXL
}

impl StringMarker {
	pub fn to_marker(&self) -> u8 {
		use StringMarkerInner::*;
		match self.inner {
			Variant8 => { 0xa8 }
			VariantXL => { 0xa9 }
		}
	}
}

pub struct ArrayMarker {
	inner: ArrayMarkerInner
}

pub enum ArrayMarkerInner {
	Variant8,
	VariantXL
}

impl ArrayMarker {
	pub fn to_marker(&self) -> u8 {
		use ArrayMarkerInner::*;
		match self.inner {
			Variant8 => { 0xaa }
			VariantXL => { 0xab }
		}
	}
}

pub struct MapMarker {
	inner: MapMarkerInner
}

pub enum MapMarkerInner {
	Variant8,
	VariantXL
}

impl MapMarker {
	pub fn to_marker(&self) -> u8 {
		use MapMarkerInner::*;
		match self.inner {
			Variant8 => { 0xac }
			VariantXL => { 0xad }
		}
	}
}

macro_rules! marker_impls {
	($($container:ident $container_inner:ident $variant:ident)*) => {
		$(
			impl From<$container_inner> for Marker {
				#[inline]
				fn from(marker: $container_inner) -> Self {
					let marker = $container { inner: marker };
					Self { inner: MarkerInner::$variant { marker } }
				}
			}

			impl<E> From<$container_inner> for Result<Marker, E> {
				#[inline]
				fn from(marker: $container_inner) -> Self {
					Ok(marker.into())
				}
			}

			impl Deref for $container {
				type Target = $container_inner;
				#[inline]
				fn deref(&self) -> &$container_inner {
					&self.inner
				}
			}
		)*
	}
}

marker_impls! {
	NumberMarker NumberMarkerInner Number
	BoolMarker BoolMarkerInner Bool
	NullMarker NullMarkerInner Null
	StringMarker StringMarkerInner String
	ArrayMarker ArrayMarkerInner Array
	MapMarker MapMarkerInner Map
}

impl Deref for Marker {
	type Target = MarkerInner;
	#[inline]
	fn deref(&self) -> &MarkerInner {
		&self.inner
	}
}

macro_rules! extract_marker_ref {
	($($fn_name:ident, $marker:ident, $variant:ident)*) => {
		$(
			pub fn $fn_name(&self) -> Option<&$marker> {
				if let MarkerInner::$variant { marker } = &self.inner {
					Some(marker)
				} else {
					None
				}
			}
		)*
	}
}
use extract_marker_ref;
