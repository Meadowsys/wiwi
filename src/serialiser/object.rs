use super::{ *, error::*, marker::* };
use ::hashbrown::HashMap;
use ::std::hash::Hash;

pub fn serialise_hashbrown_into_object<K, V, B>(object: &HashMap<K, V>, output: &mut B)
where
	K: Serialise,
	V: Serialise,
	B: BufferImplWrite
{
	serialise_length_3_variants(SerialiseLength3VariantsParams {
		marker_8: MARKER_OBJECT_8,
		marker_16: MARKER_OBJECT_16,
		marker_xl: MARKER_OBJECT_XL,
		len: object.len(),
		output
	});

	for (k, v) in object {
		k.serialise(output);
		v.serialise(output);
	}
}

pub fn deserialise_rest_of_object_into_hashbrown<'h, K, V, B>(
	marker_type: MarkerType,
	input: &mut B
) -> Result<HashMap<K, V>>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	B: BufferImplRead<'h>
{
	let len = deserialise_rest_of_length_3_variants(marker_type, input)?;
	let mut map = HashMap::with_capacity(len);

	for _ in 0..len {
		let k = K::deserialise(input)?;
		let v = V::deserialise(input)?;
		map.insert(k, v);
	}

	Ok(map)
}
