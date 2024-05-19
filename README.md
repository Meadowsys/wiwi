# wiwi

A lil lib containing misc utilities, and Stuff™. Contains some useful things, contains some silly things.

All exposed features are gated behind features, none of which are enabled by default.

## Features

<!-- make sure to check Cargo.toml and workflow files too -->

In addition to the features listed below, there exists a feature `all` that will enable all (stable) features. The feature `all-unstable` will enable all stable and unstable features. Addon features are not included in any of these two features (for now, maybe).

- **`auth`** (unstable) - Some lower(ish) level utilities to aid in writing an authentication system, in which the client password is never sent across the wire. Quite heavily inspired by [Tuta's authentication/encryption system](https://tuta.com/nl/encryption).
- **`chainer`** (unstable) - zero-cost wrappers that provide chaining APIs
- **`clock-timer`** - An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration.
- **`debounce`** - Delay calling a function until a certain time period has passed since the last time it was called.
- **`h`** - h
- **`hex`** - Fast (faster than `hex` crate[^1]) implementation of hex encoding, supporting upper hex and lower hex.
- **`id`** (unstable) - ID generator, with all IDs generated from one generater guaranteed to be monotonically increasing
- **`iter`** (unstable) - iter stuff
- **`lazy-wrap`** - Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)
- **`lsl`** (unstable) - experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol >< It is really only built for a dedicated crate just to write the script, rather than as part of another lib/app.
- **`path`** (unstable) - UTF-8 only path manipulation utilities written from scratch
- **`serialiser`** (unstable) - self describing, stable (once finished) binary serialiser, aiming for small output size by exploiting common patterns in real world data
- **`string-pool`** (unstable) - Global immutable string pool and String type
- **`sudoku`** (unstable) - Sudoku related... stuff
- **`superstring`** (unstable) - Implementation of [superstring](https://github.com/pulsar-edit/superstring)
- **`to-maybeuninit`** - Extension trait allowing converting from references to `MaybeUninit` references
- **`z85`** - A fast (faster than `z85` crate[^2]) implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct length where the spec says the application code must handle it instead.

### Unstable features

**Unstable features are NOT covered by semver.** These features may change in breaking ways in non-breaking version bumps, and might even be incomplete. I'm doing this so I don't have to mess with commenting out features and stuff whenever I have something else that I'm publishing but still have unfinished work somewhere else >.>

Unstable features have `-unstable` appended to their name. For example, if I have feature `thingie`, while it is unstable the feature is named `thingie-unstable`. The `-unstable` suffix will be removed once the feature is stabilised.

Because unstable features may break whenever, if you use them, you may want to pin the version of `wiwi` to an exact version (ie. use the `=` semver operator).

**NOTE:** Just because a feature is _not_ "unstable", doesn't mean it won't break, _ever_. It can break, but of course that will be accompanied by deprecation ahead of time and a major version bump, so no surprises there. Just don't be surprised when bumping a patch version causes an unstable feature to break everything. :p

### Addon features

These features enable things specific to 3rd-party crates and features you have enabled. For example, the `hashbrown` feature, when the `serialiser` feature is enabled, will implement `Serialise` and `Deserialise` traits for structs in `hashbrown`.

If you enable one of these, and it doesn't have anything to affect any of the features you have enabled, it'll silently do nothing.

- **`hashbrown`**
- **`image`**
- **`serde-json`**

### Feature configuration features

These don't change the exposed API or program behaviour at all, only some internal implementation details that might affect things such as compile times, performance, and binary size.

- **`debounce-dyn-fn`** - Wraps functions into a `Box<dyn Fn>`, rather than monomorphising. This will cause calls to the underlying function (not the returned one) to be a bit slower than static dispatch, but also reduces the binary size depending on how many different concrete types the debounce functions are called with.

## Platform support

This package will only _officially_ support macOS and Linux. Windows support will only be on best effort basis. Still do submit issues, I just can't guarantee I can fix them etc (but you can PR me, if you'd like!).

[zeromq]: https://zeromq.org
[z85]: https://rfc.zeromq.org/spec/32

[^1]: Based on the benchmark available in this repo: wiwi is about 21.5x faster in encode, and 7.5x faster in decode. I want better benchmarks though. For now the `hex` crate also provides more flexibility, whereas `wiwi::hex` just exposes `encode_hex`, `encode_upper_hex`, and `decode_hex` functions.
[^2]: Based on the benchmark available in this repo: wiwi is about 1.4x faster in encode, and 2.2x faster in decode. I want better benchmarks though. There is no functionality that the `z85` crate provides, that we don't also provide (`encode_z85` and `decode_z85` functions).
