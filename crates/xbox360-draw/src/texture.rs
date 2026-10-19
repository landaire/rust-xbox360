use alloc::vec::Vec;
use core::num::NonZeroU32;

use crate::draw::Rgba8;
use crate::draw::TextureId;
use crate::error::ImageLayoutError;
use crate::error::TextureError;
use crate::image::Image;

/// Largest texture width or height of every backend; the Xenos limit.
pub const MAX_TEXTURE_DIMENSION: u32 = 8192;

/// Borrowed premultiplied RGBA image, rows top to bottom, no padding, at
/// most [`MAX_TEXTURE_DIMENSION`] in either direction.
#[derive(Clone, Copy, Debug)]
pub struct ImageView<'a> {
    width: NonZeroU32,
    height: NonZeroU32,
    pixels: &'a [Rgba8],
}

impl<'a> ImageView<'a> {
    pub fn new(width: NonZeroU32, height: NonZeroU32, pixels: &'a [Rgba8]) -> Result<Self, ImageLayoutError> {
        check_layout(width, height, pixels.len())?;

        Ok(Self { width, height, pixels })
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    /// Exactly `width * height` pixels.
    pub fn pixels(&self) -> &'a [Rgba8] {
        self.pixels
    }
}

/// Texture storage of a backend. The UI runtime uploads decoded images and
/// glyph atlases through it and refers to them by [`TextureId`] in draw lists.
/// A draw command whose texture does not exist draws nothing.
pub trait TextureStore {
    fn create_texture(&mut self, image: Image) -> Result<TextureId, TextureError>;

    /// Replaces the contents of `id`. The size may change.
    fn update_texture(&mut self, id: TextureId, image: Image) -> Result<(), TextureError>;

    fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError>;
}

/// Texture id allocator and map shared by every backend, so that ids
/// behave identically everywhere. Freed slots are reused under a new
/// generation; a slot whose generation is used up is retired.
#[derive(Debug)]
pub struct TextureSlots<T> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
}

impl<T> TextureSlots<T> {
    pub fn new() -> Self {
        Self { slots: Vec::new(), free: Vec::new() }
    }

    pub fn insert(&mut self, value: T) -> Result<TextureId, TextureError> {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.value = Some(value);
            return Ok(TextureId::new(index, slot.generation));
        }

        let index = u32::try_from(self.slots.len()).map_err(|_| TextureError::SlotsExhausted)?;
        self.slots.push(Slot { generation: 0, value: Some(value) });

        Ok(TextureId::new(index, 0))
    }

    pub fn get(&self, id: TextureId) -> Option<&T> {
        let slot = self.slots.get(id.slot() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }

        slot.value.as_ref()
    }

    pub fn get_mut(&mut self, id: TextureId) -> Option<&mut T> {
        let slot = self.slots.get_mut(id.slot() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }

        slot.value.as_mut()
    }

    pub fn remove(&mut self, id: TextureId) -> Option<T> {
        let slot = self.slots.get_mut(id.slot() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }

        let value = slot.value.take()?;
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            self.free.push(id.slot());
        }

        Some(value)
    }
}

impl<T> Default for TextureSlots<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// Checks the size limit and that `pixel_count == width * height`.
pub(crate) fn check_layout(width: NonZeroU32, height: NonZeroU32, pixel_count: usize) -> Result<(), ImageLayoutError> {
    let (width, height) = (width.get(), height.get());
    if width > MAX_TEXTURE_DIMENSION || height > MAX_TEXTURE_DIMENSION {
        return Err(ImageLayoutError::TooLarge { width, height, max: MAX_TEXTURE_DIMENSION });
    }

    // At most 2^26, so the product fits a 32-bit `usize`.
    let expected = width as usize * height as usize;
    if pixel_count != expected {
        return Err(ImageLayoutError::PixelCount { width, height, expected, actual: pixel_count });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_ids_do_not_resolve_after_slot_reuse() {
        let mut slots = TextureSlots::new();
        let first = slots.insert('a').unwrap();
        assert_eq!(slots.remove(first), Some('a'));

        let second = slots.insert('b').unwrap();

        assert_ne!(second, first);
        assert_eq!(slots.get(first), None);
        assert_eq!(slots.remove(first), None);
        assert_eq!(slots.get(second), Some(&'b'));
    }

    #[test]
    fn slot_with_last_generation_is_retired() {
        let mut slots = TextureSlots::new();
        let id = slots.insert(1).unwrap();
        slots.slots[id.slot() as usize].generation = u32::MAX;
        let last = TextureId::new(id.slot(), u32::MAX);

        assert_eq!(slots.remove(last), Some(1));
        let next = slots.insert(2).unwrap();

        assert_ne!(next.slot(), id.slot());
        assert_eq!(slots.get(last), None);
    }

    #[test]
    fn image_view_checks_layout() {
        let one = NonZeroU32::MIN;
        let pixels = [Rgba8::WHITE; 2];
        let too_wide = NonZeroU32::new(MAX_TEXTURE_DIMENSION + 1).unwrap();

        assert!(ImageView::new(NonZeroU32::new(2).unwrap(), one, &pixels).is_ok());
        assert_eq!(
            ImageView::new(one, one, &pixels).unwrap_err(),
            ImageLayoutError::PixelCount { width: 1, height: 1, expected: 1, actual: 2 }
        );
        assert_eq!(
            ImageView::new(too_wide, one, &pixels).unwrap_err(),
            ImageLayoutError::TooLarge { width: MAX_TEXTURE_DIMENSION + 1, height: 1, max: MAX_TEXTURE_DIMENSION }
        );
    }
}
