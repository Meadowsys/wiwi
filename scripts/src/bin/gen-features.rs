fn main() {
	let wiwi_manifest = "Cargo.toml";
	let macro_manifest = "macro/Cargo.toml";

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

		#[unstable]
		"auth"
		"Some lower(ish) level utilities to aid in writing an authentication system, in which the client password is never sent across the wire. Quite heavily inspired by [Tuta's authentication/encryption system](https://tuta.com/nl/encryption)"
		dependencies: ["argon2", "chacha20poly1305", "chrono", "p384", "rand", "thiserror", "zeroize"]
		features: ["z85"]

		#[unstable]
		"bitstream"
		"bit stream encoder/decoder"

		#[unstable]
		"chainer"
		"zero-cost wrappers that provide chaining APIs"
		features: ["iter", "to-maybeuninit"]

		"clock-timer"
		"An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration"
		dependencies: ["chrono", "tokio"]

		"debounce"
		"Delay calling a function until a certain time period has passed since the last time it was called"
		dependencies: ["chrono", "tokio"]

		#[unstable]
		"defer"
		"utilities for deferring running code"

		"h"
		"h"

		"hex"
		"Fast (faster than `hex` crate[^1]) implementation of hex encoding, supporting upper hex and lower hex"
		dependencies: ["thiserror"]

		#[unstable]
		"id"
		"ID generator, with all IDs generated from one generater guaranteed to be monotonically increasing"
		dependencies: ["rand"]

		#[unstable]
		"int"
		"bigint / uneven int types"
		dependencies: ["macro"]

		#[unstable]
		"iter"
		"iter stuff"

		"lazy-wrap"
		"Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)"
		dependencies: ["parking-lot"]

		#[unstable]
		"lsl"
		"experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol >< It is really only built for a dedicated crate just to write the script, rather than as part of another lib/app"
		dependencies: ["hashbrown", "spin-sleep", "uuid"]
		features: ["hex", "id"]

		#[unstable]
		"memory-usage"
		"Calculate actual memory usage of Rust structs, including derive macro for custom types"
		dependencies: ["macro"]

		#[unstable]
		"minesweeper"
		"core logic components for minesweeper games of arbitrary size"
		dependencies: ["rand"]
		features: ["chainer", "z85"]

		"nominal"
		"zero cost wrapper to put data in a newtype, taking advantage of nominal typing for increased safety"

		#[unstable]
		"path"
		"UTF-8 only path manipulation utilities written from scratch"

		#[unstable]
		"serialiser"
		"self describing, stable (once finished) binary serialiser, aiming for small output size by exploiting common patterns in real world data"
		dependencies: ["ordered-float"]

		#[unstable]
		"string-pool"
		"Global immutable string pool and String type"
		dependencies: ["hashbrown"]
		features: ["lazy-wrap"]

		#[unstable]
		"sudoku"
		"Sudoku related... stuff"
		features: ["chainer", "iter"]

		"to-maybeuninit"
		"Extension trait allowing converting from references to `MaybeUninit` references"

		#[unstable]
		"with-cloned"
		"execute code using clones of variables in a temporary scope (see the documentation on `with_cloned!`, I'm not sure how to best summarise ><)"

		"z85"
		"A fast (faster than `z85` crate[^2]) implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct length where the spec says the application code must handle it instead"
		dependencies: ["thiserror"]

		#[addon]
		"hashbrown"
		"adds integration with `hashbrown` crate"
		dependencies: ["hashbrown"]

		#[addon]
		"image"
		"adds integration with `image` crate"
		dependencies: ["image"]

		#[addon]
		"large-tuples"
		"by default, tuple implementations (where applicable of course) are available for tuples with up to 8 elements, which should be enough for most uses. Enabling this feature will enable implementations for tuples with up to 32 elements."

		#[addon]
		"omega-tuples-of-doom"
		"_Surely_, no one uses tuples with more than 32 elements in them... but we don't know everyone's use case, so this feature will enable implementations for tuples with up to 128 elements. _Hopefully_, that is enough for everything. :p"
		features: ["large-tuples"]

		#[addon]
		"serde-json"
		"adds integration with `serde-json` crate"
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

	let mut generated_deps = String::new();
	generated_deps += concat!("[dependencies]\nwiwiwiwiwi = { path = \"macro\", version = \"=", std::env!("CARGO_PKG_VERSION"), "\" }\n\n");

	for Dependency {
		name,
		version,
		features,
		uses_underscore,
		has_dependent: _
	} in &_dependencies {
		if *uses_underscore {
			generated_deps += &name.replace('-', "_");
		} else {
			generated_deps += name;
		}

		generated_deps += " = { version = \"";
		generated_deps += version;
		generated_deps += "\", optional = true";

		if let [first, rest @ ..] = features {
			generated_deps += ", features = [\"";
			generated_deps += first;

			for feature in rest {
				generated_deps += "\", \"";
				generated_deps += feature;
			}

			generated_deps += "\"]";
		}

		generated_deps += " }\n";
	}
	println!("{generated_deps}");

	let mut generated_features = String::new();
	let mut switched_to_addons = false;

	generated_features += "[features]\n";

	for Feature {
		name,
		feature_type,
		desc,
		dependencies,
		features
	} in &_features {
		match feature_type {
			FeatureType::Stable => {
				generated_features += name;
			}
			FeatureType::Unstable => {
				generated_features += name;
				generated_features += "-unstable";
			}
			FeatureType::Addon => {
				if !switched_to_addons {
					switched_to_addons = true;
					generated_features += "\n# addon features\n";
				}
				generated_features += name;
			}
		}

		generated_features += " = [";

		if !dependencies.is_empty() || !features.is_empty() {
			let mut seen = false;

			macro_rules! maybe_put_comma {
				() => {
					if seen {
						generated_features += ",";
					} else {
						seen = true;
					}
				}
			}

			if dependencies.contains(&"macro") {
				maybe_put_comma!();
				generated_features += "\n\t\"";
				generated_features += "wiwiwiwiwi/";
				generated_features += name;
				generated_features += "\"";
			}

			for dependency in *dependencies {
				if *dependency == "macro" { continue }

				let dependency = _dependencies.iter()
					.find(|d| d.name == *dependency)
					.unwrap_or_else(|| panic!("feature \"{name}\" wants to depend on dependency \"{dependency}\", which does not exist"));

				maybe_put_comma!();

				generated_features += "\n\t\"";
				generated_features += "dep:";

				if dependency.uses_underscore {
					generated_features += &dependency.name.replace('-', "_");
				} else {
					generated_features += dependency.name;
				}

				generated_features += "\"";
			}

			for feature in *features {
				let feature = _features.iter()
					.find(|f| f.name == *feature)
					.unwrap_or_else(|| panic!("feature \"{name}\" wants to depend on feature\"{feature}\", which does not exist"));

				maybe_put_comma!();

				generated_features += "\n\t\"";
				generated_features += feature.name;

				if matches!(feature.feature_type, FeatureType::Unstable) {
					generated_features += "-unstable";
				}

				generated_features += "\"";
			}

			generated_features += "\n";
		}

		generated_features += "]\n";
	}

	println!("{generated_features}");
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

macro_rules! check_strs_eq {
	($s1:literal, $s2:literal) => {{
		let s1 = $s1;
		let s2 = $s2;

		if s1.len() != s2.len() {
			check_strs_eq!(@panic $s1 $s2);
		}

		let s1 = s1.as_bytes();
		let s2 = s2.as_bytes();
		let mut i = s1.len();

		while i < s1.len() {
			if s1[i] != s2[i] {
				check_strs_eq!(@panic $s1 $s2);
			}

			i += 1;
		}
	}};

	(@panic $s1:literal $s2:literal) => {
		panic!(concat!("strs are not equal: \"", $s1, "\" and ", $s2))
	};
}
use check_strs_eq;
