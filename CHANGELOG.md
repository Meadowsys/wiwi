# changelog

Hallo!

## unreleased

- completely redesigned readme
  - seperated stable and unstable features, making it easy to see the stable stuff
- `gen-features` script
  - now, dependencies and features are declared in there, and it'll check them, and then generate/update the content for all the other places that previously needed to be manually updated, including `Cargo.toml`, `README.md`, `lib.rs`, the prelude, and may come in handy for other places too
- feature `cli`
  - created the feature (unstable)
  - it's empty lol
- feature `with-cloned`
  - stabilised the feature
- tuples
  - by default, wiwi will only include code to accomodate tuples with up to 8 elements (where applicable).
  - created features `large-tuples` and `omega-tuples-of-doom`
    - `large-tuples` enables implementations for tuples with up to 32 elements
    - `omega-tuples-of-doom` enables implementations for tuples with up to 128 elements... if for some reason you need this

## v0.8.0

- created `all-addons` feature, that enables all addon features (3rd-party crate integration)
- feature `aoc`:
  - created the feature (unstable)
- feature `bitstream`:
  - added regular (panicking) and checked versions of the write methods
- feature `chainer`:
  - added `vec_chain!` macro, same to `vec!` except it returns a `VecChain`
- feature `int`:
  - renamed from `bigint` (still unstable)
  - it's... effectively useless right now
- feature `iter`:
  - added `Iter::enumerate`
  - added `Iter::enumerate1`, identical to `Iter::enumerate` but it starts at 1, not 0
  - completely redid size hint struct
  - tuple iters will now fuse and never advance any inner iters again after one returns `None`
- feature `lazy-wrap`:
  - added some documentation on the trait impl bounds of `Send`, `Sync`, `UnwindSafe`, `RefUnwindSafe`, and `Unpin`
- feature `with-cloned`:
  - created the feature (unstable)
  - macro that helps with the clone-and-move pattern (hey..! is that a good description/name for it?)
  - ...with this addition, we definitely fit under `rust-patterns` category now

## v0.7.0

