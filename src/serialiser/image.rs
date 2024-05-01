use super::{ buffer::*, core::*, error::*, integer::* };
use image::codecs::png::{ PngDecoder, PngEncoder };
use image::{ ExtendedColorType, ImageEncoder, ImageDecoder, RgbImage, RgbaImage };
use std::{ io::Cursor, ptr, slice };

pub fn serialise_to_png<T: ?Sized + Serialise>(item: &T) -> Vec<u8> {
	serialise_to_png_with_options(item, &ImageOptions::default())
}

pub fn serialise_to_png_with_options<T: ?Sized + Serialise>(item: &T, options: &ImageOptions) -> Vec<u8> {
	let serialised = serialise_with_options(item, &options.reg_options);
	let len = serialise_with_options(&(serialised.len() as u64), &options.reg_options);

	let total_len = (serialised.len() + len.len()) as u64;

	// 3 here assumes 3 channels per pixel (rgb 8 bit)
	let full_pixels = total_len / 3;
	let partial_pixel = (total_len % 3 != 0) as u64;
	let required_pixels = full_pixels + partial_pixel;

	let (width, height) = match (options.width, options.height) {
		(None, None) => {
			get_optimal_dimensions(required_pixels)
		}
		(Some(w), Some(h)) if (w as u64) * (h as u64) < required_pixels => {
			// provided dimensions are too small
			get_optimal_dimensions(required_pixels)
		}
		(Some(w), Some(h)) => {
			// dimensions fit the image, (checked above)
			(w, h)
		}
		(Some(w), None) => {
			let full_h = (required_pixels / w as u64) as u32;
			let remainder_h = (required_pixels % w as u64 != 0) as u32;
			let h = full_h + remainder_h;
			(w, h)
		}
		(None, Some(h)) => {
			let full_w = (required_pixels / h as u64) as u32;
			let remainder_w = (required_pixels % h as u64 != 0) as u32;
			let w = full_w + remainder_w;
			(w, h)
		}
	};

	let mut image = RgbImage::new(width, height);
	let image_slice = &mut *image;

	debug_assert!(image_slice.len() == height as usize * width as usize * 3);
	debug_assert!(image_slice.len() as u64 >= total_len);

	unsafe {
		let image_ptr = image_slice as *mut [u8] as *mut u8;

		let len_ptr = &*len as *const [u8] as *const u8;
		let len_count = len.len();
		ptr::copy_nonoverlapping(len_ptr, image_ptr, len_count);

		let data_ptr = &*serialised as *const [u8] as *const u8;
		let data_count = serialised.len();
		ptr::copy_nonoverlapping(data_ptr, image_ptr.add(len_count), data_count);
	}

	let mut output = Vec::new();
	PngEncoder::new(&mut output)
		.write_image(image_slice, width, height, ExtendedColorType::Rgb8)
		.expect("png encoding failed");
	output
}

pub fn deserialise_from_png<T: for<'h> Deserialise<'h>>(img_bytes: &[u8]) -> Result<T> {
	let decoder = PngDecoder::new(Cursor::new(img_bytes))
		.convert_err()?;
	let mut bytes = vec![0; decoder.total_bytes() as usize];
	decoder.read_image(&mut bytes).convert_err()?;
	let mut bytes = &*bytes;

	let len = u64::deserialise(&mut bytes)?;
	if len > usize::MAX as u64 { return err("input length overflowed usize") }

	let len = len as usize;
	if len > bytes.len() { return err("invalid input") }

	deserialise(unsafe { slice::from_raw_parts(bytes as *const [u8] as *const u8, len) })
}

