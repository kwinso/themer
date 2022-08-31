use crate::{
    block::BlockGenerator,
    config::{BlockConfig, Config, FileConfig, ThemeVars},
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
    let update_gen = UpdatesGenerator::new(theme_name, theme.clone());

    for (_, conf) in &config.files {
        match conf {
            FileConfig::Multi(mutli) => mutli.blocks.clone().into_iter().for_each(|(tag, c)| {
                let config = BlockConfig {
                    path: mutli.path.clone(),
                    comment: mutli.comment.clone(),
                    block: c,
                };
                write_update(&mutli.path, update_gen.generate(&config, Some(tag)));
            }),
            FileConfig::Single(config) => {
                write_update(&config.path, update_gen.generate(config, None));
            }
        }
    }
}

fn write_update(path: &String, update: Option<String>) {
    if let Some(update) = update {
        fs::write(expand_tilde(&path), update.as_bytes()).unwrap();
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

    pub fn generate(&self, config: &BlockConfig, tag: Option<String>) -> Option<String> {
        let blk_gen = BlockGenerator::new(
            &self.theme_name,
            &self.theme,
            FileConfig::Single(config.clone()),
        );
        let tags = BlockGenerator::get_block_tags(&tag);

        let contents = self.get_file_contents(config, &tags);
        if let Err(_) = contents {
            return None;
        }
        let contents = contents.unwrap();

        let mut new_block = blk_gen.generate();
        // Replacing dollar sign to avoid Regex issues
        new_block = blk_gen.wrap(&new_block, &tags).replace("$", "$$");

        Some(
            Self::get_block_re(&config.comment, &tags)
                .replacen(&contents, 1, new_block)
                .to_string(),
        )
    }

    fn get_file_contents(&self, conf: &BlockConfig, tags: &(String, String)) -> Result<String, ()> {
        let contents = match fs::read_to_string(expand_tilde(&conf.path)) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Error reading `{}`:\n{e}", conf.path);
                return Err(());
            }
        };

        let themer_block_re = Self::get_block_re(&conf.comment, tags);
        if !themer_block_re.is_match(&contents) {
            log::warn!(
                "Failed to find THEMER block inside of `{}`. Skipping it.",
                conf.path
            );
            return Err(());
        }

        Ok(contents)
    }

    pub fn get_block_re(comment: &String, (start, end): &(String, String)) -> Regex {
        log::debug!("Generate regex for block `{comment} {start} ... {comment} {end}");

        RegexBuilder::new(&format!("{0} {start}\n.*{0} {end}", comment))
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    }
}
