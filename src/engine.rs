use crate::{
    config::{Config, FileConfig, ThemeVars},
    utils::expand_tilde,
};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::{
    collections::{hash_set::IntoIter, HashSet},
    fs,
    path::PathBuf,
    process::exit,
};

pub fn update_configs(theme_name: String, config: &Config) {
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

    for (_, conf) in &config.files {
        let mut contents = match fs::read_to_string(expand_tilde(&conf.path)) {
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

        let mut new_block = generate_contents(&theme_name, theme, &conf);
        // Replacing dollar sign to escape
        new_block = wrap_with_themer_block(new_block, &&conf.comment).replace("$", "$$");

        contents = themer_block_re
            .replacen(&contents, 1, new_block)
            .to_string();

        fs::write(expand_tilde(&conf.path), contents.as_bytes()).unwrap();
    }
}

fn generate_contents(theme_name: &String, theme: &ThemeVars, conf: &FileConfig) -> String {
    match &conf.custom {
        Some(custom) => expand_variables(custom.to_owned(), 0, &theme_name, theme, &conf),
        None => format_vars(&theme, &conf),
    }
}

fn expand_variables(
    mut contents: String,
    depth: u8,
    theme_name: &String,
    theme: &ThemeVars,
    conf: &FileConfig,
) -> String {
    // TODO: Move variable expanding inside it's own function
    for var in extract_vars(&contents) {
        match var.as_str() {
            "<vars>" => contents = contents.replace("<vars>", &format_vars(&theme, &conf)),
            "<name>" => contents = contents.replace("<name>", &theme_name),
            var => {
                let var_name = var.replace("<", "").replace(">", "");

                if let Some(v) = theme.get(&var_name) {
                    contents = contents.replace(var, v);
                } else {
                    log::warn!(
                        "Custom block for file `{}`: variable {var} cannot be found.",
                        conf.path
                    );
                }
            }
        };
    }

    for import in extract_imports(&contents) {
        if depth == 1 {
            log::error!("Maximum import depth exceeded (tried to import <{import}>)");
            println!(
                " {} Probably, you've tried to <import> a file from already imported file",
                "?".blue()
            );
            exit(1);
        }

        let path = match import.split(" ").nth(1) {
            Some(v) => {
                log::debug!("Importing {v:#?}");
                PathBuf::from(expand_tilde(&v.to_string()))
            }
            None => {
                log::error!("`{import}` is not valid.");
                log::info!("Import path should consist of import keyword and a path separated by whitespace.");
                continue;
            }
        };

        let import_contents = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to resolve import `{import}`: {e}");
                exit(1);
            }
        };

        let expanded = expand_variables(import_contents, depth + 1, theme_name, theme, conf);
        contents = contents.replace(&format!("<{import}>"), &expanded);
    }

    return contents.trim_end().to_owned();
}

/// Finds unique imports inside contents
fn extract_imports(contents: &String) -> Vec<String> {
    lazy_static! {
        // Matches only single word tokens: no variables inside variables
        static ref RE: Regex = Regex::new("<import .*>").unwrap();
    }

    find_with_re(contents, &RE)
        .map(|x| x.replace("<", "").replace(">", ""))
        .collect()
}

/// Finds unique variables inside contents
fn extract_vars(contents: &String) -> Vec<String> {
    lazy_static! {
        // Matches only single word tokens: no variables inside variables
        static ref RE: Regex = Regex::new("<\\S+[^<>]>").unwrap();
    }

    find_with_re(contents, &RE).collect()
}

// A generic function to retrive unique substrings from string with Regex
fn find_with_re(contents: &String, re: &Regex) -> IntoIter<String> {
    re.find_iter(contents)
        .map(|x| x.as_str().to_string())
        .collect::<HashSet<String>>()
        .into_iter()
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

// TODO: test ignoring variables
// TODO: test "only" variables
// TODO: test imports (with vars inside paths)
#[cfg(test)]
mod tests {
    use super::{format_vars, generate_contents, wrap_with_themer_block};
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

        let res = generate_contents(&"dark".to_owned(), &theme, &file);

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

    #[test]
    fn imports() {
        let conf = load_config("imports");

        let res = generate_contents(
            &"dark".to_owned(),
            &conf.themes.get("dark").unwrap(),
            &conf.files.get("imports").unwrap(),
        );

        assert_eq!(
            res,
            "# This is imported file for theme dark\nbackground = #000000\nforeground = #ffffff"
        )
    }
}
