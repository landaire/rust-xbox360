//! Keystrokes from `XamInputGetKeystrokeEx` as egui events. The controller
//! drives egui's keyboard navigation: the directions move focus, A presses
//! the focused widget, B leaves it, the shoulders tab between widgets.

use xbox360_sys::xam::Keystroke;
use xbox360_sys::xam::KeystrokeFlags;
use xbox360_sys::xam::PadKey;

/// Unicode field of a keystroke that types no character.
const NO_CHARACTER: u16 = 0;

/// The egui key and modifiers for a controller key.
fn pad_key(key: PadKey) -> Option<(egui::Key, egui::Modifiers)> {
    let none = egui::Modifiers::NONE;
    Some(match key {
        PadKey::DPadUp | PadKey::LeftStickUp => (egui::Key::ArrowUp, none),
        PadKey::DPadDown | PadKey::LeftStickDown => (egui::Key::ArrowDown, none),
        PadKey::DPadLeft | PadKey::LeftStickLeft => (egui::Key::ArrowLeft, none),
        PadKey::DPadRight | PadKey::LeftStickRight => (egui::Key::ArrowRight, none),
        PadKey::A | PadKey::Start => (egui::Key::Space, none),
        PadKey::B | PadKey::Back => (egui::Key::Escape, none),
        PadKey::RightShoulder => (egui::Key::Tab, none),
        PadKey::LeftShoulder => (egui::Key::Tab, egui::Modifiers::SHIFT),
        _ => return None,
    })
}

/// Append the egui events of `keystroke` to `events`.
pub fn keystroke_events(keystroke: &Keystroke, events: &mut alloc::vec::Vec<egui::Event>) {
    let flags = keystroke.flags;
    let pressed = flags.contains(KeystrokeFlags::KEY_DOWN);
    if !pressed && !flags.contains(KeystrokeFlags::KEY_UP) {
        return;
    }
    if let Some((key, modifiers)) = PadKey::from_raw(keystroke.virtual_key).and_then(pad_key) {
        let repeat = flags.contains(KeystrokeFlags::REPEAT);
        events.push(egui::Event::Key { key, physical_key: None, pressed, repeat, modifiers });
        return;
    }
    if pressed && keystroke.unicode != NO_CHARACTER {
        if let Some(character) = char::from_u32(u32::from(keystroke.unicode)) {
            let mut text = alloc::string::String::new();
            text.push(character);
            events.push(egui::Event::Text(text));
        }
    }
}
