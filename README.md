# wiwi

A (not so) lil lib containing many things and Stuff&trade; (some useful, some silly), intending to serve as an "extended standard library" of sorts.

## Features

`wiwi` can do many things, and it is probably best to check out the documentation for this crate, for the latest published version [on docs.rs](https://docs.rs/wiwi/latest/wiwi), or for the latest commit in the repo [on wiwi.kiwin.gay](https://wiwi.kiwin.gay/wiwi).

## Platform support

This crate will only _officially_ support macOS and Linux, on 64-bit platforms.

So far we have tried to be aware of 32-bit platforms, although no tests are being run for them at this moment. We compile error on 16-bit platforms, as we highly doubt that there will be a need to compile this library on those targets; however, do file an issue or a PR if you need to do it, we'd rather do it correct than ship compiling but half working code, and would be happy to add support!

Windows support is roughly the same as the above. We aren't running tests on windows at the moment. We would like to support windows as well, but our ability to do so would be limited at best. As with 16-bit platforms, please do file issues or PRs if you find an issue or need support for something that doesn't work on windows at the moment.
