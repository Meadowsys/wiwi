use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::{ error::*, marker::*, integer::* };

impl Serialise for str {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
		const U8_MAX: u64 = u8::MAX as u64;
		const U16_MAX: u64 = u16::MAX as u64;
		const U24_MAX: u64 = (u16::MAX as u64) << 8 | u8::MAX as u64;

		match self.len() as u64 {
			len @ ..=U8_MAX => {
				output.write_byte(MARKER_STRING_8);
				output.write_byte(len as u8);
			}
			len @ ..=U16_MAX => {
				output.write_byte(MARKER_STRING_16);
				unsafe { serialise_rest_of_u64(len, MARKER_U16, output) }
			}
			len @ ..=U24_MAX => {
				output.write_byte(MARKER_STRING_24);
				unsafe { serialise_rest_of_u64(len, MARKER_U24, output) }
			}
			len => {
				output.write_byte(MARKER_STRING_XL);
				len.serialise(output);
			}
		}

		output.write_slice(self.as_bytes());
	}
}

impl<'h> Deserialise<'h> for String {
	fn deserialise<B: BufferImplRead>(input: &mut B) -> Result<Self> {
		let length = match input.read_next_byte()? {
			MARKER_STRING_8 => unsafe { deserialise_rest_of_u64(MARKER_U8, input)? }
			MARKER_STRING_16 => unsafe { deserialise_rest_of_u64(MARKER_U16, input)? }
			MARKER_STRING_24 => unsafe { deserialise_rest_of_u64(MARKER_U24, input)? }
			MARKER_STRING_XL => { u64::deserialise(input)? }
			_ => { return err("expected string") }
		};

		let str_bytes = input.read_next_bytes(length as usize)?;
		String::from_utf8(str_bytes.into()).convert_err()
	}
}
