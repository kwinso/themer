use crate::config::{Config, FileConfig, ThemeVars};
use std::process::exit;

pub fn update_configs(theme_name: String, config: Config) {
    let theme = match config.themes.get(&theme_name) {
        Some(t) => t,
        None => {
            log::error!("Theme `{theme_name}` is not listed in configuration file.");
            log::info!("Try to list avaliable themes with `themer list`");
            exit(1);
        }
    };
    for (name, conf) in config.files {
        // TODO:
        //if let Some(custom) = conf.custom {
        //}

        println!("{}", vars_to_themer_block(&theme, &conf));
        println!();
    }
}

fn vars_to_themer_block(vars: &ThemeVars, config: &FileConfig) -> String {
    let mut block = String::from("\n");

    for (key, val) in vars {
        block.push_str(
            &config
                .format
                .clone()
                .replace("<key>", key)
                .replace("<value>", val),
        );
        block.push('\n');
    }

    // Block is surrounded with newlines so no need to devide comment lines and the actual block
    // in this format! call
    format!("{0} THEMER{block}{0} THEMER_END", config.comment)
}
