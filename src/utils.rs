use crate::block::BlockGenerator;
use crate::config::BlockConfig;
use crate::config::Config;
use crate::config::FileConfig;
use crate::config::TaggedConfig;
use crate::updates::UpdatesGenerator;
use colored::Colorize;
use std::env;
use std::fs;

pub fn expand_tilde(p: &String) -> String {
    let mut new = p.clone();

    if new.starts_with("~") {
        new = new.replacen("~", &env::var("HOME").unwrap_or(String::new()), 1);
    }

    new
}

pub fn list_files(config: Config, check: bool) {
    println!("{}", "Listed configuration files:".purple());

    config
        .files
        .into_iter()
        .for_each(|(name, config)| match config {
            FileConfig::Multi(multi) => {
                list_mutli(name, multi, check);
            }
            FileConfig::Single(single) => {
                println!("{}", list_block(name, single, None, check));
            }
        });
}

fn list_mutli(name: String, multi: TaggedConfig, check: bool) {
    println!("{} ({}) [Multiple blocks]: ", name.blue(), multi.path);

    multi.blocks.iter().for_each(|(tag, config)| {
        let config = BlockConfig {
            path: multi.path.clone(),
            comment: multi.comment.clone(),
            block: config.clone(),
        };
        let out = list_block(tag.clone(), config, Some(tag.clone()), check);
        println!("  {out}");
    });
}

fn list_block(name: String, config: BlockConfig, tag: Option<String>, check: bool) -> String {
    // Do not show path if it's a tagged block
    let display_path = if tag.is_some() {
        String::new()
    } else {
        let mut dp = String::from("(");
        dp.push_str(&config.path);
        dp.push(')');

        dp
    };

    if check {
        let valid: Result<(), &'static str> = match fs::read_to_string(expand_tilde(&config.path)) {
            Err(_) => Err("Failed to read file."),
            Ok(v) => {
                let re = UpdatesGenerator::get_block_re(
                    &config.comment,
                    &BlockGenerator::get_block_tags(&tag),
                );
                match re.is_match(&v) {
                    true => Ok(()),
                    false => Err("No THEMER block found."),
                }
            }
        };
        let mut status = "ok".green();
        let mut err = String::new();
        if let Err(e) = valid {
            status = "err".red();
            err = format!("[{}]", e.to_string().red());
        }

        return format!("{status} {} {} {err}", name, display_path);
    }
    format!("- {} {}", name, display_path)
}
