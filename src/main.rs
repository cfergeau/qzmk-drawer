use std::path::PathBuf;
use qzmk_drawer::qmk::constants;

use clap::{Args, Parser, Subcommand};

use qzmk_drawer::Config;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Draw(DrawArgs),
    ParseQmkKeycodes(ParseQmkKeycodesArgs),
}

#[derive(Args)]
struct DrawArgs {
        /// source QMK JSON file
        #[arg(short, long, value_name = "FILE")]
        qmk_json: Option<PathBuf>,

        /// destination YAML file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
}

#[derive(Args)]
struct ParseQmkKeycodesArgs {
        /// source QMK JSON file
        #[arg(short, long, value_name = "FILE")]
        qmk_src_path: Option<PathBuf>,
        keycodes_filename: Option<PathBuf>,
}

fn draw(args: DrawArgs) -> Result<(), &'static str> {
    const FILENAME: &str = "data/planck-ergol.json";
    const DEST_FILENAME: &str = "planck-ergol.yaml";
    let qmk_json = args.qmk_json.unwrap_or(PathBuf::from(FILENAME));
    let output = args.output.unwrap_or(PathBuf::from(DEST_FILENAME));

    // hardcoded values for my keyboard, at some point they should come from
    // command parameters, or some extra json files
    let mut config = Config::new(qmk_json, output, 4);
    config.custom.aliases.insert( String::from("LAYOUT_planck_grid"), String::from("LAYOUT_ortho_4x12"));
    config.custom.layer_names = vec!["Qwerty", "ErgoL", "Dvorak", "Lower", "Raise", "Adjust", "NaVim", "Accents"];

    qzmk_drawer::run(config)
}

fn qmk_keycodes_path(base_path: &PathBuf, keycode_type: &str, version: &str) -> PathBuf {
    let keycodes_filename = format!("keycodes_{version}_{keycode_type}.hjson");

    base_path.join("data").join("constants").join("keycodes").join(keycodes_filename)
}

fn parse_qmk_keycodes(args: ParseQmkKeycodesArgs) -> Result<(), &'static str> {
    const QMK_SRC_PATH: &str = "/Users/teuf/perso/qzmk/qmk_firmware";
    let qmk_src_path = args.qmk_src_path.unwrap_or(PathBuf::from(QMK_SRC_PATH));

    const VERSION: &str = "0.0.1";
    const TYPE: &str = "basic";

    let qmk_src_path = match args.keycodes_filename {
        Some(filename) => qmk_src_path.join("data").join("constants").join("keycodes").join(filename),
        None => qmk_keycodes_path(&qmk_src_path, TYPE, VERSION),
    };
    println!("{}", qmk_src_path.display());
    /*
    match constants::parse(&qmk_src_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
*/
    /*
    let qmk_src_path = PathBuf::from("/Users/teuf/dev/qzmk-drawer/data/keycodes_0.0.6_quantum.hjson");
    println!("{}", qmk_src_path.display());
*/
    //constants::parse(&qmk_src_path).map(|_| ())
    let files = constants::gen_file_list(&PathBuf::from(QMK_SRC_PATH), &[ "0.0.6", "0.0.5", "0.0.4", "0.0.3", "0.0.2", "0.0.1" ]);
    for (key, value) in files.iter() {
        println!("{key}");
        for file in value {
            println!("    {}", file.to_string_lossy())
        }

    }
    constants::parse_categories(&files);
    if let Ok(k) = constants::parse(&qmk_src_path) {
        k.print();
    }

    Ok(())
}

fn main() -> Result<(), &'static str> {

    let cli = Cli::parse();
     match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match cli.command{
        Some(cmd) => match cmd {
            Commands::Draw(args) => draw(args),
            Commands::ParseQmkKeycodes(args) => parse_qmk_keycodes(args),
            //_default => Err("unknown command"),
        }
        None => Err("missing command"),
    }
}
