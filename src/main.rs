mod block;
mod config;
mod updates;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;
use config::Config;
use log;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use std::process::Command;
use std::{fs, process::exit};
use utils::expand_tilde;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[clap(
        global = true,
        short,
        long,
        default_value = "~/.config/themer/config.yml"
    )]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List avaliable themes in config file
    Themes,
    /// List avaliable files in config file
    Files {
        /// Check if config files are valid to be process by Themer
        #[clap(parse(from_flag), long)]
        check: bool,
    },
    /// Set new theme for all of your configuration files
    Set {
        /// Theme name to set
        #[clap(required = true, value_parser)]
        theme: String,
    },
}

fn setup_logger() {
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Info;

    let log_conf = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(level, log_conf, TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}

fn main() {
    setup_logger();

    let args = Args::parse();

    let config = match fs::read_to_string(expand_tilde(&args.config)) {
        Ok(c) => c,
        Err(_) => {
            log::error!(
                "Failed to read Themer configuration file in '{}'",
                args.config
            );
            exit(1);
        }
    };

    let config: Config = match serde_yaml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to parse configuration file:\n\t{e}");
            exit(1)
        }
    };
    log::debug!("{config:#?}");

    let command = args.command.unwrap_or(Commands::Themes);

    match command {
        Commands::Themes => {
            println!("{}", "Avaliable themes:".purple());
            config
                .themes
                .into_iter()
                .for_each(|x| println!("  - {}", x.0));
        }
        Commands::Files { check } => {
            utils::list_files(config, check);
        }
        Commands::Set { theme } => {
            updates::run(theme, &config);
            if let Some(reload_cmd) = config.reload {
                println!("{}", "Running reload command...".blue());

                let mut cmd = Command::new("sh");
                match cmd.args(["-c", &reload_cmd]).output() {
                    Ok(output) => {
                        if output.status.success() {
                            println!("{}", "Environment succsessfully reloaded!".green());
                        } else {
                            log::error!("Unsuccessfull outcome of reload command:");
                            println!("\t{}", output.status);
                        }
                    }
                    Err(_) => log::error!("Failed to run reload command"),
                }
            } else {
                println!(
                    "{}\n {} To see updates, you may need to reload your environment.",
                    "Theme succsessfully updated".green(),
                    "?".blue()
                );
            }
        }
    };
}
