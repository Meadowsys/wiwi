use super::{ *, marker::*, integer::* };
use hashbrown::HashMap;
use std::collections::BTreeSet;
use std::hash::Hash;

pub fn serialise_object_array_with_key_deduping<K, V, B>(objects: &[&HashMap<K, V>], output: &mut B, options: &Options)
where
	K: Serialise + Ord,
	V: Serialise,
	B: BufferImplWrite
{
	let mut keys = BTreeSet::new();
	let mut max_pairs_len = 0u64;

	for obj in objects {
		max_pairs_len = u64::max(max_pairs_len, obj.len() as _);

		for k in obj.keys() {
			keys.insert(k);
		}
	}

	let marker_unique_keys = get_marker_type(keys.len() as u64);
	let marker_len = get_marker_type(objects.len() as u64);
	let marker_kv_pairs = get_marker_type(max_pairs_len);
	let variants_byte = make_variants_byte(
		Some(marker_unique_keys),
		Some(marker_len),
		Some(marker_kv_pairs),
		None
	);

	output.write_byte(MARKER_OBJ_ARRAY_KEY_DEDUP);
	output.write_byte(variants_byte);

	serialise_len_3_variants_with_type(
		marker_unique_keys,
		keys.len() as _,
		output,
		options
	);

	let mut key_vec = Vec::with_capacity(keys.len());
	for key in keys {
		key.serialise(output, options);
		key_vec.push(key);
	}

	serialise_len_3_variants_with_type(
		marker_len,
		objects.len() as _,
		output,
		options
	);

	for obj in objects {
		serialise_len_3_variants_with_type(marker_kv_pairs, obj.len() as _, output, options);

		for (k, v) in *obj {
			let key_index = key_vec.binary_search(&k)
				.expect("key should have been in set");

			serialise_len_3_variants_with_type(
				marker_unique_keys,
				key_index as _,
				output,
				options
			);
			v.serialise(output, options);
		}
	}
}

pub fn deserialise_rest_of_object_array_with_key_deduping<'h, K, V, B>(input: &mut B) -> Result<Vec<HashMap<K, V>>>
where
	K: Deserialise<'h> + Clone + Eq + Hash,
	V: Deserialise<'h>,
	B: BufferImplRead<'h>
{
	let (
		Some(marker_unique_keys),
		Some(marker_len),
		Some(marker_kv_pairs),
		None
	) = decode_variants_byte(input.read_byte()?) else {
		return err("invalid variants info")
	};

	let amount_of_keys = deserialise_rest_of_length_3_variants(marker_unique_keys, input)?;
	let keys = (0..amount_of_keys)
		.map(|_| K::deserialise(input))
		.collect::<Result<Vec<_>>>()?;

	let array_len = deserialise_rest_of_length_3_variants(marker_len, input)?;
	(0..array_len)
		.map(|_array_i| {
			let obj_len = deserialise_rest_of_length_3_variants(marker_kv_pairs, input)?;
			(0..obj_len)
				.map(|_| {
					let k_index = deserialise_rest_of_length_3_variants(marker_unique_keys, input)?;
					let key = keys.get(k_index)
						.err_f(|| format!(
							"tried to get key index {k_index}, but there are only {amount_of_keys} key{s}",
							s = if amount_of_keys != 1 { "s" } else { "" }
						))?;
					let value = V::deserialise(input)?;

					Ok((key.clone(), value))
				})
				.collect::<Result<HashMap<_, _>>>()
		})
		.collect()
}
