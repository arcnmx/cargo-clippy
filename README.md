# cargo-clippy

[![travis-badge][]][travis] [![license-badge][]][license]

`cargo clippy` runs [clippy] on the current project.
This is a fork of [cargo-check].

You may also pass additional parameters, like so:

    cargo clippy --release -- -Dclippy -Wclippy_pedantic


**NOTE:** requires nightly Rust.


## Installation

`cargo install --git https://github.com/arcnmx/cargo-clippy.git`


[travis-badge]: https://img.shields.io/travis/arcnmx/cargo-clippy/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/cargo-clippy
[license-badge]: https://img.shields.io/badge/license-MIT-lightgray.svg?style=flat-square
[license]: https://github.com/arcnmx/cargo-clippy/blob/master/LICENSE
[clippy]: https://github.com/Manishearth/rust-clippy
[cargo-check]: https://github.com/rsolomo/cargo-check
