use super::{ buffer::*, error::* };

pub struct Marker {
	inner: MarkerInner
}

enum MarkerInner {
	SmallIntPositive { marker: u8 },
	SmallIntNegative { marker: u8 },
	Int { marker: u8 },
	Bool { marker: u8 },
	Float { marker: u8 },
	Null,
	String { marker: u8 },
	Array { marker: u8 },
	Map { marker: u8 },
	SingleTypeArray { marker: u8 },
	Binary { marker: u8 },
	// Set { marker: u8 }, // TODO: ?????
	Unassigned { marker: u8 },
	Reserved2Byte1,
	Reserved2Byte2,
	Reserved3Byte
}

impl Marker {
	pub fn read_from<'h, B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		#[deny(overlapping_range_endpoints)]
		let inner = match input.read_byte()? {
			marker @ 0x00..=0x7f => { MarkerInner::SmallIntPositive { marker } }
			marker @ 0xc0..=0xff => { MarkerInner::SmallIntNegative { marker } }
			marker @ 0x80..=0x9f => { MarkerInner::Int { marker } }
			marker @ 0xa0..=0xa1 => { MarkerInner::Bool { marker } }
			marker @ 0xa2..=0xa6 => { MarkerInner::Float { marker } }
			0xa7 => { MarkerInner::Null }
			marker @ 0xa8..=0xa9 => { MarkerInner::String { marker } }
			marker @ 0xaa..=0xab => { MarkerInner::Array { marker } }
			marker @ 0xac..=0xad => { MarkerInner::Map { marker } }
			marker @ 0xae..=0xaf => { MarkerInner::SingleTypeArray { marker } }
			marker @ 0xb0..=0xb1 => { MarkerInner::Binary { marker } }
			// marker @ 0xb2..=0xb3 => { MarkerInner::Set { marker } }
			marker @ 0xb2..=0xbc => { MarkerInner::Unassigned { marker } }
			0xbd => { MarkerInner::Reserved2Byte1 }
			0xbe => { MarkerInner::Reserved2Byte2 }
			0xbf => { MarkerInner::Reserved3Byte }
		};

		Ok(Marker { inner })
	}
}
