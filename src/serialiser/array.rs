use super::{ BufferImplWrite, Serialise };
// use super::marker::{ MARKER_HOMOARRAY, MARKER_HOMOARRAY_SMALL, MARKER_HETEROARRAY, MARKER_HETEROARRAY_SMALL };
// // pub trait HomogenousSlice {}

// #[repr(transparent)]
// pub struct HomogenousSliceWrapper<'h, T>(&'h [T]);

// #[repr(transparent)]
// pub struct HeterogenousSliceWrapper<'h, T>(&'h [T]);

// impl<'h, T: Serialise> Serialise for HeterogenousSliceWrapper<'h, T> {
// 	fn serialise<B: BufferImplWrite>(&self, out: &mut B) {}
// }

// impl Serialise for [u8] {
// 	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
// 		const U8_MAX: u64 = LARGEST_U8 as _;
// 		const U16_MAX: u64 = LARGEST_U16 as _;
// 		const U40_MAX: u64 = LARGEST_U40 as _;

// 		match self.len() as u64 {
// 			len @ ..=U8_MAX => {
// 				output.write_byte(MARKER_HOMOARRAY_8);
// 				output.write_byte(MARKER_U8);
// 				output.write_byte(len as u8);
// 			}
// 			len @ ..=U16_MAX => {
// 				output.write_byte(MARKER_HOMOARRAY_16);
// 				output.write_byte(MARKER_U8);
// 				output.write_slice(&(len as u16).to_le_bytes());
// 			}
// 			len @ ..=U40_MAX => {
// 				output.write_byte(MARKER_HOMOARRAY_40);
// 				output.write_byte(MARKER_U8);

// 				let len_bytes = len.to_le_bytes();
// 				output.write_slice(unsafe {
// 					slice::from_raw_parts(&len_bytes as *const u8, 5)
// 				});
// 			}
// 			len => {
// 				output.write_byte(MARKER_HOMOARRAY_XL);
// 				output.write_byte(MARKER_U8);
// 				len.serialise(output);
// 			}
// 		}

// 		output.write_slice(self);
// 	}
// }


// impl<T> Serialise for Vec<T>
// where
// 	[T]: Serialise
// {
// 	#[inline]
// 	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
// 		(**self).serialise(output);
// 	}
// }
