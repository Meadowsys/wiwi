# changelog <!-- omit from toc -->

Hallo!

## table of contents <!-- omit from toc -->

- [v0.10.0](#v0100)
- [v0.9.0](#v090)
- [v0.8.0](#v080)
- [v0.7.0](#v070)
- [v0.6.1](#v061)
- [v0.6.0](#v060)
- [v0.5.4](#v054)
- [v0.5.3](#v053)
- [v0.5.2](#v052)
- [v0.5.1](#v051)
- [v0.5.0](#v050)
- [v0.4.1](#v041)
- [v0.4.0](#v040)
- [v0.3.1](#v031)
- [v0.3.0](#v030)
- [v0.2.0](#v020)
- [v0.1.2](#v012)
- [v0.1.1](#v011)
- [v0.1.0](#v010)
- [v0.0.0](#v000)
- [Yanked versions and reasons](#yanked-versions-and-reasons)

## v0.10.0

I need to stop making such large changes before releasing anything lol

- enabled some safety related lints (in warning for now, will turn to deny when all fixed), which will improve safety documentation and safety overall
- removed some `allow` lints that probably were long overdue removing
- added a CI workflow that runs clippy
  - which is why CI is mostly red right now :p cause I made it error if there's a warning, and I have yet to clear all the warnings I've enabled
- all modules now have an associated `README.md` file in its directory, included in rustdoc too
- created `nightly` addon features, to enable features only available in nightly rust
  - As of now, there's nothing taking advantage of it yet
- don't show list of enabled features on docs.rs/wiwi.kiwin.gay, it feels pretty redundent
- created some bindings for [pony](https://www.ponylang.io)!
  - currently it only provides `z85_encode` and `z85_decode` functions
  - I am not sure if I want to continue this, or maybe eventually provide bindings for other languages too?
- feature `augment-panic-hook`
  - created the feature
  - allows you to augment the current panic hook in a convenient way, running some code on panic, but still calling the existing hook afterwards
- feature `auth`
  - yeet most contents and rewrite some of it, because o.o
  - rewrote P-384 keypair struct
- feature `chainer`
  - create chainers `ArrayChain`, `SliceMutChain`, `SliceRefChain`, with some method impls
  - implement more methods on `VecChain`
  - implement many standard traits, like `Clone`, `Default`, etc
    - This is done through `chainer!` macro, so all chainer structs will automatically have it where applicable
    - they are implemented by the chain type if the nonchain type implements it as well, and for the most part just delegate the impl into the inner struct (one exception is `Debug`, where the chainer struct makes itself known, but still forwards to inner)
  - some more documentation
  - callbacks provide chainer structs as much as possible
- feature `defer`
  - rewrote it :o
  - added the ability to choose, at runtime, whether to run the drop code or not (using both a boolean value and a fn/closure)
    - before, the conditional was statically encoded in the type of `Defer`
    - this type of runtime choosing was possible previously by storing state in the value of the `Defer` itself, and using conditionals in the attached closure itself, but now it's got its own seperated thing, which is cleaner and more obvious that this is possible with
    - `DeferRuntime` allows setting a boolean, and `DeferRuntimeFn` allows determining it with another closure (and another value)
    - construct these with `DeferRuntime::new` and `DeferRuntimeFn::new`
    - access and modify these runtime when values when `DeferRuntime::should_run`, `DeferRuntime::set_should_run`, `DeferRuntimeFn::should_run_value where Twhen: Copy`, `DeferRuntimeFn::should_run_value_ref`, `DeferRuntimeFn::should_run_value_mut`, `DeferRuntimeFn::set_should_run_value`, and `DeferRuntimeFn::swap_should_run_value`
  - Convert between the different static types with functions `Defer::into_always`, `Defer::into_on_success`, `Defer::into_on_unwind`, `Defer::into_runtime`, `Defer::into_runtime_fn`
    - These consume and return a new static type
    - if you want just regular boolean setting, you're probably looking for `DeferRuntime`
  - replaced some functions with associated functions instead
    - `defer_with` -> `DeferAlways::new`
    - `defer_on_success_with` -> `DeferSuccess::new`
    - `defer_on_unwind_with` -> `DeferUnwind::new`
  - removed `OnDrop::defer`, `OnDrop::defer_success`, and `OnDrop::defer_unwind`
    - they were just different names for `OnDrop::on_drop`, `OnDrop::on_success_drop`, and `OnDrop::on_unwind_drop`, respectively
  - added `into_inner` for `Defer`, to get the value back out
    - makes this viable now for the "drop guard" pattern
- feature `id`
  - uses `rand` feature's `ChaCha8` generator instead of thread rng from `rand` crate, which is a bit faster, since the last 4 bits is not intended for security, rather just a bit of non-critical unpredictability between 2 IDs if they were generated within the same millisecond
  - move current (64-bit) generator to its own module
  - create a new 128-bit ID format
    - the timeestamp won't overflow until `di 25 jul 572823 17:58:01 UTC`
    - the timestamp has opt-in to microsecond precision
      - this is done in a way that still preserves sortability, between IDs with it on and with it off
    - a process ID field, so up to 256 processes can concurrently generate IDs
    - if you use microsecond precision timestamps, you can generate up to 32768 IDs per _microsecond_
    - if you don't use microsecond precision timestamps, you can generate up to roughly 33.5M IDs per _millisecond_
      - if microseconds aren't enabled, they are toggled off in the generator, and what would have been the microsecond field just becomes 10 more bits for incrementing within the same millisecond instead. so you generate the same amount of IDs per millisecond
      - in fact... you can actually generate more IDs per millisecond with microseconds off, because the microsecond field is seperated out from the rest of the time field, 10 bits to fit up to 999 microseconds, but 10 bits can actually store 1024 bits of data, which can be utilised here as a counter
    - still have 40 bits of randomness
      - doesn't turn into an increment like ULID has its random field do, since that makes the amount of IDs you can generate per period of time quite... random (ie. if you roll a high value, you can generate less IDs than if you were to roll a low value. Not a problem in practice, but, yknow, in theory it can be a problem)
- feature `int`
  - implemented (overflowing) add, (overflowing) sub, and (widening) mul for arrays of arbitrary length holding arbitrary integers (powered by `num-traits` feature, very nice)
- feature `lsl`
  - yeeted it all, we're starting over lol
  - started sketching out the structs for it again
- feature `mcu`
  - created the feature (unstable)
  - a port of the utilities found [here](https://github.com/material-foundation/material-color-utilities)
- feature `num-traits` <!-- TODO: list them? maybe? -->
  - gone a fully modular route, creating one trait for one bit of functionality (add checked, add (regular), add unchecked, etc. like that)
  - because of the above, there's over 100 structs as of now, and I'm not nearly done yet :p
  - plan to add overarching traits like `Int`, or maybe even `Number`, which will be a supertrait of as much of the other traits as makes sense
- feature `rand`
  - created the feature
  - contains some random generators
    - currently including thread local ChaCha generators, with variants for 8, 12, and 20 rounds, reseeding every 16KiB output from OS rng (context: `thread_rng` in `rand` crate is `ChaCha12` and reseeds every 64KiB output)
- feature `serialiser-binary`
  - created the feature (unstable)
  - implement all the basic features (ie. all JSON can be encoded in this format)
- feature `serialiser-binary-2`
  - created the feature (unstable)
  - creating `serialiser-binary` feature has led me to think about it more, and I think I can do things a bit better, but this would change the fundamental structure of the format, so I think it's best I do it in a new feature
    - `serialiser-binary` is probably never gonna stabilise as a result, and get removed at some point
  - wrote a small bit of spec
- feature `unicode`
  - created the feature (unstable)
  - implement some core functionality of unicode code points, UTF-8, UTF-16, and UTF-32 str/string
- feature `z85`
  - changed the z85 decode table from `[Option<u8>; 256]` to `[u8; 256]`
    - this halves the size of the table (from 512 to 256 bits, since `Option<u8>` is size 2 and `u8` is size 1)
    - this also unintentionally/unknowingly reduced decoding time by roughly 24%!
  - done a lot of internal safety commenting

## v0.9.0

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

- store requested size and use that to set the len, instead of relying on capacity (see [list of yanked reasons] #2)
- updated that overkill crate doc stuff
- feature `sudoku`:
  - fixed missing `chainer` and `iter` feature dependency
- feature `z85`:
  - updated docs

## v0.6.0

This release was yanked (see [list of yanked reasons] #2)

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

## v0.5.4

This release was yanked (see [list of yanked reasons] #2)

<!-- TODO: review the commits / write more detailed potentially -->

- gate binaries behind appropriate features
  - they were never meant to be properly part of crate API though (they are removed in a future version)
- feature `z85`:
  - fixed link in module doc not actually hyperlinking
  - check for decode overflow (see [list of yanked reasons] #1)

## v0.5.3

This release was yanked (see [list of yanked reasons] #1, #2)

<!-- TODO: review the commits / write more detailed potentially -->

- added MIT license file (however, MIT license was always specified in crate manifest)
- feature `clock-timer-2`:
  - added info about if a tick was delayed (`Tick::delayed` and `Tick::past_due`, both of which do the same thing)
- feature `z85`:
  - tweaked some docs

## v0.5.2

This release was yanked (see [list of yanked reasons] #1, #2)

<!-- TODO: review the commits / write more detailed potentially -->

- added keywords `decode`, `encode`, `hex`, `z85` to crate manifest
- feature `hex`:
  - added neon implementation for `aarch64` (automatically enabled on `aarch64` targets using runtime feature detection)
  - fixed uppercase hex unintentionally outputting lowercase

## v0.5.1

This release was yanked (see [list of yanked reasons] #1, #2)

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - fixed missing `thiserror` dependency

## v0.5.0

This release was yanked (see [list of yanked reasons] #1, #2)

<!-- TODO: review the commits / write more detailed potentially -->

- feature `hex`:
  - created the feature
  - added footnote about its performance in readme
- feature `z85`:
  - added footnote about its performance in readme

## v0.4.1

This release was yanked (see [list of yanked reasons] #1, #2)

<!-- TODO: review the commits / write more detailed potentially -->

- feature `z85`:
  - added test for non-padded compatability with `z85` crate (we now guarantee that for non-padded inputs (ie. input length is a multiple of 4), our output will be the same as `z85` crate)
  - improved performance
  - updated module doc

## v0.4.0

This release was yanked (see [list of yanked reasons] #1, #2)

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

## Yanked versions and reasons

[list of yanked reasons]: #yanked-versions-and-reasons

1. Depending on the compile setting for overflow checks, certain invalid input could cause integer overflow, and trigger either a panic or undefined behaviour
   - Affected features: `z85`
   - Affected versions: `>= 0.4.0, <= 0.5.3`
2. `Vec::with_capacity` is allowed to overallocate, and since `UnsafeBufWriteGuard` (internal struct) was using the `Vec`'s capacity to set the len at the end of the encoding/decoding, this could cause `UnsafeBufWriteGuard` to set the len too far out, include uninitialised memory into its "initialised" len, and cause undefined behaviour
   - Affected features: `hex`, `z85`
   - Affected versions: `>= 0.4.0, <= 0.6.0`
