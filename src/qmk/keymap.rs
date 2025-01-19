use serde::{Deserialize, Serialize};
use crate::Keymap;

#[derive(Serialize, Deserialize)]
struct RawKeymap {
    keymap: String,
    keyboard: String,
    layout: String,
    layers: Vec<Vec<String>>,
}

    fn keymap_from_file(filename: &str) -> Result<Keymap, &'static str> {
        let data = match fs::read_to_string(filename) {
            Ok(data) => data,
            // most likely requires to specify a lifetime, which I haven't learnt yet
            // Err(_) => Err(format!("Unable to read file {}", filename).as_str()),
            Err(_) => return Err("Unable to read file"),
        };

        match serde_json::from_str::<RawKeymap>(&data) {
            Ok(keymap) => Ok(keymap),
            // Err(_) => Err(format!("Unable to parse file {}", filename)),
            Err(_) => Err("Unable to parse file"),
        }
    }
