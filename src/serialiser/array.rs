use super::{ *, error::*, integer::*, marker::* };

pub fn serialise_array<T: Serialise, B: BufferImplWrite>(arr: &[T], output: &mut B) {
	serialise_length_3_variants(SerialiseLength3VariantsParams {
		marker_8: MARKER_ARRAY_8,
		marker_16: MARKER_ARRAY_16,
		marker_xl: MARKER_ARRAY_XL,
		len: arr.len(),
		output
	});

	// TODO: this is not good for auto vectorisation
	arr.iter().for_each(|item| item.serialise(output));
}

#[inline]
pub fn deserialise_rest_of_array<'h, V: Deserialise<'h>, B: BufferImplRead<'h>>(
	marker_type: MarkerType,
	input: &mut B
) -> Result<Vec<V>> {
	let len = deserialise_rest_of_length_3_variants(marker_type, input)?;
	(0..len).map(|_| V::deserialise(input)).collect()
}
