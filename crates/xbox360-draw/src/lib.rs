//! Draw lists and texture storage shared by renderers: triangles in target
//! pixel coordinates with per-command texture, sampler, blend mode and clip
//! rectangle, premultiplied RGBA pixels, and the texture id allocator every
//! backend uses. Builds for every target.

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod draw;
pub mod error;
pub mod image;
pub mod texture;
