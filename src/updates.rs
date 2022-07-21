use crate::{
    block::BlockGenerator,
    config::{Config, FileConfig, ThemeVars},
    utils::expand_tilde,
};
use colored::Colorize;
use regex::{Regex, RegexBuilder};
use std::{fs, process::exit};

pub fn run(theme_name: String, config: &Config) {
    let theme = match config.themes.get(&theme_name) {
        Some(t) => t,
        None => {
            log::error!("Theme `{theme_name}` is not listed in configuration file.");
            println!(
                " {} Try to list avaliable themes with `themer themes`",
                "?".blue()
            );
            exit(1);
        }
    };
    let update = UpdatesGenerator::new(theme_name, theme.clone());

    for (_, conf) in &config.files {
        let contents = update.generate(conf);

        fs::write(expand_tilde(&conf.path), contents.as_bytes()).unwrap();
    }
}

pub struct UpdatesGenerator {
    theme_name: String,
    theme: ThemeVars,
}

impl UpdatesGenerator {
    pub fn new(theme_name: String, theme: ThemeVars) -> Self {
        Self { theme_name, theme }
    }

    pub fn generate(&self, config: &FileConfig) -> String {
        let contents = self.get_file_contents(config);

        let blk_gen = BlockGenerator::new(&self.theme_name, &self.theme, config);
        let mut new_block = blk_gen.generate();
        // Replacing dollar sign to avoid Regex issues
        new_block = blk_gen.wrap(&new_block).replace("$", "$$");

        Self::get_block_re(&config.comment)
            .replacen(&contents, 1, new_block)
            .to_string()
    }

    fn get_file_contents(&self, conf: &FileConfig) -> String {
        let contents = match fs::read_to_string(expand_tilde(&conf.path)) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Error reading `{}`:\n{e}", conf.path);
                exit(1);
            }
        };

        let themer_block_re = Self::get_block_re(&conf.comment);
        if !themer_block_re.is_match(&contents) {
            log::error!("Failed to find THEMER block inside of `{}`", conf.path);
            exit(1);
        }

        contents
    }

    pub fn get_block_re(comment: &String) -> Regex {
        RegexBuilder::new(&format!("{0} THEMER\n.*{0} THEMER_END", comment))
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    }
}
