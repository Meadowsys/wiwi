// use self::error::*;
// use ::std::slice;

// pub mod error;

// pub mod array;
// pub mod bool;
// pub mod float;
// pub mod integer;
// pub mod marker;
// pub mod none;
// pub mod object;
// pub mod specialisations;
// pub mod string;
// pub mod value;

// pub use self::value::{ Key, Value, ValueOwned };

// pub trait Serialise {
// 	fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options);
// }

// pub trait Deserialise<'h>: Sized {
// 	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self>;
// }

// pub trait DeserialiseOwned: for<'h> Deserialise<'h> {}

// impl<T> DeserialiseOwned for T
// where
// 	T: for<'h> Deserialise<'h>
// {}

// pub fn serialise<T: ?Sized + Serialise>(item: &T) -> Vec<u8> {
// 	serialise_with_options(item, &Options::default())
// }

// pub fn serialise_with_options<T: ?Sized + Serialise>(item: &T, options: &Options) -> Vec<u8> {
// 	let mut vec = Vec::with_capacity(options.capacity);
// 	item.serialise(&mut vec, options);
// 	vec
// }


// pub fn deserialise<'h, T: Deserialise<'h>>(mut bytes: &'h [u8]) -> Result<T> {
// 	let value = T::deserialise(&mut bytes);
// 	if !bytes.is_empty() { return err("trailing bytes found") }
// 	value
// }

// /// Trait for generic buffer impl. Unsafe trait to assert that implementers have implemented it correctly
// // Might make it easier to do that unsafe impl later? :p
// pub unsafe trait BufferImplWrite {
// 	// TODO: preallocation related methods
// 	// unsafe fn preallocate_at_once(amount: usize);
// 	// some kind of builder device that ultimately calls preallocate_at_once
// 	// unsafe fn preallocate_amount_tracker

// 	fn write_slice(&mut self, bytes: &[u8]);
// 	fn write_byte(&mut self, byte: u8);

// 	// TODO: unsafe write methods
// 	// unsafe fn write_ptr(&mut self, bytes: *const u8, len: usize) {
// 	// 	self.write_slice(slice::from_raw_parts(bytes, len));
// 	// }
// 	// unsafe fn write_ptr_const<const N: usize>(&mut self, bytes: *const u8) {
// 	// 	self.write_slice(slice::from_raw_parts(bytes, N));
// 	// }
// 	// unsafe fn write_bytes_const<N>
// }

// unsafe impl BufferImplWrite for Vec<u8> {
// 	fn write_slice(&mut self, bytes: &[u8]) {
// 		self.extend(bytes);
// 	}

// 	fn write_byte(&mut self, byte: u8) {
// 		self.push(byte);
// 	}
// }

// pub unsafe trait BufferImplRead<'h> {
// 	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8>;
// 	// fn peek_next_byte(&mut self) -> Option<u8>;

// 	#[inline]
// 	unsafe fn read_bytes_const_ptr<const N: usize>(&mut self) -> Result<*const u8> {
// 		self.read_bytes_ptr(N)
// 	}

// 	#[inline]
// 	fn read_bytes(&mut self, count: usize) -> Result<&'h [u8]> {
// 		unsafe {
// 			self.read_bytes_ptr(count)
// 				.map(|ptr| ::std::slice::from_raw_parts(ptr, count))
// 		}
// 	}

// 	#[inline]
// 	fn read_bytes_const<const N: usize>(&mut self) -> Result<&'h [u8; N]> {
// 		unsafe {
// 			self.read_bytes_const_ptr::<N>()
// 				.map(|ptr| &*(ptr as *const [u8; N]))
// 		}
// 	}

// 	#[inline]
// 	fn read_byte(&mut self) -> Result<u8> {
// 		unsafe {
// 			self.read_bytes_const_ptr::<1>()
// 				.map(|ptr| *ptr)
// 		}
// 	}
// }

// unsafe impl<'h> BufferImplRead<'h> for &'h [u8] {
// 	unsafe fn read_bytes_ptr(&mut self, count: usize) -> Result<*const u8> {
// 		(self.len() >= count).then(#[inline] || {
// 			let self_ptr = *self as *const [u8] as *const u8;
// 			*self = slice::from_raw_parts(
// 				self_ptr.add(count),
// 				self.len() - count
// 			);
// 			self_ptr
// 		}).err_eof()
// 	}

// 	// fn peek_next_byte(&mut self) -> Option<u8> {
// 	// 	(!self.is_empty()).then(#[inline] || unsafe {
// 	// 		*(*self as *const [u8] as *const u8)
// 	// 	})
// 	// }
// }

// #[derive(Clone, Debug)]
// pub struct Options {
// 	pub capacity: usize
// }

// impl Default for Options {
// 	fn default() -> Self {
// 		Options {
// 			capacity: 128
// 		}
// 	}
// }

// // helper things

// struct SerialiseLength3VariantsParams<'h, B> {
// 	pub marker_8: u8,
// 	pub marker_16: u8,
// 	pub marker_xl: u8,
// 	pub len: usize,
// 	pub output: &'h mut B,
// 	pub options: &'h Options
// }

// #[inline]
// fn serialise_len_3_variants<B: BufferImplWrite>(
// 	params: SerialiseLength3VariantsParams<B>
// ) {
// 	use self::{ integer::*, marker::* };

// 	let SerialiseLength3VariantsParams {
// 		marker_8,
// 		marker_16,
// 		marker_xl,
// 		len,
// 		output,
// 		options
// 	} = params;
// 	let len = len as u64;

