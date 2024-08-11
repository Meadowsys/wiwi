fn main() {
	println!("cargo::rustc-check-cfg=cfg(kiwingay)");

	// #[cfg(feature = "gpg-unstable")] {
	// 	if let Ok(path) = dotenvy::dotenv() {
	// 		if let Some(path) = path.as_os_str().to_str() {
	// 			println!("cargo::rerun-if-changed={}", path);
	// 		}
	// 	}
	// 	println!("cargo::rerun-if-env-changed=WIWI_GPG_SOURCE_DIR");
	// 	println!("cargo::rerun-if-env-changed=WIWI_LIBGPG_ERROR_DIR");
	// 	println!("cargo::rerun-if-env-changed=WIWI_LIBASSUAN_SOURCE_DIR");
	// }
}
