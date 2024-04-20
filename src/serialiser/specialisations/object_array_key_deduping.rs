use super::{ *, marker::*, integer::* };
use ::hashbrown::HashMap;
use ::std::collections::BTreeSet;
use ::std::hash::Hash;

pub fn serialise_object_array_with_key_deduping<K, V, B>(objects: &[&HashMap<K, V>], output: &mut B, options: &Options)
where
	K: Serialise + Ord,
	V: Serialise,
	B: BufferImplWrite
{
	let len = objects.len();
	let mut keys = BTreeSet::new();

	for obj in objects {
		for k in obj.keys() {
			keys.insert(k);
		}
	}

	let marker_keys = get_marker_type(keys.len() as u64);
	let marker_len = get_marker_type(objects.len() as u64);

	use MarkerType::*;
	let marker = match (marker_keys, marker_len) {
		(M8, M8) => { MARKER_OBJ_ARRAY_KEY_DEDUP_8KEY_8LEN }
		(M8, M16) => { MARKER_OBJ_ARRAY_KEY_DEDUP_8KEY_16LEN }
		(M8, MXL) => { MARKER_OBJ_ARRAY_KEY_DEDUP_8KEY_XLLEN }
		(M16, M8) => { MARKER_OBJ_ARRAY_KEY_DEDUP_16KEY_8LEN }
		(M16, M16) => { MARKER_OBJ_ARRAY_KEY_DEDUP_16KEY_16LEN }
		(M16, MXL) => { MARKER_OBJ_ARRAY_KEY_DEDUP_16KEY_XLLEN }
		(MXL, M8) => { MARKER_OBJ_ARRAY_KEY_DEDUP_XLKEY_8LEN }
		(MXL, M16) => { MARKER_OBJ_ARRAY_KEY_DEDUP_XLKEY_16LEN }
		(MXL, MXL) => { MARKER_OBJ_ARRAY_KEY_DEDUP_XLKEY_XLLEN }
	};

	output.write_byte(marker);
	serialise_length_3_variants_with_type(marker_keys, keys.len() as u64, output, options);

	let mut key_vec = Vec::with_capacity(keys.len());
	for key in keys {
		key.serialise(output, options);
		key_vec.push(key);
	}

	serialise_length_3_variants_with_type(marker_len, objects.len() as u64, output, options);
	for obj in objects {
		for (k, v) in *obj {
			let key_index = key_vec.binary_search(&k).expect("key should have been in set");
			serialise_length_3_variants_with_type(marker_keys, key_index as u64, output, options);

			v.serialise(output, options);
		}
	}
}
