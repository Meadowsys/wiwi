use std::fs;

// TODO: enhancements for the future:
// - could combine the two loops on features, by storing it in 3 more intermediate vars,
//   rather than sorting out refs then putting them in? if that makes sense
// - checking for unused dependencies
// - a way to declare dependencies as used, even though no feature depends on them
//   (likely a #[used] attr or something)

fn main() {
	let mut _dependencies = decl_dependencies! {
		"argon2 0.5.3"
		"chacha20poly1305 0.10.1"
		"chrono 0.4.35" features: ["clock"]
		"hashbrown 0.14.3" features: ["ahash"]
		"httparse 1.8.0"
		"image 0.25.1"
		"ordered-float 4.2.0"
		"p384 0.13.0"
		#[uses_underscore]
		"parking-lot 0.12.1"
		"rand 0.8.5"
		#[uses_underscore]
		"rand-chacha 0.3.1"
		"serde 1.0.204"
		#[uses_underscore]
		"serde-json 1.0.116"
		#[uses_underscore]
		"spin-sleep 1.2.0"
		"thiserror 1.0.58"
		"tokio 1.36.0" features: ["full"]
		"url 2.5.0"
		"uuid 1.8.0"
		"zeroize 1.7.0"
	};

	let _features = decl_features! {
		#[unstable]
		"aoc"
		"Utilities specific for writing solutions for [Advent of Code](https://adventofcode.com)"
		features: ["iter"]

		"augment-panic-hook"
		"Conveniently augment the panic hook (instead of replacing it), running some of your code before calling the existing one"

		#[unstable]
		"auth"
		"Lower-level(ish) utilities for writing an authentication system, in which the client password is never sent over the wire"
		dependencies: ["argon2", "chacha20poly1305", "chrono", "p384", "rand", "thiserror", "zeroize"]
		features: ["rand", "z85"]

		#[unstable]
		"bitstream"
		"Encoder and decoder for a stream of bits"
		features: ["num-traits"]

		#[unstable]
		"chainer"
		"Wrappers around common structs that provide chaining APIs (take ownership, do operation, then return back)"

		#[unstable]
		"cli"
		"CLI arguments parser"

		"clock-timer"
		"An interval tracking clock, yielding ticks at specified intervals and doing so for a specified period of time"
		dependencies: ["chrono", "tokio"]

		"debounce"
		"Delay calling a function until a specified period of time has passed since the last time it was called"
		dependencies: ["chrono", "tokio"]
		features: ["num-traits"]

		#[unstable]
		"defer"
		"Defer running code until the end of the current scope or until something gets dropped"

		"export-all-submodules"
		"A convenience macro for declaring many private modules, then reexporting everything within them using a glob use statement"

		#[unstable]
		"gpg"
		"Bindings to [GPGME](https://www.gnupg.org/software/gpgme), GnuPG's official C library"
		features: ["libassuan", "libgpg-error"]

		"h"
		"h"

		"hex"
		"Fast hex encoder and decoder for both upper hex and lower hex"
		dependencies: ["thiserror"]
		features: ["num-traits"]

		#[unstable]
		"id"
		"ID generators, of various output sizes, and guarantees of uniqueness and ordering"
		features: ["export-all-submodules", "num-traits", "rand"]

		#[unstable]
		"int"
		"Bigints (ex. u384) and uneven sized ints (ex. u15)"
		dependencies: ["macro"]
		features: ["num-traits"]

		#[unstable]
		"iter"
		"Iterator utilities"

		"lazy-wrap"
		"Wrappers around a lazily initialised value, and its initialisation function, supporting usage in static variables"
		dependencies: ["parking-lot"]

		#[unstable]
		"libassuan"
		"Bindings to [`libassuan`](https://www.gnupg.org/related_software/libassuan)"

		#[unstable]
		"libgpg-error"
		"Bindings to [`libgpg-error`](https://www.gnupg.org/related_software/libgpg-error)"

		#[unstable]
		"lsl"
		"Experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol"
		dependencies: ["hashbrown"]
		features: ["id"]

		#[unstable]
		"mcu"
		"[Material colour utilities](https://github.com/material-foundation/material-color-utilities)"
		features: ["num-traits"]

		#[unstable]
		"memory-usage"
		"Calculate actual memory usage of Rust structs, including derive macro for custom types (not the same as `size_of::<T>()`)"
		dependencies: ["macro"]

		#[unstable]
		"minesweeper"
		"Core logic components for minesweeper games of arbitrary size"
		dependencies: ["rand"]
		features: ["chainer", "iter", "num-traits", "z85"]

		"nominal"
		"Generic newtype wrappers, for increased type safety through Rust's nominal type system"

		#[unstable]
		"num-traits"
		"More traits for numbers and their various functionality, including things like [`MulWidening`]"
		features: ["export-all-submodules", "int"]

		#[unstable]
		"path"
		"UTF-8 only path manipulation utilities written from scratch"

		"rand"
		"More random number generators and utilities"
		dependencies: ["rand", "rand-chacha"]

		#[unstable]
		"serialiser-binary"
		"Self describing and stable binary format/serialiser, aiming for small output size"
		dependencies: ["macro", "ordered-float"]
		features: ["defer", "num-traits"]

		#[unstable]
		"serialiser-binary-2"
		"Self describing and stable binary format/serialiser, aiming for small output size (v2)"
		dependencies: ["macro"]
		features: ["defer", "num-traits"]

		#[unstable]
		"serialiser-text"
		"Self describing and stable text format/serialiser, aiming to be easy to read and write by hand"

		#[unstable]
		"string-pool"
		"Global immutable string pool and String type (à la Java)"
		dependencies: ["hashbrown"]
		features: ["lazy-wrap"]

		#[unstable]
		"sudoku"
		"Sudoku related... stuff"
		features: ["chainer", "iter", "num-traits"]

		#[unstable]
		"unicode"
		"Implementation of the [Unicode](https://home.unicode.org) standard, including UTF-8, UTF-16, and UTF-32 strings"
		dependencies: ["macro"]

		"with-cloned"
		"Convenience macro for the clone-and-move pattern (yes, we're calling it that :p)"

		"z85"
		"Fast encoder and decoder for [ZeroMQ](https://zeromq.org)'s [zZ85](https://rfc.zeromq.org/spec/32) format"
		dependencies: ["thiserror"]
		features: ["num-traits"]

		#[addon]
		"hashbrown"
		"Adds integration with `hashbrown` crate (where applicable)"
		dependencies: ["hashbrown"]

		#[addon]
		"image"
		"Adds integration with `image` crate (where applicable)"
		dependencies: ["image"]

		#[addon]
		"large-tuples"
		"By default, implementations on tuples are available for tuples with up to 8 elements, which should be enough for most uses. Enabling this feature will enable implementations for tuples with up to 32 elements"

		#[addon]
		"nightly"
		"Enable features only available in nightly Rust"

		#[addon]
		"omega-tuples-of-doom"
		"_Surely_, no one uses tuples with more than 32 elements in them... but we don't know everyone's use case, so this feature will enable implementations for tuples with up to 128 elements. _Hopefully_, that is enough for everything. :p"
		features: ["large-tuples"]

		#[addon]
		"serde"
		"Adds integration with `serde` crate (where applicable)"
		dependencies: ["serde"]

		#[addon]
		"serde-json"
		"Adds integration with `serde-json` crate (where applicable)"
		dependencies: ["serde-json"]
	};

	// <[_]>::is_sorted_by is unstable, so we copy its impl lol
	// array_windows is also unstable, brrt
	// TODO: use slice chain?
	macro_rules! assert_sorted {
		($arr:ident, $key_fn:expr) => {
			let key_fn = $key_fn;
			let res = $arr.windows(2)
				.all(|slice| {
					let [a, b]: &[_; 2] = slice.try_into().unwrap();
					key_fn(a) < key_fn(b)
				});
			assert!(res, concat!(stringify!($arr), " are not all sorted"));
		}
	}
	assert_sorted!(_dependencies, |d: &Dependency| d.name);
	assert_sorted!(_features, |f: &Feature| (
		matches!(f.feature_type, FeatureType::Addon),
		f.name
	));

	let wiwi_manifest = "Cargo.toml";
	let wiwi_manifest_start_marker = "# ----- start autogenerated region (see gen-features script) -----";
	let wiwi_manifest_end_marker = "# ----- end autogenerated region -----";

	let wiwi_readme = "README.md";
	let wiwi_readme_start_marker = "<!-- ----- start autogenerated region (see gen-features script) ----- -->";
	let wiwi_readme_end_marker = "<!-- ----- end autogenerated region ----- -->";

	let wiwi_lib = "src/lib.rs";
	let wiwi_lib_start_marker = "// ----- start autogenerated region (lib) (see gen-features script) -----";
	let wiwi_lib_end_marker = "// ----- end autogenerated region (lib) -----";

	let wiwi_doc_cfgs = "src/lib.rs";
	let wiwi_doc_cfgs_start_marker = "// ----- start autogenerated region (doc cfgs) (see gen-features script) -----";
	let wiwi_doc_cfgs_end_marker = "// ----- end autogenerated region (doc cfgs) -----";

	let wiwi_prelude_cfgs = "src/prelude.rs";
	let wiwi_prelude_cfgs_start_marker = "// ----- start autogenerated region (see gen-features script) -----";
	let wiwi_prelude_cfgs_end_marker = "// ----- end autogenerated region -----";

	let mut generated_manifest = String::new();
	generated_manifest += concat!(
		"[dependencies]\nwiwiwiwiwi = { path = \"macro\", version = \"=",
		std::env!("CARGO_PKG_VERSION"),
		"\" }\n\n"
	);

	let mut generated_readme_stable = String::new();
	generated_readme_stable += "### Stable features\n\n";

	let mut generated_readme_unstable = String::new();
	generated_readme_unstable += "### Unstable features\n\n";
	generated_readme_unstable += "reminder: **Unstable features are NOT covered by semver!**\n\n";

	let mut generated_readme_addons = String::new();
	generated_readme_addons += "### Addon features\n\n";

	let mut generated_lib = String::new();

	let mut generated_doc_cfgs_list = String::new();
	let mut generated_doc_cfgs_no_features_enabled = String::new();
	generated_doc_cfgs_no_features_enabled += "#![cfg_attr(all(\n";

	for Dependency {
		name,
		version,
		features,
		uses_underscore,
		has_dependent: _
	} in &_dependencies {
		if *uses_underscore {
			generated_manifest += &name.replace('-', "_");
		} else {
			generated_manifest += name;
		}

		generated_manifest += " = { version = \"";
		generated_manifest += version;
		generated_manifest += "\", optional = true";

		if let [first, rest @ ..] = features {
			generated_manifest += ", features = [\"";
			generated_manifest += first;

			for feature in rest {
				generated_manifest += "\", \"";
				generated_manifest += feature;
			}

			generated_manifest += "\"]";
		}

		generated_manifest += " }\n";
	}
	generated_manifest += "\n";

	let mut switched_to_addons = false;

	generated_manifest += "[features]\n";

	let mut all_refs = Vec::new();
	let mut all_unstable_refs = Vec::new();
	let mut all_addons_refs = Vec::new();

	for Feature {
		name,
		feature_type,
		desc,
		dependencies: _,
		features: _
	} in &_features {
		macro_rules! append_feature {
			($output:ident) => {
				$output += "- **`";
				$output += name;
				$output += "`**";

				if desc.len() > 0 {
					$output += " - ";
					$output += desc;
				}

				$output += "\n";
			}
		}

		macro_rules! push_feature {
			($unstable:literal) => {
				let name_with_underscores = name.replace('-', "_");
				generated_lib += "#[cfg(feature = \"";
				generated_lib += name;
				if $unstable { generated_lib += "-unstable" }
				generated_lib += "\")]\n#[cfg_attr(docsrs, doc(cfg(feature = \"";
				generated_lib += name;
				if $unstable { generated_lib += "-unstable" }
				generated_lib += "\")))]\n/// ";
				generated_lib += desc;
				generated_lib += "\n#[doc = \"\"]\n#[doc = include_str!(\"./";
				generated_lib += &name_with_underscores;
				generated_lib += "/README.md\")]\npub mod ";
				generated_lib += &name_with_underscores;
				generated_lib += ";\n\n";

				generated_doc_cfgs_list += "#![cfg_attr(all(not(any(docsrs, kiwingay)), feature = \"";
				generated_doc_cfgs_list += name;
				if $unstable { generated_doc_cfgs_list += "-unstable" }
				generated_doc_cfgs_list += "\"), doc = \"- `";
				generated_doc_cfgs_list += name;
				generated_doc_cfgs_list += "`";
				if $unstable { generated_doc_cfgs_list += " (unstable)" }
				generated_doc_cfgs_list += "\")]\n";

				generated_doc_cfgs_no_features_enabled += "\tnot(feature = \"";
				generated_doc_cfgs_no_features_enabled += name;
				if $unstable { generated_doc_cfgs_no_features_enabled += "-unstable" }
				generated_doc_cfgs_no_features_enabled += "\"),\n";
			}
		}

		match feature_type {
			FeatureType::Stable => {
				all_refs.push(*name);
				append_feature!(generated_readme_stable);
				push_feature!(false);
			}
			FeatureType::Unstable => {
				all_unstable_refs.push(*name);
				append_feature!(generated_readme_unstable);
				push_feature!(true);
			}
			FeatureType::Addon => {
				all_addons_refs.push(*name);
				append_feature!(generated_readme_addons);
			}
		}
	}

	generated_manifest += "all = [";
	if let [first, rest @ ..] = &*all_refs {
		generated_manifest += "\"";
		generated_manifest += first;

		for feature in rest {
			generated_manifest += "\", \"";
			generated_manifest += feature;
		}

		generated_manifest += "\"";
	}
	generated_manifest += "]\n";

	generated_manifest += "all-unstable = [\"all";
	for feature in &*all_unstable_refs {
		generated_manifest += "\", \"";
		generated_manifest += feature;
		generated_manifest += "-unstable";
	}
	generated_manifest += "\"]\n";

	generated_manifest += "all-addons = [";
	if let [first, rest @ ..] = &*all_addons_refs {
		generated_manifest += "\"";
		generated_manifest += first;

		for feature in rest {
			generated_manifest += "\", \"";
			generated_manifest += feature;
		}

		generated_manifest += "\"";
	}
	generated_manifest += "]\n\n";

	for Feature {
		name,
		feature_type,
		desc: _,
		dependencies,
		features
	} in &_features {
		match feature_type {
			FeatureType::Stable => {
				generated_manifest += name;
			}
			FeatureType::Unstable => {
				generated_manifest += name;
				generated_manifest += "-unstable";
			}
			FeatureType::Addon => {
				if !switched_to_addons {
					switched_to_addons = true;
					generated_manifest += "\n# addon features\n";
				}
				generated_manifest += name;
			}
		}

		generated_manifest += " = [";

		if !dependencies.is_empty() || !features.is_empty() {
			let mut seen = false;

			macro_rules! maybe_put_comma {
				() => {
					if seen {
						generated_manifest += ",";
					} else {
						seen = true;
					}
				}
			}

			if dependencies.contains(&"macro") {
				maybe_put_comma!();
				generated_manifest += "\n\t\"";
				generated_manifest += "wiwiwiwiwi/";
				generated_manifest += name;
				generated_manifest += "\"";
			}

			for dependency in *dependencies {
				if *dependency == "macro" { continue }

				let dependency = _dependencies.iter_mut()
					.find(|d| d.name == *dependency)
					.unwrap_or_else(|| panic!("feature \"{name}\" wants to depend on dependency \"{dependency}\", which does not exist"));
				dependency.has_dependent = true;

				maybe_put_comma!();

				generated_manifest += "\n\t\"";
				generated_manifest += "dep:";

				if dependency.uses_underscore {
					generated_manifest += &dependency.name.replace('-', "_");
				} else {
					generated_manifest += dependency.name;
				}

				generated_manifest += "\"";
			}

			for feature in *features {
				let feature = _features.iter()
					.find(|f| f.name == *feature)
					.unwrap_or_else(|| panic!("feature \"{name}\" wants to depend on feature \"{feature}\", which does not exist"));

				// TODO: implement the thing where implicitly enabled features don't allow external access to the modules
				// if matches!(feature_type, FeatureType::Stable) && matches!(feature.feature_type, FeatureType::Unstable) {
				// 	panic!("stable feature cannot depend on unstable feature ({name} depending on {})", feature.name);
				// }

				maybe_put_comma!();

				generated_manifest += "\n\t\"";
				generated_manifest += feature.name;

				if matches!(feature.feature_type, FeatureType::Unstable) {
					generated_manifest += "-unstable";
				}

				generated_manifest += "\"";
			}

			generated_manifest += "\n";
		}

		generated_manifest += "]\n";
	}
	generated_manifest += "\n";

	let mut start;
	let mut end;
	let mut current;
	let mut output = String::new();

	macro_rules! manipulate_file {
		{ $filepath:ident, $start_marker:ident, $end_marker:ident in $($stuff:tt)* } => {
			current = fs::read_to_string($filepath)
				.unwrap_or_else(|e| panic!("failed to read {}: {e:?}", $filepath));
			start = current.find($start_marker)
				.unwrap_or_else(|| panic!("failed to find start marker in {}", $filepath));
			start += $start_marker.len();
			end = current.find($end_marker)
				.unwrap_or_else(|| panic!("failed to find end marker in {}", $filepath));
			output.clear();

			output += &current[..start];
			output += "\n\n";
			{ $($stuff)* }
			output += &current[end..];

			fs::write($filepath, &*output)
				.unwrap_or_else(|e| panic!("failed to write back to {}: {e:?}", $filepath))
		}
	}

	manipulate_file! { wiwi_manifest, wiwi_manifest_start_marker, wiwi_manifest_end_marker in
		output += &generated_manifest;
	}

	manipulate_file! { wiwi_readme, wiwi_readme_start_marker, wiwi_readme_end_marker in
		output += &generated_readme_stable;
		output += "\n";
		output += &generated_readme_addons;
		output += "\n";
		output += &generated_readme_unstable;
		output += "\n";
	}

	manipulate_file! { wiwi_lib, wiwi_lib_start_marker, wiwi_lib_end_marker in
		output += &generated_lib;
	}

	manipulate_file! { wiwi_doc_cfgs, wiwi_doc_cfgs_start_marker, wiwi_doc_cfgs_end_marker in
		output += &generated_doc_cfgs_list;
		output += "\n";
		output += &generated_doc_cfgs_no_features_enabled;
		output += "\tnot(docsrs)\n), doc = \"No features enabled! (you should probably enable something, otherwise this crate does nothing :p)\")]\n\n";
	}

	manipulate_file! { wiwi_prelude_cfgs, wiwi_prelude_cfgs_start_marker, wiwi_prelude_cfgs_end_marker in
		// minus the trailing comma and newline
		let substr = generated_doc_cfgs_no_features_enabled.len() - 2;
		output += &generated_doc_cfgs_no_features_enabled[..substr];
		output += "\n), doc = \"(... of which there are none lol)\")]\n\n";
	}
}