// 	match get_marker_type(len) {
// 		MarkerType::M8 => {
// 			output.write_byte(marker_8);
// 			unsafe { serialise_rest_of_u64(len, MARKER_U8, output) }
// 		}
// 		MarkerType::M16 => {
// 			output.write_byte(marker_16);
// 			unsafe { serialise_rest_of_u64(len, MARKER_U16, output) }
// 		}
// 		MarkerType::MXL => {
// 			output.write_byte(marker_xl);
// 			len.serialise(output, options);
// 		}
// 	}
// }

// fn serialise_len_3_variants_with_type<B: BufferImplWrite>(
// 	marker_type: MarkerType,
// 	len: u64,
// 	output: &mut B,
// 	options: &Options
// ) {
// 	use self::{ integer::*, marker::* };
// 	match marker_type {
// 		MarkerType::M8 => unsafe { serialise_rest_of_u64(len, MARKER_U8, output) }
// 		MarkerType::M16 => unsafe { serialise_rest_of_u64(len, MARKER_U16, output) }
// 		MarkerType::MXL => { len.serialise(output, options) }
// 	}
// }

// // struct DeserialiseLength3VariantsParams<'h, B> {
// // 	pub marker: u8,
// // 	pub marker_8: u8,
// // 	pub marker_16: u8,
// // 	pub marker_xl: u8,
// // 	pub input: &'h mut B,
// // 	pub err_msg: &'static str
// // }
// //
// // #[inline]
// // fn deserialise_length_3_variants<B: BufferImplRead>(
// // 	params: DeserialiseLength3VariantsParams<B>
// // ) -> Result<usize> {
// // 	use self::{ integer::*, marker::* };
// //
// // 	let DeserialiseLength3VariantsParams {
// // 		marker,
// // 		marker_8,
// // 		marker_16,
// // 		marker_xl,
// // 		input,
// // 		err_msg
// // 	} = params;
// //
// // 	Ok(match input.read_byte()? {
// // 		marker if marker == marker_8 => unsafe { deserialise_rest_of_u64(MARKER_U8, input)? as usize }
// // 		marker if marker == marker_16 => unsafe { deserialise_rest_of_u64(MARKER_U16, input)? as usize }
// // 		marker if marker == marker_xl => { u64::deserialise(input)? as usize }
// // 		_ => { return err(err_msg) }
// // 	})
// // }

// #[derive(Clone, Copy)]

// // #[repr(u8)]
// pub enum MarkerType {
// 	M8 = 1,
// 	M16 = 2,
// 	MXL = 3
// }

// impl MarkerType {
// 	fn from_u8(n: u8) -> Option<Self> {
// 		const M8_U8: u8 = MarkerType::M8 as u8;
// 		const M16_U8: u8 = MarkerType::M16 as u8;
// 		const MXL_U8: u8 = MarkerType::MXL as u8;

// 		match n {
// 			M8_U8 => { Some(Self::M8) }
// 			M16_U8 => { Some(Self::M16) }
// 			MXL_U8 => { Some(Self::MXL) }
// 			_ => { None }
// 		}
// 	}
// }

// fn make_variants_byte(
// 	marker1: Option<MarkerType>,
// 	marker2: Option<MarkerType>,
// 	marker3: Option<MarkerType>,
// 	marker4: Option<MarkerType>,
// ) -> u8 {
// 	let mut byte = 0u8;

// 	if let Some(marker) = marker1 {
// 		byte |= (marker as u8) << 6;
// 	}

// 	if let Some(marker) = marker2 {
// 		byte |= (marker as u8) << 4;
// 	}

// 	if let Some(marker) = marker3 {
// 		byte |= (marker as u8) << 2;
// 	}

// 	if let Some(marker) = marker4 {
// 		byte |= marker as u8;
// 	}

// 	byte
// }

// fn decode_variants_byte(byte: u8) -> (
// 	Option<MarkerType>,
// 	Option<MarkerType>,
// 	Option<MarkerType>,
// 	Option<MarkerType>
// ) {
// 	(
// 		MarkerType::from_u8(byte >> 6),
// 		MarkerType::from_u8((byte >> 4) & 0b11),
// 		MarkerType::from_u8((byte >> 2) & 0b11),
// 		MarkerType::from_u8(byte & 0b11),
// 	)
// }

// #[inline]
// fn deserialise_rest_of_length_3_variants<'h, B: BufferImplRead<'h>>(
// 	marker_type: MarkerType,
// 	input: &mut B
// ) -> Result<usize> {
// 	use self::{ integer::*, marker::* };

// 	Ok(match marker_type {
// 		MarkerType::M8 => unsafe { deserialise_rest_of_u64(MARKER_U8, input)? as usize }
// 		MarkerType::M16 => unsafe { deserialise_rest_of_u64(MARKER_U16, input)? as usize }
// 		MarkerType::MXL => { u64::deserialise(input)? as usize }
// 	})
// }

// fn get_marker_type(len: u64) -> MarkerType {
// 	const U8_MAX: u64 = u8::MAX as u64;
// 	const U16_MAX: u64 = u16::MAX as u64;

// 	match len {
// 		len @ ..=U8_MAX => {
// 			MarkerType::M8
// 		}
// 		len @ ..=U16_MAX => {
// 			MarkerType::M16
// 		}
// 		len => {
// 			MarkerType::MXL
// 		}
// 	}
// }
