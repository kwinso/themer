mod config;

use clap::Parser;
use config::Config;
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "~/.config/themer.yml")]
    config: String,
}

// TODO: Add & setup logger for pretty messages
// TODO: Setup subcommands: themes, files (to list respectively), set (to set theme)
fn main() {
    let args = Args::parse();
    let config = match fs::read_to_string(args.config) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Failed to read Themer configuration file.");
            exit(1);
        }
    };

    let config: Config = match serde_yaml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse configuration file:\n{e}");
            exit(1)
        }
    };

    println!("Avaliable themes: ");
    config
        .themes
        .into_iter()
        .for_each(|x| println!("{}: {:#?}", x.0, x.1));
}