#[derive(Debug)]
struct Dependency {
	name: &'static str,
	version: &'static str,
	features: &'static [&'static str],
	uses_underscore: bool,
	has_dependent: bool
}

#[derive(Debug)]
struct Feature {
	name: &'static str,
	feature_type: FeatureType,
	desc: &'static str,
	dependencies: &'static [&'static str],
	features: &'static [&'static str],
}

#[derive(Debug)]
enum FeatureType {
	Stable,
	Unstable,
	Addon
}

macro_rules! decl_dependencies {
	{
		$(
			$(#[$uses_underscore:tt])?
			$name:literal
			$(features: [$($features:literal),*])?
		)*
	} => {
		[
			$(
				{
					const NAME_AND_VERSION: (&str, &str) = split_name_and_version!($name);

					decl_dependencies! {
						@impl
						$(#[$uses_underscore])?
						$name
						features: [$($($features)*)?]
					}
				}
			),*
		]
	};

	{
		@impl
		$name:literal
		features: [$($features:literal),*]
	} => {
		Dependency {
			name: NAME_AND_VERSION.0,
			version: NAME_AND_VERSION.1,
			features: &[$($features)*],
			uses_underscore: false,
			has_dependent: false
		}

	};

	{
		@impl
		#[uses_underscore]
		$name:literal
		features: [$($features:literal),*]
	} => {
		Dependency {
			name: NAME_AND_VERSION.0,
			version: NAME_AND_VERSION.1,
			features: &[$($features)*],
			uses_underscore: true,
			has_dependent: false
		}

	};
}
use decl_dependencies;

macro_rules! decl_features {
	{ $(
		$(#[$type:tt])?
		$name:literal
		$desc:literal
		$(dependencies: [$($dependencies:literal),*])?
		$(features: [$($features:literal),*])?
	)* } => {
		[$(
			decl_features! {
				@impl $($type)?
				name: $name
				desc: $desc
				dependencies: [$($($dependencies)*)?]
				features: [$($($features)*)?]
			}
		),*]
	};

	{
		@impl
		name: $name:literal
		desc: $desc:literal
		dependencies: [$($dependencies:literal)*]
		features: [$($features:literal)*]
	} => {
		Feature {
			name: $name,
			feature_type: FeatureType::Stable,
			desc: $desc,
			dependencies: &[$($dependencies),*],
			features: &[$($features),*]
		}
	};

	{
		@impl unstable
		name: $name:literal
		desc: $desc:literal
		dependencies: [$($dependencies:literal)*]
		features: [$($features:literal)*]
	} => {
		Feature {
			name: $name,
			feature_type: FeatureType::Unstable,
			desc: $desc,
			dependencies: &[$($dependencies),*],
			features: &[$($features),*]
		}
	};

	{
		@impl addon
		name: $name:literal
		desc: $desc:literal
		dependencies: [$($dependencies:literal)*]
		features: [$($features:literal)*]
	} => {
		Feature {
			name: $name,
			feature_type: FeatureType::Addon,
			desc: $desc,
			dependencies: &[$($dependencies),*],
			features: &[$($features),*]
		}
	};
}
use decl_features;

macro_rules! split_name_and_version {
	($s:literal) => {{
		use std::str::from_utf8;

		let bytes = $s.as_bytes();

		// find the first space

		let mut count = 0;
		let mut name = None;
		let mut version = None;

		while count < bytes.len() {
			if bytes[count] == b' ' {
				let (name_bytes, version_bytes) = bytes.split_at(count);
				let (_space, version_bytes) = version_bytes.split_at(1);

				name = Some(const_unwrap!(from_utf8(name_bytes)));
				version = Some(const_unwrap!(from_utf8(version_bytes)));
				break;
			}
			count += 1;
		}

		let (Some(name), Some(version)) = (name, version) else {
			panic!(concat!("expected space seperated name and version string (in input \"", $s, "\")"));
		};

		let mut count = 0;
		let bytes = version.as_bytes();
		while count < bytes.len() {
			if bytes[count] == b' ' {
				panic!(concat!("found unexpected space in version (for version in input \"", $s, "\")"));
			}

			count += 1;
		}

		(name, version)
	}}
}
use split_name_and_version;

macro_rules! const_unwrap {
	($result:expr $(, $msg:literal)?) => {
		match $result {
			Ok(val) => { val }
			Err(_e) => { panic!($($msg)?) }
		}
	}
}
use const_unwrap;
