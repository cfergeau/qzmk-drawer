static LAYERS: [&str; 8] = [
    "Qwerty", "ErgoL", "Dvorak", "Lower", "Raise", "Adjust", "NaVim", "Accents",
];

pub fn parse(layer_str: &str) -> Option<&str> {
    for layer in LAYERS {
        if layer.eq_ignore_ascii_case(layer_str) {
            return Some(layer);
        }

        let ulayer = format!("_{layer}");
        if ulayer.eq_ignore_ascii_case(layer_str) {
            return Some(layer);
        }
    }
    None
}

pub fn name(idx: usize) -> String {
    if let Some(layer) = LAYERS.get(idx) {
        layer.to_string()
    } else {
        format!("layer{idx}")
    }
}
