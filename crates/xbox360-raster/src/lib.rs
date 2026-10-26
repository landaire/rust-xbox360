//! Software rasterizer and the reference output for every backend. Coverage,
//! stepping and blending are integer; attribute setup is `f64` arithmetic
//! without contraction. Both give bit-identical output on every host.

#![no_std]
#![cfg_attr(
    all(target_arch = "powerpc", target_os = "xbox360"),
    feature(core_intrinsics, powerpc_target_feature, repr_simd, stdarch_powerpc),
    allow(internal_features)
)]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod error;

mod blend;
mod raster;
mod shade;
mod target;
#[cfg(all(target_arch = "powerpc", target_os = "xbox360"))]
mod vmx;

use alloc::vec;
use alloc::vec::Vec;
use core::mem;
use core::num::NonZeroU32;

// The draw list contract lives in xbox360-draw; the rasterizer reaches it
// through these crate paths.
use xbox360_draw::draw;
use xbox360_draw::image;
use xbox360_draw::texture;

use crate::draw::BlendMode;
use crate::draw::ClipRect;
use crate::draw::ColorWriteMask;
use crate::draw::DrawCommand;
use crate::draw::DrawList;
use crate::draw::Rgba8;
use crate::draw::Sampler;
use crate::draw::TextShadow;
use crate::draw::TextureId;
use crate::draw::Vertex;
use crate::error::TextureError;
use crate::image::AlphaMode;
use crate::image::Image;
use crate::texture::MAX_TEXTURE_DIMENSION;
use crate::texture::TextureSlots;
use crate::texture::TextureStore;

use self::blend::Blend;
use self::blend::ColorEncoding;
use self::raster::AlignedRect;
use self::raster::CoverBounds;
use self::raster::WindowBounds;
use self::raster::for_each_triangle;
use self::raster::misses_window;
use self::raster::rectangle_covers;
use self::shade::Scratch;
use self::shade::Shader;
use self::shade::TextureRef;

pub use self::target::Bands;
pub use self::target::RenderTarget;
pub use self::target::TargetBand;

/// Renders draw lists into memory and owns the textures they refer to.
#[derive(Debug)]
pub struct SoftwareRenderer {
    textures: TextureSlots<Texture>,
}

#[derive(Debug)]
enum Texture {
    Image {
        image: Image,
        /// Every texel has alpha 255.
        is_opaque: bool,
    },
    /// Straight texels that target passes draw into, `width * height` of
    /// them without row padding.
    Target {
        width: NonZeroU32,
        height: NonZeroU32,
        texels: Vec<Rgba8>,
    },
}

impl Texture {
    fn from_image(image: Image) -> Self {
        let is_opaque = image.pixels().iter().all(|texel| texel.alpha() == u8::MAX);

        Self::Image { image, is_opaque }
    }
}

/// What a run of commands draws into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Destination {
    Frame,
    /// The texels of this render target, which the renderer holds apart
    /// from the texture store while the pass draws.
    Target(TextureId),
}

impl SoftwareRenderer {
    pub fn new() -> Self {
        Self {
            textures: TextureSlots::new(),
        }
    }

    /// Renders the target passes of `list`, then its frame into `target`.
    pub fn render(&mut self, list: &DrawList, target: &mut RenderTarget<'_>) {
        self.render_target_passes(list);
        self.render_band(list, &mut target.band());
    }

    /// Renders the target passes of `list` into their textures, in order.
    /// A pass whose target is not a render target of this renderer draws
    /// nothing. Callers that render the frame with [`Self::render_band`]
    /// call this first.
    pub fn render_target_passes(&mut self, list: &DrawList) {
        let mut scratch = Scratch::new();
        for pass in list.target_passes() {
            let id = pass.target();
            let Some(Texture::Target {
                width,
                height,
                texels,
            }) = self.textures.get_mut(id)
            else {
                continue;
            };
            let (width, height) = (width.get(), height.get());
            let mut texels = mem::take(texels);

            texels.fill(Rgba8::TRANSPARENT);
            let mut band = TargetBand::from_texels(&mut texels, width, height);
            self.draw_commands(
                list,
                list.pass_commands(pass),
                &mut band,
                Destination::Target(id),
                &mut scratch,
            );

            let Some(Texture::Target { texels: slot, .. }) = self.textures.get_mut(id) else {
                unreachable!("drawing a pass leaves the texture store unchanged");
            };
            *slot = texels;
        }
    }

