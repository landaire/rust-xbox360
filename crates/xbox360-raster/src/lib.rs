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

// The draw list contract lives in xbox360-draw; the rasterizer reaches it
// through these crate paths.
use xbox360_draw::draw;
use xbox360_draw::image;
use xbox360_draw::texture;

use crate::draw::BlendMode;
use crate::draw::ClipRect;
use crate::draw::DrawCommand;
use crate::draw::DrawList;
use crate::draw::TextureId;
use crate::draw::Vertex;
use crate::error::TextureError;
use crate::image::Image;
use crate::texture::TextureSlots;
use crate::texture::TextureStore;

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
struct Texture {
    image: Image,
    /// Every texel has alpha 255.
    is_opaque: bool,
}

impl Texture {
    fn new(image: Image) -> Self {
        let is_opaque = image.pixels().iter().all(|texel| texel.alpha() == u8::MAX);

        Self { image, is_opaque }
    }
}

impl SoftwareRenderer {
    pub fn new() -> Self {
        Self {
            textures: TextureSlots::new(),
        }
    }

    pub fn render(&self, list: &DrawList, target: &mut RenderTarget<'_>) {
        self.render_band(list, &mut target.band());
    }

    /// Renders the rows of `band` only. Disjoint bands of one target may be
    /// rendered concurrently and produce the same pixels as [`Self::render`].
    pub fn render_band(&self, list: &DrawList, band: &mut TargetBand<'_>) {
        let band_rect = ClipRect {
            left: 0,
            top: band.top() as i32,
            right: band.width() as i32,
            bottom: band.bottom() as i32,
        };
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

        let mut scratch = Scratch::new();
        for command in &commands[first.unwrap_or(0)..] {
            let window = command
                .clip()
                .map_or(band_rect, |clip| clip.intersect(band_rect));
            if window.is_empty() {
                continue;
            }

            let texture = match command.texture() {
                Some(id) => {
                    let Some(texture) = self.sampled_texture(id) else {
                        continue;
                    };
                    Some(texture)
                }
                None => None,
            };

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
                        self.draw_rect(&rect, window, texture, command, &mut scratch, band);
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

                    let shader = Shader::new(triangle, texture, command.sampler(), command.blend());
                    while let Some(span) = scan.next_span() {
                        shader.shade_span(
                            &mut scratch,
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

    fn draw_rect(
        &self,
        rect: &AlignedRect,
        window: ClipRect,
        texture: Option<TextureRef<'_>>,
        command: &DrawCommand,
        scratch: &mut Scratch,
        band: &mut TargetBand<'_>,
    ) {
        let (left, right) = (rect.left.max(window.left), rect.right.min(window.right));
        let (top, bottom) = (rect.top.max(window.top), rect.bottom.min(window.bottom));
        if left >= right || top >= bottom {
            return;
        }

        let shader = Shader::new(&rect.triangle, texture, command.sampler(), command.blend());
        for y in top..bottom {
            shader.shade_span(scratch, band.row_mut(y as u32), y, left, right);
        }
    }

    fn sampled_texture(&self, id: TextureId) -> Option<TextureRef<'_>> {
        let image = &self.textures.get(id)?.image;

        Some(TextureRef {
            pixels: image.pixels(),
            width: image.width().get(),
            height: image.height().get(),
        })
    }

    /// Whether `command` replaces every pixel of `window`: it blends
    /// normally with opaque colors and texels, its clip contains the
    /// window, and two consecutive triangles form a rectangle that covers
    /// it. Normal blending of an opaque source writes the source exactly.
    fn covers_opaque(&self, list: &DrawList, command: &DrawCommand, window: ClipRect) -> bool {
        if command.blend() != BlendMode::Normal
            || command
                .clip()
                .is_some_and(|clip| clip.intersect(window) != window)
        {
            return false;
        }
        if let Some(id) = command.texture()
            && !self
                .textures
                .get(id)
                .is_some_and(|texture| texture.is_opaque)
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
        self.textures.insert(Texture::new(image))
    }

    fn update_texture(&mut self, id: TextureId, image: Image) -> Result<(), TextureError> {
        let texture = self
            .textures
            .get_mut(id)
            .ok_or(TextureError::UnknownTexture { id })?;
        *texture = Texture::new(image);

        Ok(())
    }

    fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError> {
        self.textures
            .remove(id)
            .ok_or(TextureError::UnknownTexture { id })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
