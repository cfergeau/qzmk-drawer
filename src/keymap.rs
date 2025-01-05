use std::fs::File;
use std::io::prelude::*;

pub mod layer;

pub enum Key {
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
    // KC_TRNS "{t: ▽, type: trans}"
    // KC_NO ""
    pub fn to_yaml(&self) -> String {
        match self {
            Key::BasicKey(key) => key.to_string(),
            Key::SpecialKey(key) => key.to_string(),
            Key::No => "null".to_string(),
            Key::Trans => "{t: ▽, type: trans}".to_string(),
            Key::Unicode(uc, None) => format!("UM({uc})"),
            Key::Unicode(lower, Some(upper)) => format!("'UP({lower}, {upper})'"),
            Key::LayerTap { layer, key } => format!("{{t: {key}, h: {layer}}}"),
            Key::ModTap { modifier, key } => format!("{{t: {key}, h: {modifier}}}"),
            Key::ModKey { modifier, key } => format!("{modifier}({key})"),
            Key::LayerChange { _action: _, layer } => layer.to_string(),
            Key::Unknown(key) => {
                println!("unknown key {key}");
                key.to_string()
            } //_ => panic!("missing to_yaml implementation"),
        }
    }
}

pub struct Keymap {
    pub keymap: String,
    pub keyboard: String,
    pub layout: String,
    pub num_rows: usize,
    pub num_columns: usize,
    pub layers: Vec<Vec<Key>>,
}

// layout:
//   ortho_layout:
//       split: false
//       rows: 4
//       columns: 12
//       thumbs: MIT  # also try "2x2u" with one fewer key on the last row
impl Keymap {
    pub fn to_yaml(&self) -> Result<String, &'static str> {
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
            yaml.push_str(&layer::name(i));
            yaml.push('\n');
            //println!("layer #{i}: {} elements: {:?}", layer.len(), layer);
            for key in layer {
                // transform keycode
                // enum {
                // UnicodeSingle,
                // UnicodeCase,
                // LayerTap,
                // ModTap,
                // }
                // UG_*, UM(), UP(), LT()
                yaml.push_str("  ");
                yaml.push_str("- ");
                yaml.push_str(&key.to_yaml());
                yaml.push('\n');
            }
            i += 1;
        }
        println!("{}", yaml);
        return Ok(yaml);
        //Err("unimplemented")
    }

    pub fn to_file(&self, path: &str) -> Result<(), &'static str> {
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
}
