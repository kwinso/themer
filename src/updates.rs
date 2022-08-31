use crate::{
    block::BlockGenerator,
    config::{BlockConfig, Config, FileConfig},
    utils::expand_tilde,
};
use colored::Colorize;
use std::{fs, process::exit};

pub fn run(theme_name: String, config: &Config) {
    let vars = match config.themes.get(&theme_name) {
        Some(t) => t.clone(),
        None => {
            log::error!("Theme `{theme_name}` is not listed in configuration file.");
            println!(
                " {} Try to list avaliable themes with `themer themes`",
                "?".blue()
            );
            exit(1);
        }
    };

    let block_gen = BlockGenerator::new(
        theme_name,
        &vars,
        FileConfig::Single(BlockConfig::default()),
    );
    let mut update_gen = UpdatesGenerator::new(block_gen);

    for (_, conf) in &config.files {
        let mut results: Result<String, UpdatesError> = Ok(String::new());
        let mut tag = None;
        let mut path = String::new();
        match conf {
            FileConfig::Multi(mutli) => mutli.clone().blocks.into_iter().for_each(|(t, c)| {
                let config = BlockConfig {
                    path: mutli.path.clone(),
                    comment: mutli.comment.clone(),
                    block: c,
                };

                path = mutli.path.clone();
                tag = Some(t);
                results = update_gen.generate(&config, &tag);
            }),
            FileConfig::Single(config) => {
                path = config.path.clone();
                tag = None;
                results = update_gen.generate(config, &tag);
            }
        }
        match results {
            Ok(s) => fs::write(expand_tilde(&path), s.as_bytes()).unwrap(),
            Err(e) => match e {
                UpdatesError::NoBlock => log::error!(
                    "Failed to find Themer block (tag: {}) inside {path}",
                    tag.unwrap_or("No tag".to_string())
                ),
                UpdatesError::UnableToRead => log::error!("Failed to read file {path}"),
            },
        }
    }
}

pub struct UpdatesGenerator {
    pub block_generator: BlockGenerator,
}

pub enum UpdatesError {
    UnableToRead,
    NoBlock,
}

impl UpdatesGenerator {
    pub fn new(gen: BlockGenerator) -> Self {
        Self {
            block_generator: gen,
        }
    }

    pub fn read_file(&self, path: &String) -> Result<String, UpdatesError> {
        match fs::read_to_string(expand_tilde(&path)) {
            Ok(f) => Ok(f),
            Err(_) => {
                return Err(UpdatesError::UnableToRead);
            }
        }
    }

    pub fn validate_block(&self, contents: &String) -> Result<(), UpdatesError> {
        if !self.block_generator.get_re().is_match(contents) {
            return Err(UpdatesError::NoBlock);
        }

        Ok(())
    }

    pub fn generate(
        &mut self,
        config: &BlockConfig,
        tag: &Option<String>,
    ) -> Result<String, UpdatesError> {
        self.block_generator.set_tag(tag.clone());

        let contents = self.read_file(&config.path)?;

        self.validate_block(&contents)?;

        let mut update = self.block_generator.generate();
        // Replacing dollar sign to avoid Regex issues
        update = self.block_generator.wrap(&update).replace("$", "$$");

        Ok(self
            .block_generator
            .get_re()
            .replacen(&contents, 1, update)
            .to_string())
    }
}
