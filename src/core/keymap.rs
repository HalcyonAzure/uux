//! Windows virtual-key codes (what the input layer produces and what a Windows
//! UU host expects) to macOS virtual key codes (`kVK_*` in Carbon
//! HIToolbox/Events.h). A macOS UU host injects the received number as a kVK
//! code without translating it, so a Mac host must be sent kVK values.
//! Keys with no Mac counterpart return None and are not sent.

pub fn mac_keycode(vk: u16) -> Option<u16> {
    Some(match vk {
        0x41 => 0x00, // A
        0x53 => 0x01, // S
        0x44 => 0x02, // D
        0x46 => 0x03, // F
        0x48 => 0x04, // H
        0x47 => 0x05, // G
        0x5A => 0x06, // Z
        0x58 => 0x07, // X
        0x43 => 0x08, // C
        0x56 => 0x09, // V
        0x42 => 0x0B, // B
        0x51 => 0x0C, // Q
        0x57 => 0x0D, // W
        0x45 => 0x0E, // E
        0x52 => 0x0F, // R
        0x59 => 0x10, // Y
        0x54 => 0x11, // T
        0x31 => 0x12, // 1
        0x32 => 0x13, // 2
        0x33 => 0x14, // 3
        0x34 => 0x15, // 4
        0x36 => 0x16, // 6
        0x35 => 0x17, // 5
        0xBB => 0x18, // = +
        0x39 => 0x19, // 9
        0x37 => 0x1A, // 7
        0xBD => 0x1B, // - _
        0x38 => 0x1C, // 8
        0x30 => 0x1D, // 0
        0xDD => 0x1E, // ] }
        0x4F => 0x1F, // O
        0x55 => 0x20, // U
        0xDB => 0x21, // [ {
        0x49 => 0x22, // I
        0x50 => 0x23, // P
        0x0D => 0x24, // Enter -> Return
        0x4C => 0x25, // L
        0x4A => 0x26, // J
        0xDE => 0x27, // ' "
        0x4B => 0x28, // K
        0xBA => 0x29, // ; :
        0xDC => 0x2A, // \ |
        0xBC => 0x2B, // , <
        0xBF => 0x2C, // / ?
        0x4E => 0x2D, // N
        0x4D => 0x2E, // M
        0xBE => 0x2F, // . >
        0x09 => 0x30, // Tab
        0x20 => 0x31, // Space
        0xC0 => 0x32, // ` ~
        0x08 => 0x33, // Backspace -> kVK_Delete
        0x1B => 0x35, // Escape
        0x5C => 0x36, // Right Win -> Right Command
        0x5B => 0x37, // Left Win -> Command
        0x10 | 0xA0 => 0x38, // Shift
        0x14 => 0x39, // Caps Lock
        0x12 | 0xA4 => 0x3A, // Alt -> Option
        0x11 | 0xA2 => 0x3B, // Ctrl -> Control
        0xA1 => 0x3C, // Right Shift
        0xA5 => 0x3D, // Right Alt -> Right Option
        0xA3 => 0x3E, // Right Ctrl
        0x2D => 0x72, // Insert -> Help
        0x24 => 0x73, // Home
        0x21 => 0x74, // Page Up
        0x2E => 0x75, // Delete -> Forward Delete
        0x23 => 0x77, // End
        0x22 => 0x79, // Page Down
        0x25 => 0x7B, // Left
        0x27 => 0x7C, // Right
        0x28 => 0x7D, // Down
        0x26 => 0x7E, // Up
        0x70 => 0x7A, // F1
        0x71 => 0x78, // F2
        0x72 => 0x63, // F3
        0x73 => 0x76, // F4
        0x74 => 0x60, // F5
        0x75 => 0x61, // F6
        0x76 => 0x62, // F7
        0x77 => 0x64, // F8
        0x78 => 0x65, // F9
        0x79 => 0x6D, // F10
        0x7A => 0x67, // F11
        0x7B => 0x6F, // F12
        0x7C | 0x2C => 0x69, // F13; Print Screen
        0x7D | 0x91 => 0x6B, // F14; Scroll Lock
        0x7E | 0x13 => 0x71, // F15; Pause
        0x7F => 0x6A, // F16
        0x80 => 0x40, // F17
        0x81 => 0x4F, // F18
        0x82 => 0x50, // F19
        0x83 => 0x5A, // F20
        0x90 => 0x47, // Num Lock -> Keypad Clear
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::mac_keycode;

    #[test]
    fn observed_mismatches_are_translated() {
        // Codes a Mac host previously received untranslated, from a live capture.
        assert_eq!(mac_keycode(0x41), Some(0x00)); // a, was keypad '.'
        assert_eq!(mac_keycode(0x20), Some(0x31)); // space, was U
        assert_eq!(mac_keycode(0x31), Some(0x12)); // 1, was space
        assert_eq!(mac_keycode(0x10), Some(0x38)); // shift, was Y
        assert_eq!(mac_keycode(0x0D), Some(0x24)); // enter, was W
    }

    #[test]
    fn letters_and_digits_map_to_distinct_keys() {
        let mut seen = std::collections::BTreeSet::new();
        for vk in (0x41..=0x5A).chain(0x30..=0x39) {
            let code = mac_keycode(vk).unwrap_or_else(|| panic!("unmapped {vk:#x}"));
            assert!(seen.insert(code), "duplicate kVK {code:#x} for {vk:#x}");
        }
    }

    #[test]
    fn keys_without_mac_counterpart_are_dropped() {
        assert_eq!(mac_keycode(0x5D), None); // Menu
        assert_eq!(mac_keycode(0x87), None); // F24
    }
}
