# wiwi

A lil lib containing misc utilities, and Stuff™. Contains some useful things, contains some silly things.

All exposed features are gated behind features, none of which are enabled by default.

**Unstable features are NOT covered by semver.** These features may be incomplete, may change in breaking versions in non-breaking semver bumps, and might even be incomplete. I'm doing this so I don't have to mess with commenting out features and stuff whenever I have something else that I'm publishing >.> You must also enable the `unstable` feature to make use of them.

## Features

<!-- make sure to check Cargo.toml and workflow files too -->

- **`auth`** (unstable) - Some lower(ish) level utilities to aid in writing an authentication system, in which the client password is never sent across the wire. Quite heavily inspired by [Tuta's authentication/encryption system](https://tuta.com/nl/encryption).
- **`chaining`** (unstable) - zero-cost wrappers that provide chaining APIs for structs
- **`clock-timer`** - An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration.
- **`debounce`** - Delay calling a function until a certain time period has passed since the last time it was called.
- **`h`** - h
- **`hex`** - Fast (faster than `hex` crate[^1]) implementation of hex encoding, supporting upper hex and lower hex.
- **`id`** (unstable) - ID generator, with all IDs generated from one generater guaranteed to be monotonically increasing
- **`lazy-wrap`** - Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)
- **`lsl`** (unstable) - experimental lib to help with writing Second Life scripts in Rust... because yes, I got fed up with it very quickly and immediately missed Rust lol >< It is really only built for a dedicated crate just to write the script, rather than as part of another lib/app.
- **`serialiser`** (unstable) - self describing, stable (once finished) binary serialiser, aiming for small output size by exploiting common patterns in real world data
- **`string-pool`** - Global immutable string pool and String type
- **`z85`** - A fast (faster than `z85` crate[^2]) implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct length where the spec says the application code must handle it instead.

### Addon features

These features enable things specific to 3rd-party crates and features you have enabled. For example, the `hashbrown` feature, when the `serialiser` feature is enabled, will implement `Serialise` and `Deserialise` traits for structs in `hashbrown`.

- **`hashbrown`**
- **`image`**
- **`serde-json`**

### Feature configuration features

These don't change the exposed API or program behaviour at all, only some internal implementation details that might affect things such as compile times, performance, and binary size.

- **`debounce-dyn-fn`** - Wraps functions into a `Box<dyn Fn>`, rather than monomorphising. This will cause calls to the underlying function (not the returned one) to be a bit slower than static dispatch, but also reduces the binary size depending on how many different concrete types the debounce functions are called with.

[zeromq]: https://zeromq.org
[z85]: https://rfc.zeromq.org/spec/32

[^1]: Based on the benchmark available in this repo: wiwi is about 21.5x faster in encode, and 7.5x faster in decode. I want better benchmarks though. For now the `hex` crate also provides more flexibility, whereas `wiwi::hex` just exposes `encode_hex`, `encode_upper_hex`, and `decode_hex` functions.
[^2]: Based on the benchmark available in this repo: wiwi is about 1.4x faster in encode, and 2.2x faster in decode. I want better benchmarks though. There is no functionality that the `z85` crate provides, that we don't also provide (`encode_z85` and `decode_z85` functions).
