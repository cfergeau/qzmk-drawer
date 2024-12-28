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
            } //_ => panic!("missing to_yaml implementation"),
        }
    }
}
