use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Keymap {
    keymap: String,
    keyboard: String,
    layout: String,
    layers: Vec<Vec<String>>,
}

fn main() {
    const FILENAME: &str = "data/planck-ergol.json";

    let data =
        fs::read_to_string(FILENAME).expect(format!("Unable to read file {}", FILENAME).as_str());

    let keymap :Keymap = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("keyboard: {}", keymap.keyboard);
    println!("keymap: {}", keymap.keymap);
    println!("layout: {}", keymap.layout);
    let mut i = 0;
    for layer in keymap.layers {
        println!("layer #{i}: {} elements: {:?}", layer.len(), layer);
        i+=1;
    }
}
