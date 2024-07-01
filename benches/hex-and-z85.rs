#![allow(dead_code, unused_imports, unused_variables)]

use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use rand::{ Rng, thread_rng };

fn benchmark(c: &mut Criterion) {
	const FIFTY_MIB: usize = 50 * 1024 * 1024;

	let mut rng = thread_rng();
	let mut bytes = vec![0u8; FIFTY_MIB];
	rng.fill(&mut *bytes);
	let bytes = black_box(&*bytes);

	let encoded_z85 = z85::encode(bytes);
	let encoded_z85 = black_box(encoded_z85.as_bytes());
	let encoded_z85_wiwi = wiwi::z85::encode_z85(bytes);
	let encoded_z85_wiwi = black_box(encoded_z85_wiwi.as_bytes());

	let encoded_hex = hex::encode(bytes);
	let encoded_hex = black_box(encoded_hex.as_bytes());
	let encoded_hex_wiwi = wiwi::hex::encode_hex(bytes);
	let encoded_hex_wiwi = black_box(encoded_hex_wiwi.as_bytes());

	c
		.bench_function("(50MB) z85 encode", |b| b.iter(|| {
			let _: String = z85::encode(bytes);
		}))
		.bench_function("(50MB) z85 decode", |b| b.iter(|| {
			let _: Vec<u8> = z85::decode(encoded_z85).unwrap();
		}))
		.bench_function("(50MB) z85 encode (wiwi)", |b| b.iter(|| {
			let _: String = wiwi::z85::encode_z85(bytes);
		}))
		.bench_function("(50MB) z85 decode (wiwi)", |b| b.iter(|| {
			let _: Vec<u8> = wiwi::z85::decode_z85(encoded_z85_wiwi).unwrap();
		}))

		.bench_function("(50MB) hex encode", |b| b.iter(|| {
			let _: String = hex::encode(bytes);
		}))
		.bench_function("(50MB) hex decode", |b| b.iter(|| {
			let _: Vec<u8> = hex::decode(encoded_hex).unwrap();
		}))
		.bench_function("(50MB) hex encode (wiwi)", |b| b.iter(|| {
			let _: String = wiwi::hex::encode_hex(bytes);
		}))
		.bench_function("(50MB) hex decode (wiwi)", |b| b.iter(|| {
			let _: Vec<u8> = wiwi::hex::decode_hex(encoded_hex_wiwi).unwrap();
		}))

		// // .bench_function("(50MB) base32 encode", |b| b.iter(|| {}))
		// // .bench_function("(50MB) base32 decode", |b| b.iter(|| {}))
		// .bench_function("(50MB) base32 encode (wiwi)", |b| b.iter(|| {
		// 	let _: String = wiwi::base32::encode_base32(bytes);
		// }))
		// // .bench_function("(50MB) base32 decode (wiwi)", |b| b.iter(|| {}))

		// // .bench_function("(50MB) base32hex encode", |b| b.iter(|| {}))
		// // .bench_function("(50MB) base32hex decode", |b| b.iter(|| {}))
		// .bench_function("(50MB) base32hex encode (wiwi)", |b| b.iter(|| {
		// 	let _: String = wiwi::base32::encode_base32hex(bytes);
		// }))
		// // .bench_function("(50MB) base32hex decode (wiwi)", |b| b.iter(|| {}))

		// // .bench_function("(50MB) base64 encode", |b| b.iter(|| {}))
		// // .bench_function("(50MB) base64 decode", |b| b.iter(|| {}))
		// .bench_function("(50MB) base64 encode (wiwi)", |b| b.iter(|| {
		// 	let _: String = wiwi::base64::encode_base64(bytes);
		// }))
		// // .bench_function("(50MB) base64 decode (wiwi)", |b| b.iter(|| {}))

		// // .bench_function("(50MB) base64url encode", |b| b.iter(|| {}))
		// // .bench_function("(50MB) base64url decode", |b| b.iter(|| {}))
		// .bench_function("(50MB) base64url encode (wiwi)", |b| b.iter(|| {
		// 	let _: String = wiwi::base64::encode_base64url(bytes);
		// }))
		// // .bench_function("(50MB) base64url decode (wiwi)", |b| b.iter(|| {}))
	;
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
