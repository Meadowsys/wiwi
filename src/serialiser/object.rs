use super::{ buffer::*, core::*, error::*, len_int::* };
use ::std::{ collections::HashMap, hash::Hash };

pub const MARKER_OBJECT_8: u8 = 0xae;
pub const MARKER_OBJECT_XL: u8 = 0xaf;

fn serialise_object_len<B: BufferWrite>(len: usize, output: &mut B) {
	match len {
		len @ ..=0xff => {
			output.write_byte(MARKER_OBJECT_8);
			output.write_byte(len as _);
		}
		len => {
			output.write_byte(MARKER_OBJECT_XL);
			serialise_len_int(len, output);
		}
	}
}

fn serialise_object_kv<'h, I, K, V, B>(object: I, output: &mut B, options: &Options)
where
	I: IntoIterator<Item = (&'h K, &'h V)>,
	K: Serialise + 'h,
	V: Serialise + 'h,
	B: BufferWrite
{
	for (k, v) in object {
		k.serialise(output, options);
		v.serialise(output, options);
	}
}

pub fn serialise_std_into_object<K, V, B>(
	object: &HashMap<K, V>,
	output: &mut B,
	options: &Options
)
where
	K: Serialise,
	V: Serialise,
	B: BufferWrite
{
	serialise_object_len(object.len(), output);
	serialise_object_kv(object, output, options);
}

pub fn deserialise_object_len<'h, B: BufferRead<'h>>(marker: u8, input: &mut B) -> Result<usize> {
	match marker {
		MARKER_OBJECT_8 => { input.read_byte().map(|len| len as _) }
		MARKER_OBJECT_XL => { deserialise_len_int(input) }
		_ => { err("expected object") }
	}
}

pub fn deserialise_object_kv<'h, K, V, B>(len: usize, input: &mut B)
	-> impl Iterator<Item = Result<(K, V)>> + '_
where
	K: Deserialise<'h>,
	V: Deserialise<'h>,
	B: BufferRead<'h>
{
	(0..len).map(|_| {
		let k = K::deserialise(input)?;
		let v = V::deserialise(input)?;
		Ok((k, v))
	})
}

pub fn deserialise_object_into_std<'h, K, V, B>(input: &mut B) -> Result<HashMap<K, V>>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	B: BufferRead<'h>
{
	let len = deserialise_object_len(input.read_byte()?, input)?;
	deserialise_object_kv(len, input).collect()
}
