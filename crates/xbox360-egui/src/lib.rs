//! egui on the Xbox 360: [`Painter`] turns egui's tessellated output into an
//! xbox360-draw [`DrawList`] and keeps egui's textures in any
//! [`TextureStore`] (the GPU renderer of xbox360-gpu on the console), and
//! [`input`] turns controller and keyboard keystrokes into egui events.
//! Builds for every target.

#![no_std]

extern crate alloc;

pub mod input;
mod painter;

pub use self::painter::PaintError;
pub use self::painter::Painter;