    /// Renders the frame rows of `band` only. The target passes are not
    /// rendered; [`Self::render_target_passes`] renders them before the
    /// bands of a frame. Disjoint bands of one target may be rendered
    /// concurrently and produce the same pixels as [`Self::render`].
    pub fn render_band(&self, list: &DrawList, band: &mut TargetBand<'_>) {
        let band_rect = band.rect();
        if band_rect.is_empty() {
            return;
        }

        // A command that covers the whole band with opaque pixels hides the
        // clear and every command before it.
        let commands = list.commands();
        let first = commands
            .iter()
            .rposition(|command| self.covers_opaque(list, command, band_rect));
        if first.is_none()
            && let Some(color) = list.clear_color()
        {
            for y in band.top()..band.bottom() {
                band.row_mut(y).fill(color);
            }
        }

        self.draw_commands(
            list,
            &commands[first.unwrap_or(0)..],
            band,
            Destination::Frame,
            &mut Scratch::new(),
        );
    }

    fn draw_commands(
        &self,
        list: &DrawList,
        commands: &[DrawCommand],
        band: &mut TargetBand<'_>,
        destination: Destination,
        scratch: &mut Scratch,
    ) {
        let band_rect = band.rect();
        let encoding = match destination {
            Destination::Frame => ColorEncoding::Premultiplied,
            Destination::Target(_) => ColorEncoding::Straight,
        };
        for command in commands {
            let window = command
                .clip()
                .map_or(band_rect, |clip| clip.intersect(band_rect));
            if window.is_empty() {
                continue;
            }

            let texture = match command.texture() {
                // The pass holds the texels of its own target, which its
                // commands do not sample.
                Some(id) if destination == Destination::Target(id) => continue,
                Some(id) => {
                    let Some(texture) = self.sampled_texture(id) else {
                        continue;
                    };
                    Some(texture)
                }
                None => None,
            };
            let blend = Blend::new(command.blend(), command.color_write(), encoding);

            // `DrawList` guarantees that every index addresses a vertex.
            let vertices = list.vertices();
            let bounds = WindowBounds::new(window);
            let triangles = list.command_triangles(command);
            let corners = |index: usize| {
                let [first, second, third] = triangles[index];
                [
                    &vertices[first as usize],
                    &vertices[second as usize],
                    &vertices[third as usize],
                ]
            };
            let mut index = 0;
            while index < triangles.len() {
                let first = corners(index);
                if index + 1 < triangles.len() {
                    let second = corners(index + 1);
                    if misses_window(first, bounds) && misses_window(second, bounds) {
                        index += AlignedRect::TRIANGLES;
                        continue;
                    }
                    if let Some(rect) = AlignedRect::new(first, second) {
                        index += AlignedRect::TRIANGLES;
                        let shading = Shading {
                            texture,
                            sampler: command.sampler(),
                            blend,
                            shadow: command.shadow(),
                        };
                        draw_rect(&rect, window, shading, scratch, band);
                        continue;
                    }
                }
                index += 1;
                if misses_window(first, bounds) {
                    continue;
                }

                for_each_triangle(first, |triangle| {
                    let Some(mut scan) = triangle.scan(window) else {
                        return;
                    };

                    let shader = Shader::new(triangle, texture, command.sampler(), blend, command.shadow());
                    while let Some(span) = scan.next_span() {
                        shader.shade_span(
                            scratch,
                            band.row_mut(span.row as u32),
                            span.row,
                            span.start,
                            span.end,
                        );
                    }
                });
            }
        }
    }

    fn sampled_texture(&self, id: TextureId) -> Option<TextureRef<'_>> {
        let texture = match self.textures.get(id)? {
            Texture::Image { image, .. } => TextureRef {
                pixels: image.pixels(),
                width: image.width().get(),
                height: image.height().get(),
                encoding: match image.alpha_mode() {
                    AlphaMode::Premultiplied => ColorEncoding::Premultiplied,
                    AlphaMode::Straight => ColorEncoding::Straight,
                },
            },
            Texture::Target {
                width,
                height,
                texels,
            } => TextureRef {
                pixels: texels,
                width: width.get(),
                height: height.get(),
                encoding: ColorEncoding::Straight,
            },
        };

