//! Renders [`DrawList`]s with the Xenos GPU of the Xbox 360 and implements
//! [`TextureStore`] over GPU memory. Frames go through the command ring of an
//! `xbox360_video::Display`: draws into EDRAM, a resolve into the display's
//! back buffer, and a flip at vertical blank. The shader programs come from
//! this crate's own encoder ([`shader`]).
//!
//! The encoder builds on every target; the renderer exists for
//! `target_os = "xbox360"` only.

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod shader;

#[cfg(target_os = "xbox360")]
mod renderer;

#[cfg(target_os = "xbox360")]
pub use self::renderer::Gpu;
#[cfg(target_os = "xbox360")]
pub use self::renderer::GpuError;
