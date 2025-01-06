use qzmk_drawer::Config;

fn main() -> Result<(), &'static str> {
    const FILENAME: &str = "data/planck-ergol.json";
    const DEST_FILENAME: &str = "planck-ergol.yaml";

    let mut config = Config::new(FILENAME, DEST_FILENAME, 4);
    config.custom.aliases.insert( String::from("LAYOUT_planck_grid"), String::from("LAYOUT_ortho_4x12"));
    config.custom.layer_names = vec!["Qwerty", "ErgoL", "Dvorak", "Lower", "Raise", "Adjust", "NaVim", "Accents"];

    qzmk_drawer::run(config)
}
