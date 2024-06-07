# changelog

Hallo!

## unreleased

## v0.6.1

<!-- TODO: review the commits / write more detailed potentially -->

- store requested size and use that to set the len, instead of relying on capacity[^2]
- updated that overkill crate doc stuff
- feature `sudoku`:
  - fixed missing `chainer` and `iter` feature dependency
- feature `z85`:
  - updated docs

## v0.6.0 (yanked[^2])

<!-- TODO: review the commits / write more detailed potentially -->

- officially support macOS and Linux (Windows support will only be on best effort basis... not to imply I don't want to support it or anything)
- fancy, over the top, crate docs
  - locally built crate docs will list the features that are enabled
  - docs.rs prints a notice saying that the `all` feature is enabled (ie. only stable features), and to build locally if you want docs for unstable features
  - a message along the lines of "no features enabled! you should probably enable some" will show up if you have no features enabled (also something similar for the prelude too)
- use `Result::expect` instead of `Result::unwrap` for better panic messages
- move binaries out of the crate and into a (non-published) scripts crate
- created addon features system
- removed multiple runtime feature system (we only support tokio now, nor was it additive features)
- created unstable feature system
  - features that are not completed/stabilised yet. These are **_not_** covered by the semver guarantee that the rest of the crate is.
  - for features that are unstable, they have their name appended with `-unstable` (eg. feature `thingie` would be `thingie-unstable` while unstable)
  - got tired of commenting out incomplete features just to be able to publish lol
- created feature `all` (enables all stable features, doesn't enable any addon features)
- created feature `all-unstable` (enables all stable and unstable features, doesn't enable any addon features)
- addon features:
  - added `hashbrown`
  - added `image`
  - added `serde-json`
- feature `auth`:
  - created the feature (unstable)
- feature `chainer`:
  - created the feature (unstable)
- feature `clock-timer`:
  - updated docs
- feature `hex`:
  - disabled neon implementation (it doesn't seem to make a difference, LLVM auto vectorisation go brr?)
  - updated docs
- feature `id`:
  - created the feature (unstable)
- feature `iter`:
  - created the feature (unstable)
- feature `lsl`:
  - created the feature (unstable)
- feature `path`:
  - created the feature (unstable)
- feature `serialiser`:
  - created the feature (unstable)
- feature `string-pool`:
  - demoted to unstable lol <!-- swimming pool -->
  - updated docs
- feature `sudoku`:
  - created the feature (unstable)
- feature `superstring`:
  - created the feature (unstable)
- feature `to-maybeuninit`:
  - created the feature (unstable)
- feature `z85`:
  - updated docs

## v0.5.4 (yanked[^2])

<!-- TODO: review the commits / write more detailed potentially -->

- gate binaries behind appropriate features
  - they were never meant to be properly part of crate API though (they are removed in a future version)
- feature `z85`:
  - fixed link in module doc not actually hyperlinking
  - check for decode overflow[^1]

## v0.5.3 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- added MIT license file (however, MIT license was always specified in crate manifest)
- feature `clock-timer-2`:
  - added info about if a tick was delayed (`Tick::delayed` and `Tick::past_due`, both of which do the same thing)
- feature `z85`:
  - tweaked some docs

## v0.5.2 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- added keywords `decode`, `encode`, `hex`, `z85` to crate manifest
- feature `hex`:
  - added neon implementation for `aarch64` (automatically enabled on `aarch64` targets using runtime feature detection)
  - fixed uppercase hex unintentionally outputting lowercase

## v0.5.1 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - fixed missing `thiserror` dependency

## v0.5.0 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - created the feature
  - added footnote about its performance in readme
- feature `z85`:
  - added footnote about its performance in readme

## v0.4.1 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `z85`:
  - added test for non-padded compatability with `z85` crate (we now guarantee that for non-padded inputs (ie. input length is a multiple of 4), our output will be the same as `z85` crate)
  - improved performance
  - updated module doc

## v0.4.0 (yanked[^1][^2])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `z85`:
  - created the feature

## v0.3.1

<!-- TODO: review the commits / write more detailed potentially -->

- feature `clock-timer-2`:
  - added methods on `Tick` to get start/end time, elapsed/remaining time, etc

## v0.3.0

<!-- TODO: review the commits / write more detailed potentially -->

- created readme! (finally)
- added crate doc (it's just including readme in it :p)
- added configuration features (they can only be enabled if the feature they configure is enabled too, otherwise it will compile error)
- feature `clock-timer`:
  - added module doc
  - deprecated (use feature `clock-timer-2` instead)
  - effectively removed, as the `clock-timer` feature is removed (replaced by `clock-timer-2`)
- feature `debounce`:
  - added module doc
- feature `h`:
  - added module doc
- feature `lazy-wrap`:
  - added module doc
- feature `clock-timer-2`:
  - created the feature
  - the (eventual) replacement of `clock-tower` feature
  - requires `tokio` runtime feature to work
- feature `debounce`:
  - added configuration `debounce-dyn-function`, controling whether to wrap the actual functions in `Box<dyn Fn>` instead of monomorphising
  - switched to use `impl Fn` rather than generics

## v0.2.0

<!-- TODO: review the commits / write more detailed potentially -->

- removed the default features that were accidentally committed (whoopsie, that's stuck on crates.io forever lol)
- added `prelude` module
- feature `debounce`:
  - created the feature

## v0.1.2

<!-- TODO: review the commits / write more detailed potentially -->

- feature `clock-timer`:
  - re-exported `chrono::Timelike` (it's needed for the API)

## v0.1.1

<!-- TODO: review the commits / write more detailed potentially -->

- added configuration for docs.rs so its not just empty crate lol

## v0.1.0

<!-- TODO: review the commits / write more detailed potentially -->

- created features system
- created runtime feature selection system (compmile errors if none are selected, and currently only supports `tokio`)
- feature `lazy-wrap`:
  - created the feature
  - copied from our crate `lazy-wrap` v0.4.1
  - added `Debug` and `Display` impls
  - added documentation
- feature `string-pool`:
  - created the feature
  - copied from our crate `string-pool` (either v0.2.1 or `wiwi` branch on git, cannot remember)
  - `Pool` trait now requires `Debug` and `Default`, and `String` now always implements `Debug` and `Default`
- feature `h`:
  - created the feature
  - h
- feature `clock-timer`:
  - created the feature
  - requires `tokio` runtime feature to work

## v0.0.0

- yoink! `wiwi` on crates.io is now mine muahaha
- initialised the package and stuffs

[^1]: Depending on the compile setting for overflow checks, certain invalid input could cause either a panic or undefine behaviour (affected features: `z85`, affected versions: `>= 0.4.0, <= 0.5.3`)
[^2]: `Vec::with_capacity` is allowed to overallocate, and in doing so will cause undefined behaviour (affected features: `hex`, `z85`, affected versions: `>= 0.4.0, <= 0.6.0`)
