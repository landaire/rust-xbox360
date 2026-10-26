use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;
use core::num::NonZeroU32;

use egui::ClippedPrimitive;
use egui::ImageData;
use egui::TexturesDelta;
use egui::epaint::Primitive;
use xbox360_draw::draw::AddressMode;
use xbox360_draw::draw::BlendMode;
use xbox360_draw::draw::ClipRect;
use xbox360_draw::draw::ColorWriteMask;
use xbox360_draw::draw::DrawList;
use xbox360_draw::draw::DrawState;
use xbox360_draw::draw::Filter;
use xbox360_draw::draw::Rgba8;
use xbox360_draw::draw::Sampler;
use xbox360_draw::draw::TextureId;
use xbox360_draw::draw::Vertex;
use xbox360_draw::error::DrawListError;
use xbox360_draw::error::ImageLayoutError;
use xbox360_draw::error::TextureError;
use xbox360_draw::image::Image;
use xbox360_draw::texture::TextureStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintError {
    Texture(TextureError),
    Image(ImageLayoutError),
    DrawList(DrawListError),
    /// egui sent a partial update for a texture it never set in full.
    UnknownTexture(egui::TextureId),
}

impl fmt::Display for PaintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Texture(error) => write!(f, "texture store: {error}"),
            Self::Image(error) => write!(f, "texture image: {error}"),
            Self::DrawList(error) => write!(f, "draw list: {error}"),
            Self::UnknownTexture(id) => write!(f, "partial update of unknown egui texture {id:?}"),
        }
    }
}

impl core::error::Error for PaintError {}

impl From<TextureError> for PaintError {
    fn from(error: TextureError) -> Self {
        Self::Texture(error)
    }
}

impl From<ImageLayoutError> for PaintError {
    fn from(error: ImageLayoutError) -> Self {
        Self::Image(error)
    }
}

impl From<DrawListError> for PaintError {
    fn from(error: DrawListError) -> Self {
        Self::DrawList(error)
    }
}

/// An egui texture held by the store, with its pixels for partial updates.
struct Texture {
    id: TextureId,
    width: usize,
    pixels: Vec<Rgba8>,
    sampler: Sampler,
}

/// Paints egui output into draw lists. Positions are in pixels:
/// `pixels_per_point` scales egui's points.
#[derive(Default)]
pub struct Painter {
    textures: BTreeMap<egui::TextureId, Texture>,
}

impl Painter {
    pub fn new() -> Self {
        Self::default()
    }

    /// The store's texture for an egui texture, once egui has set it.
    pub fn texture(&self, id: egui::TextureId) -> Option<TextureId> {
        self.textures.get(&id).map(|texture| texture.id)
    }

    /// Apply egui's texture changes, append `primitives` to `list`, then free
    /// the textures egui released. Meshes of unknown textures draw nothing.
    pub fn paint(
        &mut self,
        store: &mut dyn TextureStore,
        list: &mut DrawList,
        textures: &TexturesDelta,
        primitives: &[ClippedPrimitive],
        pixels_per_point: f32,
    ) -> Result<(), PaintError> {
        for (id, delta) in &textures.set {
            self.set_texture(store, *id, delta)?;
        }
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for primitive in primitives {
            let Primitive::Mesh(mesh) = &primitive.primitive else {
                // Paint callbacks have no meaning without a host graphics API.
                continue;
            };
            let Some(texture) = self.textures.get(&mesh.texture_id) else {
                continue;
            };
            let clip = primitive.clip_rect;
            let clip = ClipRect {
                left: (clip.min.x * pixels_per_point).floor() as i32,
                top: (clip.min.y * pixels_per_point).floor() as i32,
                right: (clip.max.x * pixels_per_point).ceil() as i32,
                bottom: (clip.max.y * pixels_per_point).ceil() as i32,
            };
            vertices.clear();
            vertices.extend(mesh.vertices.iter().map(|vertex| Vertex {
                position: [vertex.pos.x * pixels_per_point, vertex.pos.y * pixels_per_point],
                uv: [vertex.uv.x, vertex.uv.y],
                color: color(vertex.color),
            }));
            triangles.clear();
            triangles.extend(mesh.indices.chunks_exact(3).map(|triangle| [triangle[0], triangle[1], triangle[2]]));
            let state = DrawState { texture: Some(texture.id), sampler: texture.sampler, blend: BlendMode::Normal, clip: Some(clip), color_write: ColorWriteMask::ALL, shadow: None };
            list.push(state, &vertices, &triangles)?;
        }
        for id in &textures.free {
            if let Some(texture) = self.textures.remove(id) {
                store.destroy_texture(texture.id)?;
            }
        }
        Ok(())
    }

