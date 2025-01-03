use qzmk_drawer::Config;

fn main() -> Result<(), &'static str> {
    const FILENAME: &str = "data/planck-ergol.json";
    const DEST_FILENAME: &str = "planck-ergol.yaml";

    let config = Config::new(FILENAME, DEST_FILENAME);

    qzmk_drawer::run(config)
}
