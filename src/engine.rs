use crate::config::{Config, FileConfig, ThemeVars};
use regex::{Regex, RegexBuilder};
use std::{fs, process::exit};

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
        let contents = match fs::read_to_string(&conf.path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Error reading `{}`:\n{e}", conf.path);
                exit(1);
            }
        };

        let themer_block_re = get_block_re(&conf.comment);
        if !themer_block_re.is_match(&contents) {
            log::error!("Failed to find THEMER block inside of `{}`", conf.path);
            exit(1);
        }

        let new_block = vars_to_themer_block(&theme, &conf);
        let contents = themer_block_re.replacen(&contents, 1, new_block);

        fs::write(&conf.path, contents.as_bytes()).unwrap();
    }
}

fn get_block_re(comment: &String) -> Regex {
    RegexBuilder::new(&format!("{0} THEMER\n.*{0} THEMER_END", comment))
        .dot_matches_new_line(true)
        .build()
        .unwrap()
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

#[cfg(test)]
mod tests {
    use super::vars_to_themer_block;
    use crate::config::FileConfig;
    use std::collections::HashMap;

    #[test]
    fn valid_themer_block() {
        let vars = HashMap::from([
            ("background".to_owned(), "#000000".to_owned()),
            ("foreground".to_owned(), "#ffffff".to_owned()),
        ]);
        let conf = FileConfig {
            path: String::new(),
            comment: "#".to_owned(),
            format: "set my_<key> as <value>".to_owned(),
            custom: None,
        };

        let res = vars_to_themer_block(&vars, &conf);
        assert_eq!(
            res,
            r#"# THEMER
set my_background as #000000
set my_foreground as #ffffff
# THEMER_END
"#
        )
    }
}
