use crate::config::{Config, FileConfig, ThemeVars};
use colored::Colorize;
use regex::{Regex, RegexBuilder};
use std::{collections::HashSet, fs, process::exit};

pub fn update_configs(theme_name: String, config: Config) {
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

    for (_, conf) in config.files {
        let mut contents = match fs::read_to_string(&conf.path) {
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

        let mut new_block;

        if let Some(mut custom) = conf.custom.clone() {
            for var in get_custom_block_vars(&custom) {
                match var.as_str() {
                    "<colors>" => custom = custom.replace("<colors>", &format_vars(&theme, &conf)),
                    "<name>" => custom = custom.replace("<name>", &theme_name),
                    var => {
                        let plain_var = var.replace("<", "").replace(">", "");

                        if let Some(v) = theme.get(&plain_var) {
                            custom = custom.replace(var, v);
                        } else {
                            log::warn!(
                                "Custom block for file `{}`: variable {var} cannot be found.",
                                conf.path
                            );
                        }
                    }
                };
            }

            new_block = custom;
        } else {
            new_block = format_vars(&theme, &conf);
        }

        new_block = wrap_with_themer_block(new_block, &&conf.comment);

        contents = themer_block_re
            .replacen(&contents, 1, new_block)
            .to_string();

        fs::write(&conf.path, contents.as_bytes()).unwrap();
    }
}

/// Finds unique variables in contents block
fn get_custom_block_vars(contents: &String) -> Vec<String> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new("<.*>").unwrap();
    }

    RE.find_iter(contents)
        .map(|x| x.as_str().to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect()
}

pub fn get_block_re(comment: &String) -> Regex {
    RegexBuilder::new(&format!("{0} THEMER\n.*{0} THEMER_END", comment))
        .dot_matches_new_line(true)
        .build()
        .unwrap()
}

fn wrap_with_themer_block(contents: String, comment: &String) -> String {
    // Block is surrounded with newlines so no need to devide comment lines and the actual block
    // in this format! call
    format!("{0} THEMER\n{contents}\n{0} THEMER_END", comment)
}

fn format_vars(vars: &ThemeVars, config: &FileConfig) -> String {
    let mut block = String::new();

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

    block.trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::format_vars;
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

        let res = format_vars(&vars, &conf);
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
