use crate::keymap::layer;
use crate::keymap::Key;
use crate::keymap::Keymap;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct RawKeymap {
    keymap: String,
    keyboard: String,
    layout: String,
    layers: Vec<Vec<String>>,
}

impl RawKeymap {
    fn to_keymap(self) -> Result<Keymap, &'static str> {
        let mut layers: Vec<Vec<Key>> = Vec::new();
        for layer in &self.layers {
            let mut keys: Vec<Key> = Vec::new();
            for keycode in layer {
                match from_str(keycode) {
                    Some(key) => keys.push(key),
                    None => return Err("failed to parse key"),
                }
            }
            layers.push(keys);
        }

        Ok(Keymap {
            keymap: self.keymap,
            keyboard: self.keyboard,
            layout: self.layout,
            layers,
        })
    }
}

pub fn keymap_from_file(filename: &str) -> Result<Keymap, &'static str> {
    let data = match fs::read_to_string(filename) {
        Ok(data) => data,
        // most likely requires to specify a lifetime, which I haven't learnt yet
        // Err(_) => Err(format!("Unable to read file {}", filename).as_str()),
        Err(_) => return Err("Unable to read file"),
    };

    match serde_json::from_str::<RawKeymap>(&data) {
        Ok(raw_keymap) => raw_keymap.to_keymap(),
        // Err(_) => Err(format!("Unable to parse file {}", filename)),
        Err(_) => Err("Unable to parse file"),
    }
}

fn parse_kc(key_str: &str) -> Option<Key> {
    match key_str.strip_prefix("KC_") {
        Some(key) => Some(Key::BasicKey(key.to_string())),
        None => None,
    }
}

fn parse_special(key_str: &str) -> Option<Key> {
    lazy_static! {
        static ref SPECIAL: Regex = Regex::new(r"^[A-Z]{2}(_[A-Z0-9]+)+$").unwrap();
    }
    if SPECIAL.is_match(key_str) {
        return Some(Key::SpecialKey(key_str.to_string()));
    }
    None
}

fn parse_unicode(key_str: &str) -> Option<Key> {
    // UM(xx)
    // UP(lower, upper)
    lazy_static! {
        static ref UM: Regex = Regex::new(r"^UM\(([\w_]+)\)$").unwrap();
        static ref UP: Regex = Regex::new(r"^UP\(([\w_]+), *([\w_]+)\)$").unwrap();
    }
    if let Some(um) = UM.captures(key_str) {
        return Some(Key::Unicode(um[1].to_string(), None));
    }
    if let Some(up) = UP.captures(key_str) {
        return Some(Key::Unicode(up[1].to_string(), Some(up[2].to_string())));
    }
    None
}

fn parse_layertap(key_str: &str) -> Option<Key> {
    lazy_static! {
        static ref LT: Regex = Regex::new(r"^LT\(([\w_]+), *([\w_]+)\)$").unwrap();
    }
    if let Some(lt) = LT.captures(key_str) {
        if let Some(layer) = layer::parse(&lt[1]) {
            return Some(Key::LayerTap {
                layer: layer.to_string(),
                key: lt[2].to_string(),
            });
        }
    }
    None
}

fn parse_modtap(key_str: &str) -> Option<Key> {
    lazy_static! {
        static ref MT: Regex = Regex::new(r"^([A-Z_]+)_T\(KC_([A-Z]+)\)$").unwrap();
    }
    if let Some(mt) = MT.captures(key_str) {
        return Some(Key::ModTap {
            modifier: mt[1].to_string(),
            key: mt[2].to_string(),
        });
    }
    None
}

fn parse_layer_change(key_str: &str) -> Option<Key> {
    lazy_static! {
        static ref LAYER_CHANGE: Regex = Regex::new(r"^(MO|PDF)\(([A-Z_]+)\)$").unwrap();
    }
    if let Some(lc) = LAYER_CHANGE.captures(key_str) {
        if let Some(layer) = layer::parse(&lc[2]) {
            return Some(Key::LayerChange {
                _action: lc[1].to_string(),
                layer: layer.to_string(),
            });
        }
    }
    None
}

// This method needs work:
// - the parsed modifier needs to be validated against a list of known modifiers
//   https://github.com/qmk/qmk_firmware/blob/master/docs/feature_advanced_keycodes.modifier
//   otherwise it's going to be prone to false positives
// - this does not handle nested modifiers such as `LCTL(LALT(KC_DEL))`
fn parse_modkey(key_str: &str) -> Option<Key> {
    lazy_static! {
        static ref MK: Regex = Regex::new(r"^([A-Z_]+)\(KC_([A-Z]+)\)$").unwrap();
    }
    if let Some(mt) = MK.captures(key_str) {
        return Some(Key::ModKey {
            modifier: mt[1].to_string(),
            key: mt[2].to_string(),
        });
    }
    None
}

pub fn from_str(key_str: &str) -> Option<Key> {
    if key_str == "KC_TRNS" {
        return Some(Key::Trans);
    }
    if key_str == "KC_NO" {
        return Some(Key::No);
    }
    if let Some(key) = parse_kc(key_str) {
        return Some(key);
    }
    if let Some(key) = parse_special(key_str) {
        return Some(key);
    }
    if let Some(key) = parse_unicode(key_str) {
        return Some(key);
    }
    if let Some(key) = parse_layertap(key_str) {
        return Some(key);
    }
    if let Some(key) = parse_modtap(key_str) {
        return Some(key);
    }
    if let Some(key) = parse_layer_change(key_str) {
        return Some(key);
    }
    // Keep this last as this is prone to false positives
    if let Some(key) = parse_modkey(key_str) {
        return Some(key);
    }
    println!("unknown key {key_str}");
    Some(Key::Unknown(key_str.to_string()))
    //None
}
