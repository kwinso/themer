use crate::block::BlockGenerator;
use crate::config::BlockConfig;
use crate::config::Config;
use crate::config::FileConfig;
use crate::config::TaggedConfig;
use crate::updates::UpdatesGenerator;
use colored::Colorize;
use std::collections::BTreeMap;
use std::env;

pub fn expand_tilde(p: &String) -> String {
    let mut new = p.clone();

    if new.starts_with("~") {
        new = new.replacen("~", &env::var("HOME").unwrap_or(String::new()), 1);
    }

    new
}

pub fn list_files(config: Config, check: bool) {
    println!("{}", "Listed configuration files:\n".purple());

    let vars = BTreeMap::new();
    let block_gen = BlockGenerator::new(
        String::new(),
        &vars,
        FileConfig::Single(BlockConfig::default()),
    );
    let mut updates = UpdatesGenerator::new(block_gen);

    config
        .files
        .into_iter()
        .for_each(|(name, config)| match config {
            FileConfig::Multi(multi) => {
                list_mutli(name, multi, &mut updates, check);
            }
            FileConfig::Single(single) => {
                updates.block_generator.config = single;
                println!("{}", list_block(name, &mut updates, check));
            }
        });
}

fn list_mutli(name: String, multi: TaggedConfig, updates: &mut UpdatesGenerator, check: bool) {
    println!("{} ({}) [Multiple blocks]: ", name.blue(), multi.path);

    multi.blocks.iter().for_each(|(tag, config)| {
        let config = BlockConfig {
            tag: Some(tag.to_string()),
            path: multi.path.clone(),
            comment: multi.comment.clone(),
            closing_comment: multi.comment_end.clone(),
            block: config.clone(),
        };

        updates.block_generator.config = config;
        let out = list_block(tag.clone(), updates, check);

        println!("  {out}");
    });
}

fn list_block(name: String, updates: &mut UpdatesGenerator, check: bool) -> String {
    // Do not show path if it's a tagged block
    let display_path = if updates.block_generator.config.tag.is_some() {
        String::new()
    } else {
        let mut dp = String::from("(");
        dp.push_str(&updates.block_generator.config.path);
        dp.push(')');

        dp
    };

    if check {
        let mut err: Option<&'static str> = None;

        if let Ok(c) = updates.read_file(&updates.block_generator.config.path) {
            if updates.validate_block(&c).is_err() {
                err = Some("No valid block found");
            }
        } else {
            err = Some("Failed to read file");
        }

        let mut status = "ok".green();
        let mut err_msg = String::new();

        if let Some(e) = err {
            status = "err".red();
            err_msg = format!("[{}]", e.to_string().red());
        }

        return format!("{status} {} {} {err_msg}", name, display_path);
    }
    format!("- {} {}", name, display_path)
}
