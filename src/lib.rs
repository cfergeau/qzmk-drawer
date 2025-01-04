pub mod keymap;
pub mod qmk;

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
