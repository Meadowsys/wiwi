use super::internal_prelude::*;

impl<T: Serialise> Serialise for Option<T> {
	type Serialiser<'h> = OptionSerialiser<'h, T> where Self: 'h;

	#[inline]
	fn build_serialiser(&self) -> OptionSerialiser<'_, T> {
		OptionSerialiser::new(self)
	}
}

pub struct OptionSerialiser<'h, T: Serialise + 'h> {
	inner: Option<T::Serialiser<'h>>
}

impl<'h, T: Serialise + 'h> OptionSerialiser<'h, T> {
	#[inline]
	fn new(optional_val: &'h Option<T>) -> Self {
		let inner = optional_val.as_ref()
			.map(|val| val.build_serialiser());
		OptionSerialiser { inner }
	}
}

impl<'h, T: Serialise + 'h> Serialiser<'h> for OptionSerialiser<'h, T> {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.inner
			.as_ref()
			// whatever the item serialises into
			.map(|ser| ser.needed_capacity())
			// or null marker (1)
			.unwrap_or(1)
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(ser) = &self.inner {
			ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_NULL);
		}
	}
}

impl<'h, T: Deserialise<'h>> Deserialise<'h> for Option<T> {
	type Error = T::Error;

	#[inline]
	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Option<T>, T::Error> {
		Ok(if marker == MARKER_NULL {
			None
		} else {
			Some(use_ok!(T::deserialise_with_marker(buf, marker)))
		})
	}
}
