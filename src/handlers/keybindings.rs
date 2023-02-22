use smithay::input::keyboard::xkb::ffi::xkb_keysym_from_name;
use smithay::input::keyboard::xkb::ffi::XKB_KEYSYM_CASE_INSENSITIVE;
use smithay::input::keyboard::xkb::keysym_to_utf8;
use smithay::input::keyboard::xkb::keysyms;
use std::sync::Arc;
use std::{collections::HashMap, process::Command};

// code to convert emacs style keybindings to xkb keysyms
pub fn emacs_to_xkb(key: &str) -> (String, HashMap<&'static str, bool>) {
    // TODO: add more keys
    // FIXME: this is a hack, we should use a proper parser
    // for now, we just assume that the key is a single character
    // this will only support english keyboards, but that's fine for now, and simple function keys such as ctrl, alt, etc.
    let mut key = key.to_string();
    let mut modifiers = HashMap::new();
    match key.chars().nth(0) {
        Some('C') => {
            modifiers.insert("Control", true);
            modifiers.insert("Alt", false);
            modifiers.insert("Shift", false);
            key.remove(0);
        }
        Some('M') => {
            modifiers.insert("Alt", true);
            modifiers.insert("Control", false);
            modifiers.insert("Shift", false);
            key.remove(0);
        }
        Some('S') => {
            modifiers.insert("Shift", true);
            modifiers.insert("Control", false);
            modifiers.insert("Alt", false);
            key.remove(0);
        }
        _ => {
            println!("Unknown modifier: {}", key);
        }
    }

    // strip the modifier from the key
    key.remove(0);

    // someday i will come back to this and make it better
    // ... someday
    // SOMEDAY

    return (key, modifiers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emacs_to_xkb() {
        // this will test "C-x", if Ctrl is true, then other modifiers should be false
        let key = "C-x";
        let (key, modifiers) = emacs_to_xkb(key);
        assert_eq!(key, "x");
        assert_eq!(modifiers.get("Control"), Some(&true));
        assert_eq!(modifiers.get("Alt"), Some(&false));
        assert_eq!(modifiers.get("Shift"), Some(&false));
    }
}
