pub fn macro_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut iter = input.into_iter();
	let proc_macro::TokenTree::Literal(size) = iter.next().expect("expected int size") else {
		panic!("expected literal for the int size");
	};
	let size = size.to_string().parse().expect("expected number literal for int size");
	assert!(iter.next().is_none(), "expected input to consist of just the int size");

	let int_info = get_int_info_for(size);
	// process(size);
	// process_packed(size);
	// process_packed_smaller_reprs(size);

	// let temp = "const TEST_".to_string() + &*size.to_string() + "" + (int_info.largest_packed_int.0 as u8).to_string()
	let temp = format!(
		r#"const TEST_{size}: &str = "u{size} containing {}xu{}s";"#,
		int_info.max_packed_int.1,
		int_info.max_packed_int.0 as u8
	);

	temp.parse().unwrap()
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum InnerIntType {
	U8 = 8,
	U16 = 16,
	U32 = 32,
	U64 = 64,
	U128 = 128
}

struct IntInfo {
	/// the size of the int (ie. macro input)
	///
	/// `u16` allows theoretically generating ints of up to 65535 bits... but why? lol
	size: u16,
	/// the minimum sized single integer that can hold it
	///
	/// `Option` in case the passed in type is a bigint (ie. more than 128)
	min_single_int: Option<InnerIntType>,
	/// the largest int and amount of that int that is able to hold
	/// this int type while wasting less than 8 bits
	///
	/// - 1 to 8 use u8
	/// - 9 to 16 use u16 (no benefit in 2xu8)
	/// - 17 to 24 use 3xu8
	/// - 25 to 32 use u32
	/// - 33 to 40 use 5xu8
	/// - 41 to 48 use 3xu16
	/// - 49 to 56 use 7xu8
	/// - 57 to 64 use u64
	/// - etc etc
	///
	/// reprs with smaller internal int types can be calculated from this
	/// (eg. u48 can be represented as 3 `u16`, or 6 `u8`)
	max_packed_int: (InnerIntType, u16)
}

fn get_int_info_for(size: u16) -> IntInfo {
	use InnerIntType::*;

	let min_single_int = match size {
		..=8 => { Some(U8) }
		9..=16 => { Some(U16) }
		17..=32 => { Some(U32) }
		33..=64 => { Some(U64) }
		65..=128 => { Some(U128) }
		129.. => { None }
	};

	let max_packed_int = [U128, U64, U32, U16, U8].into_iter()
		.filter_map(|width| try_get_largest_packed_int(size, width))
		.next()
		.expect("invalid state kinda weird");

	IntInfo { size, min_single_int, max_packed_int }
}

/// Tries to see if the given int `size` can be fit in a number of `width` int types
/// while wasting less than 8 bits.
///
/// `size` is macro input, `width` is the width of the std int type
fn try_get_largest_packed_int(size: u16, width: InnerIntType) -> Option<(InnerIntType, u16)> {
	let width_int = width as u16;

	// width should be less than 8 larger than size
	if width_int - 7 > size { return None }

	match size % width_int {
		0 => { Some((width, size / width_int)) }
		r if width_int - r < 8 => { Some((width, (size / width_int) + 1)) }
		_ => { None }
	}
}

fn process(size: u16) {}

fn process_packed(size: u16) {}

fn process_packed_smaller_reprs(size: u16) {}
