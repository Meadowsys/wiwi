# wiwi pony bindings

Bindings to `wiwi` for [Pony](https://www.ponylang.io)

## status

I wrote this as a kind of mess around, to see if I could use pony/rust's C FFI interfaces together. Currently, only the feature `z85` is included, and makes available only `Wiwi.z85_encode` and `Wiwi.z85_decode` functions

## how to use

You need to build the bindings lib first. You can do so by `cd`ing into the `bindings` dir, then running `cargo build --release --lib`, to build the static lib (it gets placed in the `target` dir in the root of the repo (workspace))

You then need to tell `ponyc` where to find the bindings package during compilation. You can do that in one of two ways:

- You can include a path to the `bindings` folder in your `PONYPATH` environment variable when invoking `ponyc`, and writing `use "pony"` in your source files
- Alternatively, you can symlink from any name in the root of your pony project, to `bindings/pony` folder, and writing `use "<symlink-name>"` at the top of your pony code

I have not messed with corral much (and am quite early in learning the language itself, for that matter :p), so there are other ways to include this in your project too, in perhaps more standard ways.
