# cargo-clippy

[![travis-badge][]][travis] [![license-badge][]][license]

`cargo clippy` runs [clippy] on the current project.
This is a fork of [cargo-check].

**NOTE:** requires nightly Rust.

## Installation

First compile with `cargo build --release`, then add
`target/release/cargo-clippy` into your `$PATH`.

**WARNING:** simply copying it is not enough; you must
specifically add the directory to your path, or use
a symbolic link instead.

[travis-badge]: https://img.shields.io/travis/arcnmx/cargo-clippy/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/cargo-clippy
[license-badge]: https://img.shields.io/badge/license-MIT-lightgray.svg?style=flat-square
[license]: https://github.com/arcnmx/cargo-clippy/blob/master/LICENSE
[clippy]: https://github.com/Manishearth/rust-clippy
[cargo-check]: https://github.com/rsolomo/cargo-check
