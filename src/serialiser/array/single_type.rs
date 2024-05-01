use super::*;
use std::marker::PhantomData;

pub const MARKER_SINGLE_TYPE_ARRAY_8: u8 = 0xae;
pub const MARKER_SINGLE_TYPE_ARRAY_XL: u8 = 0xaf;

pub trait SingleTypeSerialise {
	// TODO: maybe make seperate trait for variable/no? so users can choose to switch
	// between saving storage and traversing twice, or saving compute and traversing once
	// (for compatible types of course)
	const VARIABLE_MARKER: bool = false;
	fn get_preferred_marker(&self, prev_preferred: Option<u8>) -> u8;
	fn serialise_rest_for_marker<B: BufferWrite>(&self, marker: u8, output: &mut B, options: &Options);
}

pub struct SingleTypeArray<T, A> {
	array: A,
	item: PhantomData<T>
}

impl<T, A> SingleTypeArray<T, A>
where
	A: AsRef<[T]>,
	T: SingleTypeSerialise
{
	pub fn new(array: A) -> Self {
		Self { array, item: PhantomData }
	}
}

impl<T, A> Serialise for SingleTypeArray<T, A>
where
	A: AsRef<[T]>,
	T: SingleTypeSerialise
{
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		let array = self.array.as_ref();
		match array.len() {
			len @ ..=0xff => {
				output.write_byte(MARKER_SINGLE_TYPE_ARRAY_8);
				output.write_byte(len as _);
			}
			len => {
				output.write_byte(MARKER_SINGLE_TYPE_ARRAY_XL);
				serialise_len_int(len, output);
			}
		}

		if T::VARIABLE_MARKER {
			let mut iter = array.iter();

			let mut marker = if let Some(item) = iter.next() {
				item.get_preferred_marker(None)
			} else {
				return
			};

			for item in iter {
				marker = item.get_preferred_marker(Some(marker));
			}

			output.write_byte(marker);

			for item in array {
				item.serialise_rest_for_marker(marker, output, options);
			}
		} else {
			let mut iter = array.iter();

			let marker = if let Some(item) = iter.next() {
				let marker = item.get_preferred_marker(None);
				item.serialise_rest_for_marker(marker, output, options);
				marker
			} else {
				return
			};

			output.write_byte(marker);

			for item in iter {
				item.serialise_rest_for_marker(marker, output, options);
			}
		}
	}
}
