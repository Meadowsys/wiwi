use super::{ buffer::*, core::*, error::*, len_int::* };
use ::std::hash::{ BuildHasher, Hash };
use ::std::collections::{ BTreeMap, HashMap };

pub const MARKER_MAP_8: u8 = 0xac;
pub const MARKER_MAP_XL: u8 = 0xad;

pub fn serialise_map_len<B: BufferWrite>(len: usize, output: &mut B) {
	match len {
		len @ ..=0xff => {
			output.write_byte(MARKER_MAP_8);
			output.write_byte(len as _);
		}
		len => {
			output.write_byte(MARKER_MAP_XL);
			serialise_len_int(len, output);
		}
	}
}

pub fn serialise_map_kv<'h, I, K, V, B>(map: I, output: &mut B, options: &Options)
where
	I: IntoIterator<Item = (&'h K, &'h V)>,
	K: Serialise + 'h,
	V: Serialise + 'h,
	B: BufferWrite
{
	for (k, v) in map {
		k.serialise(output, options);
		v.serialise(output, options);
	}
}

pub fn deserialise_map_len<'h, B: BufferRead<'h>>(marker: u8, input: &mut B) -> Result<usize> {
	match marker {
		MARKER_MAP_8 => { input.read_byte().map(|len| len as _) }
		MARKER_MAP_XL => { deserialise_len_int(input) }
		_ => { err("expected map") }
	}
}

pub fn deserialise_map_kv<'h, K, V, B>(len: usize, input: &mut B)
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

impl<K, V, S> Serialise for HashMap<K, V, S>
where
	K: Serialise,
	V: Serialise,
	S: BuildHasher
{
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		serialise_map_len(self.len(), output);
		serialise_map_kv(self, output, options);
	}
}

impl<'h, K, V, S> Deserialise<'h> for HashMap<K, V, S>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	S: BuildHasher + Default
{
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		let len = deserialise_map_len(input.read_byte()?, input)?;
		deserialise_map_kv(len, input).collect()
	}
}

impl<K, V> Serialise for BTreeMap<K, V>
where
	K: Serialise,
	V: Serialise
{
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		serialise_map_len(self.len(), output);
		serialise_map_kv(self, output, options);
	}
}

impl<'h, K, V> Deserialise<'h> for BTreeMap<K, V>
where
	K: Deserialise<'h> + Ord,
	V: Deserialise<'h>
{
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		let len = deserialise_map_len(input.read_byte()?, input)?;
		deserialise_map_kv(len, input).collect()
	}
}

#[cfg(feature = "hashbrown")]
impl<K, V, S> Serialise for ::hashbrown::HashMap<K, V, S>
where
	K: Serialise,
	V: Serialise,
	S: BuildHasher
{
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		serialise_map_len(self.len(), output);
		serialise_map_kv(self, output, options);
	}
}

#[cfg(feature = "hashbrown")]
impl<'h, K, V, S> Deserialise<'h> for ::hashbrown::HashMap<K, V, S>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	S: BuildHasher + Default
{
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		let len = deserialise_map_len(input.read_byte()?, input)?;
		deserialise_map_kv(len, input).collect()
	}
}

/*
#[cfg(feature = "serde-json")]
impl Serialise for ::serde_json::Map<String, ::serde_json::Value> {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		serialise_map_len(self.len(), output);
		serialise_map_kv(self, output, options);
	}
}

#[cfg(feature = "serde-json")]
impl<'h> Deserialise<'h> for ::serde_json::Map<String, ::serde_json::Value> {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		let len = deserialise_map_len(input.read_byte()?, input)?;
		deserialise_map_kv(len, input).collect()
	}
}
*/