        Some(texture)
    }

    /// Whether `command` replaces every pixel of `window`: it blends
    /// normally with opaque colors and texels into every channel, its clip
    /// contains the window, and two consecutive triangles form a rectangle
    /// that covers it. Normal blending of an opaque source writes the
    /// source exactly.
    fn covers_opaque(&self, list: &DrawList, command: &DrawCommand, window: ClipRect) -> bool {
        if command.blend() != BlendMode::Normal
            || command.color_write() != ColorWriteMask::ALL
            || command
                .clip()
                .is_some_and(|clip| clip.intersect(window) != window)
        {
            return false;
        }
        if let Some(id) = command.texture()
            && !matches!(
                self.textures.get(id),
                Some(Texture::Image {
                    is_opaque: true,
                    ..
                })
            )
        {
            return false;
        }

        let vertices = list.vertices();
        let bounds = CoverBounds::new(window);
        let triangles = list.command_triangles(command);
        let corners = |index: usize| {
            let [first, second, third] = triangles[index];
            [
                &vertices[first as usize],
                &vertices[second as usize],
                &vertices[third as usize],
            ]
        };
        let opaque = |triangle: [&Vertex; 3]| {
            triangle[0].color.alpha() == u8::MAX
                && triangle[1].color.alpha() == u8::MAX
                && triangle[2].color.alpha() == u8::MAX
        };
        (1..triangles.len()).any(|index| {
            let (first, second) = (corners(index - 1), corners(index));
            rectangle_covers(first, second, bounds) && opaque(first) && opaque(second)
        })
    }
}

impl Default for SoftwareRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureStore for SoftwareRenderer {
    fn create_texture(&mut self, image: Image) -> Result<TextureId, TextureError> {
        self.textures.insert(Texture::from_image(image))
    }

    fn update_texture(&mut self, id: TextureId, image: Image) -> Result<(), TextureError> {
        let texture = self
            .textures
            .get_mut(id)
            .ok_or(TextureError::UnknownTexture { id })?;
        if let Texture::Target { .. } = texture {
            return Err(TextureError::RenderTarget { id });
        }
        *texture = Texture::from_image(image);

        Ok(())
    }

    fn create_target(
        &mut self,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> Result<TextureId, TextureError> {
        if width.get() > MAX_TEXTURE_DIMENSION || height.get() > MAX_TEXTURE_DIMENSION {
            return Err(TextureError::TooLarge {
                width: width.get(),
                height: height.get(),
                max: MAX_TEXTURE_DIMENSION,
            });
        }

        // At most 2^26 texels, so the count fits a 32-bit `usize`.
        let texels = vec![Rgba8::TRANSPARENT; width.get() as usize * height.get() as usize];

        self.textures.insert(Texture::Target {
            width,
            height,
            texels,
        })
    }

    fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError> {
        self.textures
            .remove(id)
            .ok_or(TextureError::UnknownTexture { id })?;

        Ok(())
    }
}

/// Per-command inputs of [`Shader::new`].
#[derive(Clone, Copy, Debug)]
struct Shading<'a> {
    texture: Option<TextureRef<'a>>,
    sampler: Sampler,
    blend: Blend,
    shadow: Option<TextShadow>,
}

fn draw_rect(
    rect: &AlignedRect,
    window: ClipRect,
    shading: Shading<'_>,
    scratch: &mut Scratch,
    band: &mut TargetBand<'_>,
) {
    let (left, right) = (rect.left.max(window.left), rect.right.min(window.right));
    let (top, bottom) = (rect.top.max(window.top), rect.bottom.min(window.bottom));
    if left >= right || top >= bottom {
        return;
    }

    let shader = Shader::new(&rect.triangle, shading.texture, shading.sampler, shading.blend, shading.shadow);
    for y in top..bottom {
        shader.shade_span(scratch, band.row_mut(y as u32), y, left, right);
    }
}

#[cfg(test)]
mod tests;
