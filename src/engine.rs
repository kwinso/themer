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

        let mut new_block = get_updated_block(&theme_name, theme, &conf);
        new_block = wrap_with_themer_block(new_block, &&conf.comment);

        contents = themer_block_re
            .replacen(&contents, 1, new_block)
            .to_string();

        fs::write(&conf.path, contents.as_bytes()).unwrap();
    }
}

fn get_updated_block(theme_name: &String, theme: &ThemeVars, conf: &FileConfig) -> String {
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

        return custom.trim_end().to_owned();
    }
    format_vars(&theme, &conf)
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

    for (key, val) in vars.into_iter().filter(|x| !config.ignore.contains(x.0)) {
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

// TODO: Add test to check writing to files
#[cfg(test)]
mod tests {
    use super::{format_vars, get_updated_block, wrap_with_themer_block};
    use crate::config::Config;
    use std::fs;

    fn load_config(name: &'static str) -> Config {
        serde_yaml::from_str(&fs::read_to_string(format!("./test-configs/{name}.yml")).unwrap())
            .unwrap()
    }

    #[test]
    fn valid_themer_block() {
        let conf = load_config("basic");

        let res = format_vars(
            &conf.themes.get("dark").unwrap(),
            &conf.files.get("basic").unwrap(),
        );
        assert_eq!(
            res,
            "set my_background as \"#000000\"\nset my_foreground as \"#ffffff\""
        )
    }

    #[test]
    fn valid_custom_block() {
        let conf = load_config("custom");
        let theme = conf.themes.get("dark").unwrap();
        let file = conf.files.get("custom").unwrap();

        let res = get_updated_block(&"dark".to_owned(), &theme, &file);

        let vars = format_vars(&theme, &file);

        let expected = format!(
            r#"# This is just a comment
# This is colors for my theme dark:
{}
set foreground as "{}""#,
            vars,
            theme.get("foreground").unwrap()
        );

        assert_eq!(res, expected);
    }

    #[test]
    fn valid_wrapper() {
        let s = String::from("some string \n on newline");
        let res = wrap_with_themer_block(s.clone(), &String::from("#"));

        assert_eq!(res, format!("# THEMER\n{s}\n# THEMER_END"));
    }
}
