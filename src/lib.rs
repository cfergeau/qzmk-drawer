use keymap::Key;
use std::fs::File;
use std::io::prelude::*;

pub mod keymap;
pub mod qmk;

pub struct Keymap {
    keymap: String,
    keyboard: String,
    layout: String,
    layers: Vec<Vec<Key>>,
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
            yaml.push_str(&keymap::layer::name(i));
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
}

pub struct Config {
    src_json: String,
    dest_yaml: String,
}

impl Config {
    pub fn new(src_json: &str, dest_yaml: &str) -> Config {
        Config {
            src_json: String::from(src_json),
            dest_yaml: String::from(dest_yaml),
        }
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let keymap = qmk::parser::keymap_from_file(&config.src_json);
    if keymap.is_err() {
        return Err("could not read keymap from file");
    }
    let mut keymap = keymap.unwrap();
    //println!("keyboard: {}", keymap.keyboard);
    //println!("keymap: {}", keymap.keymap);
    //println!("layout: {}", keymap.layout);
    if keymap.layout == "LAYOUT_planck_grid" {
        keymap.layout = "LAYOUT_ortho_4x12".to_string();
    }
    //keymap.to_yaml().expect("conversion to yaml failed");
    keymap.to_file(&config.dest_yaml)
}
