pub trait PtrExt {
	type Out;
	fn ref_to_ptr(&self) -> *const Self::Out;
	fn ref_to_ptr_mut(&mut self) -> *mut Self::Out;
}

impl<T> PtrExt for T {
	type Out = T;

	#[inline(always)]
	fn ref_to_ptr(&self) -> *const T {
		self
	}

	#[inline(always)]
	fn ref_to_ptr_mut(&mut self) -> *mut T {
		self
	}
}

impl<T> PtrExt for [T] {
	type Out = T;

	#[inline(always)]
	fn ref_to_ptr(&self) -> *const T {
		self.as_ptr()
	}

	#[inline(always)]
	fn ref_to_ptr_mut(&mut self) -> *mut T {
		self.as_mut_ptr()
	}
}

pub trait PtrSliceExt {
	fn slice_to_ptr(&self) -> *const Self;
	fn slice_to_ptr_mut(&mut self) -> *mut Self;
}

impl<T> PtrSliceExt for [T] {
	#[inline(always)]
	fn slice_to_ptr(&self) -> *const [T] {
		self
	}

	#[inline(always)]
	fn slice_to_ptr_mut(&mut self) -> *mut [T] {
		self
	}
}
