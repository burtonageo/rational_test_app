# About

This binary demonstrates an error where using LTO with static linking causes a bitcode error.

## Structure

The top-level crate is a binary which uses the `rational` library to demonstrate that the
functionality is present.

The `rational` library is made up of 3 main components:

* The `rational_impl` library is a `cdylib` and `static` library, which provides a few simple APIs.
* The `rational_impl_types` provides shared types for `rational` and `rational_impl`.
* The `rational` crate builds the `rational_impl` crate using a build script, and then links to it.
  It does not depend on `rational_impl` directly, so it should be an independent library as far as
  cargo is concerned.

Static linking can be controlled using the `link_static` feature.

## Bug

When lto is enabled for the `rational_test_app` library alongside the `link_static` feature, (at
least on MacOS), the following error occurs:

```
error: failed to get bitcode from object file for LTO (could not find requested section)
```

This can be reproduced by running `cargo build --release` in this repository. If you use
a debug build, or enable dynamic linking in the release build (using
`cargo build --release --no-default-features`), then this bug will not trigger.
