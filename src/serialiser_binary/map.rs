use super::internal_prelude::*;
use super::USizeSerialiser;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap as HashbrownHashMap;
use std::collections::HashMap as StdHashMap;

// TODO: deterministic wrapper? cause hashmap iter ordering is nondeterministic

impl<K, V, S> Serialise for StdHashMap<K, V, S>
where
	K: Serialise,
	V: Serialise
{
	type Serialiser<'h> = MapSerialiser<'h, K, V> where K: 'h, V: 'h, S: 'h;

	fn build_serialiser(&self) -> MapSerialiser<'_, K, V> {
		MapSerialiser::new(self)
	}
}

// TODO: allocator API?
#[cfg(feature = "hashbrown")]
impl<K, V, S> Serialise for HashbrownHashMap<K, V, S>
where
	K: Serialise,
	V: Serialise
{
	type Serialiser<'h> = MapSerialiser<'h, K, V> where K: 'h, V: 'h, S: 'h;

	fn build_serialiser(&self) -> MapSerialiser<'_, K, V> {
		MapSerialiser::new(self)
	}
}

pub struct MapSerialiser<'h, K: Serialise + 'h, V: Serialise + 'h> {
	kv: Vec<(K::Serialiser<'h>, V::Serialiser<'h>)>,
	len_ser: Option<USizeSerialiser>
}

impl<'h, K, V> MapSerialiser<'h, K, V>
where
	K: Serialise,
	V: Serialise
{
	fn new<I: IntoIterator<Item = (&'h K, &'h V)>>(iter: I) -> Self {
		let kv = iter.into_iter()
			.map(|(k, v)| (k.build_serialiser(), v.build_serialiser()))
			.collect::<Vec<_>>();

		let len_ser = if kv.len() > u8::MAX.into_usize() {
			Some(USizeSerialiser::new(kv.len()))
		} else {
			None
		};

		Self { kv, len_ser }
	}
}

impl<'h, K, V> Serialiser<'h> for MapSerialiser<'h, K, V>
where
	K: Serialise,
	V: Serialise
{
	unsafe fn needed_capacity(&self) -> usize {
		let len_ser = if let Some(len_ser) = &self.len_ser {
			// marker + length serialised
			1 + len_ser.needed_capacity()
		} else {
			// marker + one byte for len
			2
		};

		let items = self.kv.iter()
			.map(|(k, v)| k.needed_capacity() + v.needed_capacity())
			.sum::<usize>();

		len_ser + items
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = &self.len_ser {
			buf.write_byte(MARKER_MAP_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_MAP_8);
			buf.write_byte(self.kv.len().into_u8_lossy());
		}

		for (k, v) in &self.kv {
			k.serialise(buf);
			v.serialise(buf);
		}
	}
}
