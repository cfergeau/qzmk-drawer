use std::path::PathBuf;

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
            _default => Err("unknown command"),
        }
        None => Err("missing command"),
    }
}
