pub struct LayerNames {
    names: Vec<String>,
}

impl LayerNames {
    pub fn new(names: Vec<String>) -> LayerNames {
        LayerNames {
            names,
        }
    }

    pub fn pretty_name(&self, name: &str) -> Option<String> {
        for layer in &self.names {
            if layer.eq_ignore_ascii_case(name) {
                return Some(String::from(layer));
            }

            let ulayer = format!("_{layer}");
            if ulayer.eq_ignore_ascii_case(name) {
                return Some(String::from(layer));
            }
        }
        None
    }

    pub fn get(&self, idx: usize) -> String {
        if let Some(name) = self.names.get(idx) {
            name.to_string()
        } else {
            format!("layer{idx}")
        }
    }
}
