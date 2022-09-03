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
        let update = update_gen.generate(&conf);
        write_results(update, &conf);
    }
}

fn write_results(results: Result<String, UpdatesError>, conf: &FileConfig) {
    let path = &conf.get_path();
    match results {
        Ok(s) => fs::write(expand_tilde(&path), s.as_bytes()).unwrap(),
        Err(e) => match e {
            UpdatesError::InvalidBlock(message) => {
                log::error!("{message}")
            }
            UpdatesError::UnableToRead => log::error!("Failed to read file {path}"),
        },
    }
}

#[derive(Debug)]
pub enum UpdatesError {
    UnableToRead,
    InvalidBlock(String),
}

pub struct UpdatesGenerator {
    pub block_generator: BlockGenerator,
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
            let msg = format!(
                "No Themer block with tag '{}'",
                self.block_generator
                    .config
                    .tag
                    .clone()
                    .unwrap_or(String::from("No Tag"))
            );
            return Err(UpdatesError::InvalidBlock(msg));
        }

        Ok(())
    }

    pub fn generate(&mut self, config: &FileConfig) -> Result<String, UpdatesError> {
        let mut contents = self.read_file(&config.get_path())?;

        for block in config.flatten() {
            contents = self.update_block(&mut contents, &block)?;
        }

        Ok(contents)
    }

    fn update_block(
        &mut self,
        contents: &mut String,
        config: &BlockConfig,
    ) -> Result<String, UpdatesError> {
        self.block_generator.config = config.clone();
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
