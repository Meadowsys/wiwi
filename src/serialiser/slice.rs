// use super::{ BufferImplWrite, Serialise };
// use super::marker::{ MARKER_HOMOARRAY, MARKER_HOMOARRAY_SMALL, MARKER_HETEROARRAY, MARKER_HETEROARRAY_SMALL };
// // pub trait HomogenousSlice {}

// #[repr(transparent)]
// pub struct HomogenousSliceWrapper<'h, T>(&'h [T]);

// #[repr(transparent)]
// pub struct HeterogenousSliceWrapper<'h, T>(&'h [T]);

// impl<'h, T: Serialise> Serialise for HeterogenousSliceWrapper<'h, T> {
// 	fn serialise<B: BufferImplWrite>(&self, out: &mut B) {}
// }
