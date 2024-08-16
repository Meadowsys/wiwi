use super::{ Output, Serialise, Serialiser };

impl Serialise for bool {
	type Serialiser<'h> = BoolSerialiser;

	#[inline]
	fn build_serialiser(&self) -> BoolSerialiser {
		BoolSerialiser(*self)
	}
}

pub struct BoolSerialiser(bool);

impl<'h> Serialiser<'h> for BoolSerialiser {
	#[inline]
	fn serialise<O>(&self, buf: &mut O)
	where
		O: Output<'h>
	{
		buf.write_byte(self.0 as u8);
	}
}
