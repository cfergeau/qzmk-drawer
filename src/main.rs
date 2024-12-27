use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

static LAYERS: [&str; 8] = [
    "Qwerty", "ErgoL", "Dvorak", "Lower", "Raise", "Adjust", "NaVim", "Accents",
];

enum Key {
    No,
    Trans,
    BasicKey(String),
    SpecialKey(String),
    Unicode(String, Option<String>),
    LayerTap { layer: String, key: String },
    ModTap { modifier: String, key: String },
    ModKey { modifier: String, key: String },
    LayerChange { _action: String, layer: String },
    Unknown(String),
}

impl Key {
    fn parse_kc(key_str: &str) -> Option<Self> {
        match key_str.strip_prefix("KC_") {
            Some(key) => Some(Key::BasicKey(key.to_string())),
            None => None,
        }
    }

    fn parse_special(key_str: &str) -> Option<Self> {
        lazy_static! {
            static ref SPECIAL: Regex = Regex::new(r"^[A-Z]{2}(_[A-Z0-9]+)+$").unwrap();
        }
        if SPECIAL.is_match(key_str) {
            return Some(Key::SpecialKey(key_str.to_string()));
        }
        None
    }

    fn parse_unicode(key_str: &str) -> Option<Self> {
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

    fn parse_layertap(key_str: &str) -> Option<Self> {
        lazy_static! {
            static ref LT: Regex = Regex::new(r"^LT\(([\w_]+), *([\w_]+)\)$").unwrap();
        }
        if let Some(lt) = LT.captures(key_str) {
            if let Some(layer) = Key::parse_layer_name(&lt[1]) {
                return Some(Key::LayerTap {
                    layer: layer.to_string(),
                    key: lt[2].to_string(),
                });
            }
        }
        None
    }

    fn parse_modtap(key_str: &str) -> Option<Self> {
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

    fn parse_layer_change(key_str: &str) -> Option<Self> {
        lazy_static! {
            static ref LAYER_CHANGE: Regex = Regex::new(r"^(MO|PDF)\(([A-Z_]+)\)$").unwrap();
        }
        if let Some(lc) = LAYER_CHANGE.captures(key_str) {
            if let Some(layer) = Key::parse_layer_name(&lc[2]) {
                return Some(Key::LayerChange {
                    _action: lc[1].to_string(),
                    layer: layer.to_string(),
                });
            }
        }
        None
    }

    fn parse_layer_name(layer_str: &str) -> Option<&str> {
        for layer in LAYERS {
            if layer.to_ascii_lowercase() == layer_str.to_ascii_lowercase() {
                return Some(layer);
            }

            let ulayer = format!("_{layer}");
            if ulayer.to_ascii_lowercase() == layer_str.to_ascii_lowercase() {
                return Some(layer);
            }
        }
        None
    }

    fn layer_name(idx: usize) -> String {
        if let Some(layer) = LAYERS.get(idx) {
            layer.to_string()
        } else {
            format!("layer{idx}")
        }
    }

    // This method needs work:
    // - the parsed modifier needs to be validated against a list of known modifiers
    //   https://github.com/qmk/qmk_firmware/blob/master/docs/feature_advanced_keycodes.modifier
    //   otherwise it's going to be prone to false positives
    // - this does not handle nested modifiers such as `LCTL(LALT(KC_DEL))`
    fn parse_modkey(key_str: &str) -> Option<Self> {
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

    fn from_str(key_str: &str) -> Option<Self> {
        if key_str == "KC_TRNS" {
            return Some(Key::Trans);
        }
        if key_str == "KC_NO" {
            return Some(Key::No);
        }
        if let Some(key) = Key::parse_kc(key_str) {
            return Some(key);
        }
        if let Some(key) = Key::parse_special(key_str) {
            return Some(key);
        }
        if let Some(key) = Key::parse_unicode(key_str) {
            return Some(key);
        }
        if let Some(key) = Key::parse_layertap(key_str) {
            return Some(key);
        }
        if let Some(key) = Key::parse_modtap(key_str) {
            return Some(key);
        }
        if let Some(key) = Key::parse_layer_change(key_str) {
            return Some(key);
        }
        // Keep this last as this is prone to false positives
        if let Some(key) = Key::parse_modkey(key_str) {
            return Some(key);
        }
        println!("unknown key {key_str}");
        Some(Key::Unknown(key_str.to_string()))
        //None
    }

    // KC_TRNS "{t: ▽, type: trans}"
    // KC_NO ""
    fn to_yaml(&self) -> String {
        match self {
            Key::BasicKey(key) => key.to_string(),
            Key::SpecialKey(key) => key.to_string(),
            Key::No => "".to_string(),
            Key::Trans => "{t: ▽, type: trans}".to_string(),
            Key::Unicode(uc, None) => format!("UC({uc})"),
            Key::Unicode(lower, Some(upper)) => format!("UP({lower}, {upper})"),
            Key::LayerTap { layer, key } => format!("{{t: {key}, h: {layer}}}"),
            Key::ModTap { modifier, key } => format!("{{t: {key}, h: {modifier}}}"),
            Key::ModKey { modifier, key } => format!("{modifier}({key})"),
            Key::LayerChange { _action: _, layer } => layer.to_string(),
            Key::Unknown(key) => {
                println!("unknown key {key}");
                key.to_string()
            }
            //_ => panic!("missing to_yaml implementation"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Keymap {
    keymap: String,
    keyboard: String,
    layout: String,
    layers: Vec<Vec<String>>,
}

// layout:
//   ortho_layout:
//       split: false
//       rows: 4
//       columns: 12
//       thumbs: MIT  # also try "2x2u" with one fewer key on the last row
impl Keymap {
    fn to_yaml(&self) -> Result<String, &'static str> {
        let mut yaml = String::from("layout:");
        yaml.push(' ');
        // {qmk_keyboard: planck/rev7, layout_name: LAYOUT_ortho_4x12}
        yaml.push_str(
            format!(
                "{{qmk_keyboard: {}, layout_name: {}}}",
                self.keyboard, self.layout
            )
            .as_str(),
        );
        yaml.push('\n');
        yaml.push_str("layers:");
        yaml.push('\n');

        let mut i = 0;
        for layer in &self.layers {
            yaml.push_str("  ");
            yaml.push_str(&Key::layer_name(i));
            yaml.push('\n');
            //println!("layer #{i}: {} elements: {:?}", layer.len(), layer);
            for keycode in layer {
                // transform keycode
                // enum {
                // UnicodeSingle,
                // UnicodeCase,
                // LayerTap,
                // ModTap,
                // }
                // UG_*, UM(), UP(), LT()
                if let Some(key) = Key::from_str(keycode) {
                    yaml.push_str("  ");
                    yaml.push_str("- ");
                    yaml.push_str(&key.to_yaml());
                    yaml.push('\n');
                }
            }
            i += 1;
        }
        println!("{}", yaml);
        return Ok(yaml);
        //Err("unimplemented")
    }

    fn to_file(&self, path: &str) -> Result<(), &'static str> {
        let mut file = match File::create(&path) {
            Err(_) => return Err("couldn't create file"),
            Ok(f) => f,
        };

        let yaml = self.to_yaml()?;
        match file.write_all(yaml.as_bytes()) {
            Err(_) => Err("couldn't write to file"),
            Ok(_) => Ok(()),
        }
    }

    fn from_file(filename: &str) -> Result<Self, &'static str> {
        let data = match fs::read_to_string(filename) {
            Ok(data) => data,
            // most likely requires to specify a lifetime, which I haven't learnt yet
            // Err(_) => Err(format!("Unable to read file {}", filename).as_str()),
            Err(_) => return Err("Unable to read file"),
        };

        match serde_json::from_str::<Keymap>(&data) {
            Ok(keymap) => Ok(keymap),
            // Err(_) => Err(format!("Unable to parse file {}", filename)),
            Err(_) => Err("Unable to parse file"),
        }
    }
}

fn main() {
    const FILENAME: &str = "data/planck-ergol.json";
    const DEST_FILENAME: &str = "planck-ergol.yaml";

    let mut keymap = Keymap::from_file(FILENAME).expect("could not read keymap from file");
    //println!("keyboard: {}", keymap.keyboard);
    //println!("keymap: {}", keymap.keymap);
    //println!("layout: {}", keymap.layout);
    if keymap.layout == "LAYOUT_planck_grid" {
        keymap.layout = "LAYOUT_ortho_4x12".to_string();
    }
    //keymap.to_yaml().expect("conversion to yaml failed");
    keymap
        .to_file(DEST_FILENAME)
        .expect("writing yaml to file failed");
}
