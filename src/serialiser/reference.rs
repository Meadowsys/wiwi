use super::{ buffer::*, core::* };

impl<'h, T: Serialise + ?Sized> Serialise for &'h T {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		(**self).serialise(output, options);
	}
}

impl<'h, T: Serialise + ?Sized> Serialise for &'h mut T {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		(**self).serialise(output, options);
	}
}