/// returns (width, height)
fn get_optimal_dimensions(required_pixels: u64) -> (u32, u32) {
	// prefer wider than taller
	// this function returns dimensions that are square, width 4x height,
	// or somewhere in between
	// if there is a better way to do that below match statement,
	// I would like to know, lol

	macro_rules! do_width {
		($w:ident, $shift:literal, $mask:literal) => {
			($w >> $shift) as u32 + ($w & $mask != 0) as u32
		}
	}

	match required_pixels {
		w @                 ..=0x4 => { (w as u32, 1) }
		w @               ..=0x1_0 => { (do_width!(w,  1,        0x1),        0x2) }
		w @               ..=0x4_0 => { (do_width!(w,  2,        0x3),        0x4) }
		w @              ..=0x1_00 => { (do_width!(w,  3,        0x7),        0x8) }
		w @              ..=0x4_00 => { (do_width!(w,  4,        0xf),       0x10) }
		w @             ..=0x1_000 => { (do_width!(w,  5,       0x1f),       0x20) }
		w @             ..=0x4_000 => { (do_width!(w,  6,       0x3f),       0x40) }
		w @            ..=0x1_0000 => { (do_width!(w,  7,       0x7f),       0x80) }
		w @            ..=0x4_0000 => { (do_width!(w,  8,       0xff),      0x100) }
		w @           ..=0x1_00000 => { (do_width!(w,  9,      0x1ff),      0x200) }
		w @           ..=0x4_00000 => { (do_width!(w, 10,      0x3ff),      0x400) }
		w @          ..=0x1_000000 => { (do_width!(w, 11,      0x7ff),      0x800) }
		w @          ..=0x4_000000 => { (do_width!(w, 12,      0xfff),     0x1000) }
		w @         ..=0x1_0000000 => { (do_width!(w, 13,     0x1fff),     0x2000) }
		w @         ..=0x4_0000000 => { (do_width!(w, 14,     0x3fff),     0x4000) }
		w @        ..=0x1_00000000 => { (do_width!(w, 15,     0x7fff),     0x8000) }
		w @        ..=0x4_00000000 => { (do_width!(w, 16,     0xffff),    0x10000) }
		w @       ..=0x1_000000000 => { (do_width!(w, 17,    0x1ffff),    0x20000) }
		w @       ..=0x4_000000000 => { (do_width!(w, 18,    0x3ffff),    0x40000) }
		w @      ..=0x1_0000000000 => { (do_width!(w, 19,    0x7ffff),    0x80000) }
		w @      ..=0x4_0000000000 => { (do_width!(w, 20,    0xfffff),   0x100000) }
		w @     ..=0x1_00000000000 => { (do_width!(w, 21,   0x1fffff),   0x200000) }
		w @     ..=0x4_00000000000 => { (do_width!(w, 22,   0x3fffff),   0x400000) }
		w @    ..=0x1_000000000000 => { (do_width!(w, 23,   0x7fffff),   0x800000) }
		w @    ..=0x4_000000000000 => { (do_width!(w, 24,   0xffffff),  0x1000000) }
		w @   ..=0x1_0000000000000 => { (do_width!(w, 25,  0x1ffffff),  0x2000000) }
		w @   ..=0x4_0000000000000 => { (do_width!(w, 26,  0x3ffffff),  0x4000000) }
		w @  ..=0x1_00000000000000 => { (do_width!(w, 27,  0x7ffffff),  0x8000000) }
		w @  ..=0x4_00000000000000 => { (do_width!(w, 28,  0xfffffff), 0x10000000) }
		w @ ..=0x1_000000000000000 => { (do_width!(w, 29, 0x1fffffff), 0x20000000) }
		w @ ..=0x4_000000000000000 => { (do_width!(w, 30, 0x3fffffff), 0x40000000) }
		w                          => { (do_width!(w, 31, 0x7fffffff), 0x80000000) }
	}
}

pub struct ImageOptions {
	pub width: Option<u32>,
	pub height: Option<u32>,
	pub reg_options: Options
}

impl Default for ImageOptions {
	fn default() -> Self {
		let width = None;
		let height = None;
		let reg_options = Options::default();
		Self { width, height, reg_options }
	}
}
