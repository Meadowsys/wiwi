# wiwi

A lil lib containing misc utilities, and Stuff™. Contains some useful things, contains some silly things. Contains many things!

Everything wiwi can do is gated behind feature flags, none of which are enabled by default.

## Features

<!-- make sure to check Cargo.toml and workflow files too -->

In addition to the features listed below, there exists a feature `all` that will enable all (stable) features. The feature `all-unstable` will enable all stable and unstable features. Addon features are not included in any of these two features (for now, maybe).

- **`aoc`** (unstable) - Utilities specific for writing solutions for [Advent of Code](https://adventofcode.com)
- **`auth`** (unstable) - Some lower(ish) level utilities to aid in writing an authentication system, in which the client password is never sent across the wire. Quite heavily inspired by [Tuta's authentication/encryption system](https://tuta.com/nl/encryption).
- **`bitstream`** (unstable) - bit stream encoder/decoder
- **`chainer`** (unstable) - zero-cost wrappers that provide chaining APIs
- **`clock-timer`** - An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration.
- **`debounce`** - Delay calling a function until a certain time period has passed since the last time it was called.
- **`defer`** (unstable) - utilities for deferring running code
- **`h`** - h
- **`hex`** - Fast (faster than `hex` crate[^1]) implementation of hex encoding, supporting upper hex and lower hex.
- **`id`** (unstable) - ID generator, with all IDs generated from one generater guaranteed to be monotonically increasing
- **`int`** (unstable) - bigint / uneven int types
- **`iter`** (unstable) - iter stuff
- **`lazy-wrap`** - Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)
- **`lsl`** (unstable) - experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol >< It is really only built for a dedicated crate just to write the script, rather than as part of another lib/app.
- **`memory-usage`** (unstable) - Calculate actual memory usage of Rust structs, including derive macro for custom types
- **`minesweeper`** (unstable) - core logic components for minesweeper games of arbitrary size
- **`nominal`** - zero cost wrapper to put data in a newtype, taking advantage of nominal typing for increased safety
- **`path`** (unstable) - UTF-8 only path manipulation utilities written from scratch
- **`serialiser`** (unstable) - self describing, stable (once finished) binary serialiser, aiming for small output size by exploiting common patterns in real world data
- **`string-pool`** (unstable) - Global immutable string pool and String type
- **`sudoku`** (unstable) - Sudoku related... stuff
- **`to-maybeuninit`** - Extension trait allowing converting from references to `MaybeUninit` references
- **`z85`** - A fast (faster than `z85` crate[^2]) implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct length where the spec says the application code must handle it instead.

### Addon features

These features enable integrations between `wiwi` and 3rd-party crates for the features you have enabled, where available. If no integrations are available for the features of wiwi that are enabled, addon features will silently do nothing.

There is a feature `all-addons` that will enable every single addon feature. This can really bloat your dependency tree, make sure you know what you want to do!

- **`hashbrown`**
- **`image`**
- **`serde-json`**

## Platform support

This package will only _officially_ support macOS and Linux. Windows support will only be on best effort basis. Still do submit issues, I just can't guarantee I can fix them etc (but you can PR me, if you'd like!).

### Unstable features

**Unstable features are NOT covered by semver.** These features may change in breaking ways in non-breaking version bumps, and might even be incomplete. I'm doing this so I don't have to mess with commenting out features and stuff whenever I have something else that I'm publishing but still have unfinished work somewhere else >.>

Unstable features have `-unstable` appended to their name. For example, if I have feature `thingie`, while it is unstable the feature is named `thingie-unstable`. The `-unstable` suffix will be removed once the feature is stabilised.

Because unstable features may break whenever, if you use them, you may want to pin the version of `wiwi` to an exact version (ie. use the `=` semver operator).

**NOTE:** Just because a feature is _not_ "unstable", doesn't mean it won't break, _ever_. It can break, but of course those breakages will adhere to semver rules, as expected. Just don't be surprised when bumping a patch version causes an unstable feature to break everything. :p

[zeromq]: https://zeromq.org
[z85]: https://rfc.zeromq.org/spec/32

[^1]: Based on the benchmark available in this repo: wiwi is about 21.5x faster in encode, and 7.5x faster in decode. I want better benchmarks though. For now the `hex` crate also provides more flexibility, whereas `wiwi::hex` just exposes `encode_hex`, `encode_upper_hex`, and `decode_hex` functions.
[^2]: Based on the benchmark available in this repo: wiwi is about 1.4x faster in encode, and 2.2x faster in decode. I want better benchmarks though. There is no functionality that the `z85` crate provides, that we don't also provide (`encode_z85` and `decode_z85` functions).