- docs for the latest commits are now being published to [wiwi.kiwin.gay](https://wiwi.kiwin.gay)! (`all-unstable` is enabled for it)
- created a proc macro crate, so we can have proc macros now!
- update categories and keywords
  - keywords: changed to `fundamental`, `general`, `general-purpose`, `z85`
  - categories: `rust-patterns`
- update the fancy, over the top crate docs
- removed feature configuration features (`debounce-dyn-fn`)
- prelude:
  - added `h::H`
- feature `bigint`:
  - created the feature (unstable)
- feature `bitstream`:
  - created the feature (unstable)
- feature `chainer`:
  - ... fresh start
    - the old one is still available under `wiwi::chainer`
    - the new one is temporarily available under `wiwi::chainer::new` and will be exported under `wiwi::chainer` once we feel it's ready to replace the existing one
      - we have features depending on chainer too, so gotta keep the old one around for now
    - chain traits `ChainHalf` and `NonChainHalf`
      - they have many trait bounds and require each other, ensuring a "base level" of ability to convert between chainer/non for all chainers
      - all the traits and requirements (and even the struct definition itself) is handled in the macro, so very little boilerplate required
    - chains are now declared via macro, saving considerable boilerplate
    - chained functions are now declared via macro, also saving lots of boilerplate
    - maybe about like, 98% of the old `VecChain` has been reimplemented in the new `VecChain`, no other chainers available just yet
- feature `debounce`:
  - removed `debounce-dyn-fn`, it will now always store the inner function in a dyn box
    - reasoning: a debounced function will inherently not run in a hot loop, since its literally a delaying mechanism, so the overhead of dyn box should be unnoticeable. The heap allocation indirection / dyn vtable indirection is only really noticeable when used in hot loops
- feature `defer`:
  - created the feature (unstable)
- feature `h`:
  - added `H`
  - h
- feature `id`:
  - `GeneratedID::to_alphanumeric_string` and `GeneratedID::from_alphanumeric_string`, encoding to/from a custom string representation that's `0-9A-Za-z` (base 62 I suppose)
  - make `IDGenerator` threadsafe (by not holding onto an instance of `ThreadRng`)
- feature `iter`:
  - updated docs
  - added many tests
  - changed `Iter::size_hint`
    - it now returns a struct containing upper and lower bound, both which may be present or not
    - bounds can either be estimates or hard bounds
    - estimates are estimates, same as std's size_hint in terms of trustworthiness
    - hard bounds are for when you're strictly sure its going to be that way, consumers of the function are allowed to rely on this for safety
      - because of that, hard bounds can only be constructed through `unsafe` interfaces
  - assert that `Iter` is object safe
  - impl `IntoIter` for `Vec` with a custom `Iter` type
  - renamed a few interfaces
    - `AsWiwiIter::as_wiwi_iter` -> `AsWiwiIter::borrow_std_as_wiwi_iter`
    - `AsStdIterator::as_std_iterator` -> `AsStdIterator::borrow_wiwi_as_std_iterator`
    - `IntoWiwiIter::into_wiwi_iter` -> `IntoWiwiIter::convert_std_into_wiwi_iter`
    - `IntoStdIterator::into_std_iterator` -> `IntoStdIterator::convert_wiwi_into_std_iterator`
    - `IntoIter::into_iter` -> `IntoIter::into_wiwi_iter` (unfortunate)
  - added `Iter::for_each`, `Iter::map`
  - `RepeatPerItem<I, T>` -> `RepeatPerItem<T, I>`
  - attempted to add a "peekable here" type of API and trait, but it's not possible without trait specialisation I think
  - tuple impls (`IntoIter` is implemented for tuples with up to 32 elements, where all its elements also implement `IntoIter`)
  - `DoubleEndedIter`
- feature `memory-usage`:
  - created the feature (unstable)
- feature `minesweeper`:
  - created the feature (unstable)
- feature `nominal`:
  - copied from [wiwipaccer](https://github.com/meadowsys/wiwipaccer) (but with _many_ modifications, base idea is the same though)
- feature `serialiser`:
  - at some point we yeeted everything to start over, not sure when, but it's (mostly) yeeted now
- feature `sudoku`:
  - functions to brute force generate valid solutions
    - not feasible at all lol (maybe in 200 years when we have quantum computers or something)
- feature `superstring`:
  - removed the feature

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
  - created the feature
- feature `z85`:
  - updated docs

## v0.5.4 (yanked[^2])

<!-- TODO: review the commits / write more detailed potentially -->

- gate binaries behind appropriate features
  - they were never meant to be properly part of crate API though (they are removed in a future version)
- feature `z85`:
  - fixed link in module doc not actually hyperlinking
  - check for decode overflow[^1]

## v0.5.3 (yanked[^2][^1])

<!-- TODO: review the commits / write more detailed potentially -->

- added MIT license file (however, MIT license was always specified in crate manifest)
- feature `clock-timer-2`:
  - added info about if a tick was delayed (`Tick::delayed` and `Tick::past_due`, both of which do the same thing)
- feature `z85`:
  - tweaked some docs

## v0.5.2 (yanked[^2][^1])

<!-- TODO: review the commits / write more detailed potentially -->

- added keywords `decode`, `encode`, `hex`, `z85` to crate manifest
- feature `hex`:
  - added neon implementation for `aarch64` (automatically enabled on `aarch64` targets using runtime feature detection)
  - fixed uppercase hex unintentionally outputting lowercase

## v0.5.1 (yanked[^2][^1])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - fixed missing `thiserror` dependency

## v0.5.0 (yanked[^2][^1])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - created the feature
  - added footnote about its performance in readme
- feature `z85`:
  - added footnote about its performance in readme

## v0.4.1 (yanked[^2][^1])

<!-- TODO: review the commits / write more detailed potentially -->

- feature `z85`:
  - added test for non-padded compatability with `z85` crate (we now guarantee that for non-padded inputs (ie. input length is a multiple of 4), our output will be the same as `z85` crate)
  - improved performance
  - updated module doc

## v0.4.0 (yanked[^2][^1])

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
  - h
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

[^1]: Depending on the compile setting for overflow checks, certain invalid input could cause either a panic or undefined behaviour (affected features: `z85`, affected versions: `>= 0.4.0, <= 0.5.3`)
[^2]: `Vec::with_capacity` is allowed to overallocate, and in doing so will cause undefined behaviour (affected features: `hex`, `z85`, affected versions: `>= 0.4.0, <= 0.6.0`)
