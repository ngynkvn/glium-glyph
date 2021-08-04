## ?

This is a fork repo.
## glium-glyph

[![docs](https://docs.rs/glium-glyph/badge.svg)](https://docs.rs/crate/glium-glyph)
[![crates.io](https://img.shields.io/crates/v/glium-glyph.svg)](https://crates.io/crates/glium-glyph)
[![dependency status](https://deps.rs/repo/github/ngynkvn/glium-glyph/status.svg)](https://deps.rs/repo/github/ngynkvn/glium-glyph)

[Glium](https://github.com/glium/glium) frontend for the [glyph-brush](https://github.com/alexheretic/glyph-brush) text renderer.

It bases on the [gfx_glyph](https://crates.io/crates/gfx_glyph) crate. While it works, some docs are still missing, and maybe one API call or two. I've written it mostly to scratch a personal itch of mine. If you find something to be missing, file an issue or send me a PR!

TODO:

* examples and docs are still about the gfx-brush crate
* mode for crisp text rendering (exposing texture interpolation commands send to glium)
* mode for crisp text rendering (making rusttype [not "alias"](https://gitlab.redox-os.org/redox-os/rusttype/issues/61), might need changes in glyph-brush)
