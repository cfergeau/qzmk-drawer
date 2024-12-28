use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub mod keymap;
pub mod qmk;

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
            yaml.push_str(&qmk::parser::layer_name(i));
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
                if let Some(key) = qmk::parser::from_str(keycode) {
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
