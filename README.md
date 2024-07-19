# wiwi

A (not so) lil lib containing misc utilities, and Stuff™. Contains some useful things, contains some silly things. Contains many things!

Everything wiwi can do is gated behind feature flags, none of which are enabled by default.

## Features

After these descriptions of the different types of features, there will be a subsection dedicated to listing each feature.

Stable features are what you might expect, they are considered done and follow semver. Of course this does not mean there can't be functionality added, or that it can't break (if it does, there will be breaking semver bump, adhering to semver).

Unstable features are features that can be any degree of finished, can break whenever, or disappear whenever. They **_do not_** adhere to semver. Before becoming stable, unstable features have `-unstable` appended to their name, which will be removed when the feature stabilises. If you are using unstable features, you may find it wise to pin the version to an exact version (ie. semver `=` operator, ex. `wiwi = "=1.0.0"` in `Cargo.toml`).

Addon features enable additional things in some features, including integrations between `wiwi` and 3rd-party crates. They will enable things in features where applicable, and will silently do nothing if there's nothing to affect.

There exist features `all` to enable all stable features, `all-unstable` to enable all unstable (in addition to stable) features, and `all-addons` to enable all addon features (but you very likely do not want this! Unless you're truly using all the integrations, this can and will bloat your dependency tree).

<!-- ----- start autogenerated region (see gen-features script) ----- -->

### Stable features

- **`augment-panic-hook`** - allows you to augment the current panic hook in a convenient way, running some code on panic, but still calling the existing hook afterwards
- **`clock-timer`** - An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration
- **`debounce`** - Delay calling a function until a certain time period has passed since the last time it was called
- **`export-all-submodules`** - convenience macro for declaring many private modules, then reexporting everything within them using a glob use statement
- **`h`** - h
- **`hex`** - Fast (faster than `hex` crate[^1]) implementation of hex encoding, supporting upper hex and lower hex
- **`lazy-wrap`** - Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)
- **`nominal`** - zero cost wrapper to put data in a newtype, taking advantage of nominal typing for increased safety
- **`rand`** - RNG lib, building on top of `rand`
- **`with-cloned`** - easily execute code using clones of variables in a temporary scope (see the documentation on `with_cloned!`, I'm not sure how to best summarise ><)
- **`z85`** - A fast (faster than `z85` crate[^2]) implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct length where the spec says the application code must handle it instead

### Addon features

- **`hashbrown`** - adds integration with `hashbrown` crate
- **`image`** - adds integration with `image` crate
- **`large-tuples`** - by default, tuple implementations (where applicable of course) are available for tuples with up to 8 elements, which should be enough for most uses. Enabling this feature will enable implementations for tuples with up to 32 elements.
- **`nightly`** - enable features only available in nightly rust
- **`omega-tuples-of-doom`** - _Surely_, no one uses tuples with more than 32 elements in them... but we don't know everyone's use case, so this feature will enable implementations for tuples with up to 128 elements. _Hopefully_, that is enough for everything. :p
- **`serde`** - adds integration with `serde` crate
- **`serde-json`** - adds integration with `serde-json` crate

### Unstable features

reminder: **Unstable features are NOT covered by semver!**

- **`aoc`** - Utilities specific for writing solutions for [Advent of Code](https://adventofcode.com)
- **`auth`** - Some lower(ish) level utilities to aid in writing an authentication system, in which the client password is never sent across the wire. Quite heavily inspired by [Tuta's authentication/encryption system](https://tuta.com/nl/encryption)
- **`bitstream`** - bit stream encoder/decoder
- **`chainer`** - zero-cost wrappers that provide chaining APIs
- **`cli`** - command line args parser
- **`defer`** - utilities for deferring running code
- **`id`** - ID generator, with all IDs generated from one generater guaranteed to be monotonically increasing
- **`int`** - bigint / uneven int types
- **`iter`** - iter stuff
- **`lsl`** - experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol >< It is really only built for a dedicated crate just to write the script, rather than as part of another lib/app
- **`mcu`** - [material colour utilities](https://github.com/material-foundation/material-color-utilities), ported to rust
- **`memory-usage`** - Calculate actual memory usage of Rust structs, including derive macro for custom types
- **`minesweeper`** - core logic components for minesweeper games of arbitrary size
- **`num-traits`** - traits for number types and number functionality
- **`path`** - UTF-8 only path manipulation utilities written from scratch
- **`serialiser-binary`** - self describing, stable (once finished) binary serialiser, aiming for small output size by exploiting common patterns in real world data
- **`serialiser-text`** - self describing, stable (once finished) text serialiser, aiming for human readability, and ease of writing
- **`string-pool`** - Global immutable string pool and String type
- **`sudoku`** - Sudoku related... stuff
- **`unicode`** - implementation of the [Unicode](https://home.unicode.org) standard, including UTF-8, UTF-16, and UTF-32 strings

<!-- ----- end autogenerated region ----- -->

## Platform support

This package will only _officially_ support macOS and Linux. Windows support will only be on best effort basis. This does not mean I don't want to support Windows though! Just that my ability to do so is going to be lesser than unix platforms. Still do submit issues though, and you can PR me if you'd like!

[zeromq]: https://zeromq.org
[z85]: https://rfc.zeromq.org/spec/32

[^1]: Based on the benchmark available in this repo: wiwi is about 21.5x faster in encode, and 7.5x faster in decode. I want better benchmarks though. For now the `hex` crate also provides more flexibility, whereas `wiwi::hex` just exposes `encode_hex`, `encode_upper_hex`, and `decode_hex` functions.
[^2]: Based on the benchmark available in this repo: wiwi is about 1.4x faster in encode, and 2.2x faster in decode. I want better benchmarks though. There is no functionality that the `z85` crate provides, that we don't also provide (`encode_z85` and `decode_z85` functions).