    fn set_texture(&mut self, store: &mut dyn TextureStore, id: egui::TextureId, delta: &egui::epaint::ImageDelta) -> Result<(), PaintError> {
        let ImageData::Color(image) = &delta.image;
        let [width, height] = image.size;
        let patch: Vec<Rgba8> = image.pixels.iter().map(|&pixel| color(pixel)).collect();
        let sampler = sampler(delta.options);
        match delta.pos {
            None => {
                let created = image_of(width, height, patch.clone())?;
                match self.textures.get_mut(&id) {
                    Some(texture) => {
                        store.update_texture(texture.id, created)?;
                        texture.width = width;
                        texture.pixels = patch;
                        texture.sampler = sampler;
                    }
                    None => {
                        let texture = store.create_texture(created)?;
                        self.textures.insert(id, Texture { id: texture, width, pixels: patch, sampler });
                    }
                }
            }
            Some([x, y]) => {
                let texture = self.textures.get_mut(&id).ok_or(PaintError::UnknownTexture(id))?;
                for (row, source) in patch.chunks_exact(width).enumerate() {
                    let start = (y + row) * texture.width + x;
                    texture.pixels[start..start + width].copy_from_slice(source);
                }
                let height = texture.pixels.len() / texture.width;
                store.update_texture(texture.id, image_of(texture.width, height, texture.pixels.clone())?)?;
            }
        }
        Ok(())
    }
}

/// egui's premultiplied sRGBA as `Rgba8`.
fn color(color: egui::Color32) -> Rgba8 {
    let [red, green, blue, alpha] = color.to_array();
    Rgba8::new(red, green, blue, alpha)
}

fn sampler(options: egui::TextureOptions) -> Sampler {
    let filter = match options.magnification {
        egui::TextureFilter::Nearest => Filter::Nearest,
        egui::TextureFilter::Linear => Filter::Linear,
    };
    let address = match options.wrap_mode {
        egui::TextureWrapMode::ClampToEdge => AddressMode::Clamp,
        egui::TextureWrapMode::Repeat => AddressMode::Wrap,
        egui::TextureWrapMode::MirroredRepeat => AddressMode::Mirror,
    };
    Sampler { filter, address_u: address, address_v: address, ..Sampler::default() }
}

fn image_of(width: usize, height: usize, pixels: Vec<Rgba8>) -> Result<Image, PaintError> {
    let too_large = ImageLayoutError::TooLarge { width: width as u32, height: height as u32, max: u32::MAX };
    let width = NonZeroU32::new(width as u32).ok_or(too_large)?;
    let height = NonZeroU32::new(height as u32).ok_or(too_large)?;
    Ok(Image::new(width, height, pixels)?)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::vec::Vec;

    use xbox360_draw::texture::TextureSlots;

    use super::*;

    #[derive(Default)]
    struct Store {
        slots: TextureSlots<Image>,
    }

    impl TextureStore for Store {
        fn create_texture(&mut self, image: Image) -> Result<TextureId, TextureError> {
            self.slots.insert(image)
        }
        fn update_texture(&mut self, id: TextureId, image: Image) -> Result<(), TextureError> {
            *self.slots.get_mut(id).ok_or(TextureError::UnknownTexture { id })? = image;
            Ok(())
        }
        fn create_target(&mut self, width: NonZeroU32, height: NonZeroU32) -> Result<TextureId, TextureError> {
            let pixels = std::vec![Rgba8::TRANSPARENT; width.get() as usize * height.get() as usize];
            let image = Image::new_straight(width, height, pixels).map_err(|error| match error {
                ImageLayoutError::TooLarge { width, height, max } => TextureError::TooLarge { width, height, max },
                ImageLayoutError::PixelCount { .. } => unreachable!("the pixel count matches the size"),
            })?;
            self.slots.insert(image)
        }
        fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError> {
            self.slots.remove(id).map(drop).ok_or(TextureError::UnknownTexture { id })
        }
    }

    #[test]
    fn text_samples_covered_texels() {
        let context = egui::Context::default();
        let mut store = Store::default();
        let mut painter = Painter::new();
        let mut list = DrawList::new();
        for _ in 0..2 {
            let input = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(640.0, 480.0))), ..Default::default() };
            let output = context.run(input, |context| {
                egui::CentralPanel::default().show(context, |ui| ui.label("Hello"));
            });
            let primitives = context.tessellate(output.shapes, output.pixels_per_point);
            list.clear();
            painter.paint(&mut store, &mut list, &output.textures_delta, &primitives, 1.0).unwrap();
        }
        let command = &list.commands()[0];
        let image = store.slots.get(command.texture().unwrap()).unwrap();
        let (w, h) = (image.width().get() as f32, image.height().get() as f32);
        let covered: Vec<u32> = list
            .vertices()
            .iter()
            .map(|v| {
                let (x, y) = ((v.uv[0] * w) as usize, (v.uv[1] * h) as usize);
                image.pixels()[y.min(h as usize - 1) * w as usize + x.min(w as usize - 1)].0 >> 24
            })
            .collect();
        std::println!("atlas {}x{}, {} vertices, alpha samples {:?}", w, h, covered.len(), &covered[..covered.len().min(40)]);
        assert!(covered.iter().any(|&alpha| alpha != 0 && alpha != 255));
    }
}
