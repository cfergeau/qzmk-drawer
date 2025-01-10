use std::collections::HashMap;
use std::path::PathBuf;
use crate::keymap::layer::LayerNames;

pub mod keymap;
pub mod qmk;

// This struct stores keyboard specific knowlegde which is not available from QMK
// JSON. These parameters are currently hardcoded in the source code, this struct
// allows to define all of them in a single place.
pub struct CustomConfig {
    num_rows: usize,
    // FIXME: not sure how to use this is layers.rs yet
    pub aliases: HashMap<String, String>,
    pub layer_names: Vec<&'static str>,
}

pub struct Config {
    src_json: PathBuf,
    dest_yaml: PathBuf,
    pub custom: CustomConfig,
}

impl Config {
    pub fn new(src_json: PathBuf, dest_yaml: PathBuf, num_rows: usize) -> Config {
        Config {
            src_json,
            dest_yaml,
            custom: CustomConfig {
                num_rows,
                aliases: HashMap::new(),
                layer_names: Vec::new(),
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    // FIXME: might make more sense to not split the keys per rows during parsing, but to only do
    // it during rendering
    let layer_names = LayerNames::new(config.custom.layer_names.iter().map(|x| x.to_string()).collect());
    let keymap = qmk::parser::keymap_from_file(&config.src_json, config.custom.num_rows, &layer_names);
    if keymap.is_err() {
        return Err("could not read keymap from file");
    }
    let mut keymap = keymap.unwrap();
    //println!("keyboard: {}", keymap.keyboard);
    //println!("keymap: {}", keymap.keymap);
    //println!("layout: {}", keymap.layout);
    if let Some(alias) = config.custom.aliases.get(&keymap.layout) {
        keymap.layout = String::from(alias);
    }
    //keymap.to_yaml().expect("conversion to yaml failed");
    keymap.to_file(&config.dest_yaml, &layer_names)
}
