use crate::{
    config::{FileConfig, ThemeVars},
    utils::expand_tilde,
};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{hash_set::IntoIter, HashSet},
    fs,
    path::PathBuf,
    process::exit,
};

pub struct BlockGenerator {
    theme_name: String,
    theme: ThemeVars,
    config: FileConfig,
}

impl BlockGenerator {
    pub fn new(theme_name: &String, theme: &ThemeVars, config: &FileConfig) -> Self {
        Self {
            theme: Self::apply_aliases(theme, &config.aliases),
            theme_name: theme_name.clone(),
            config: config.clone(),
        }
    }

    fn apply_aliases(theme: &ThemeVars, aliases: &Option<ThemeVars>) -> ThemeVars {
        let mut theme = theme.clone();

        if let Some(aliases) = aliases {
            for (new_key, old_key) in aliases {
                if theme.contains_key(old_key) {
                    // Remove old key and add new one to ThemeVars
                    let val = theme.get(old_key).unwrap().clone();
                    theme.remove(old_key);
                    theme.insert(new_key.to_owned(), val.to_owned());
                } else {
                    log::warn!("Failed to alias {new_key}: {old_key} does not exist");
                }
            }
        }
        theme
    }

    pub fn generate(&self) -> String {
        match &self.config.custom {
            Some(custom) => self.custom_block(custom.to_owned(), 0),
            None => self.default_block(),
        }
    }

    /// Wraps contents with appropriate comments that will identify Themer block
    pub fn wrap(&self, contents: &String) -> String {
        format!(
            "{0} THEMER\n{contents}\n{0} THEMER_END",
            &self.config.comment
        )
    }

    fn default_block(&self) -> String {
        let mut block = String::new();

        let mut filter_closure: Option<Box<dyn FnMut(&(String, String)) -> bool>> = None;

        // `only` has more "power" than `ignore`, so here we decide how to filter variables
        if !self.config.only.is_empty() {
            filter_closure = Some(Box::new(|x| self.config.only.contains(&x.0)));
        } else if !self.config.ignore.is_empty() {
            filter_closure = Some(Box::new(|x| !self.config.ignore.contains(&x.0)));
        }

        // Filters variables if needed, otherwise leaving everything as it was
        let vars = self
            .theme
            .clone()
            .into_iter()
            .filter(filter_closure.unwrap_or(Box::new(|_| true)));

        for (key, val) in vars {
            block.push_str(
                &self
                    .config
                    .format
                    .clone()
                    .replace("<key>", &key)
                    .replace("<value>", &val),
            );
            block.push('\n');
        }

        block.trim_end().to_owned()
    }

    pub fn custom_block(&self, mut input: String, depth: u8) -> String {
        input = self.expand_vars(input);
        input = self.resolve_imports(input, depth);

        return input.trim_end().to_owned();
    }

    /// Turns one-word variables into actual values
    fn expand_vars(&self, mut input: String) -> String {
        for var in Self::extract_vars(&input) {
            match var.as_str() {
                "<vars>" => input = input.replace("<vars>", &self.default_block()),
                "<name>" => input = input.replace("<name>", &self.theme_name),
                var => {
                    let var_name = var.replace("<", "").replace(">", "");

                    if let Some(v) = self.theme.get(&var_name) {
                        input = input.replace(var, v);
                    } else {
                        log::warn!(
                            "Custom block for file `{}`: variable {var} cannot be found.",
                            self.config.path
                        );
                    }
                }
            };
        }

        input
    }

    fn resolve_imports(&self, mut input: String, import_depth: u8) -> String {
        for import in Self::extract_imports(&input) {
            if import_depth == 1 {
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

            let expanded = self.custom_block(import_contents, import_depth + 1);
            input = input.replace(&format!("<{import}>"), &expanded);
        }

        input
    }

    /// A generic function to retrive unique substrings from string with Regex
    fn find_with_re(contents: &String, re: &Regex) -> IntoIter<String> {
        re.find_iter(contents)
            .map(|x| x.as_str().to_string())
            .collect::<HashSet<String>>()
            .into_iter()
    }
    /// Finds unique variables inside contents
    fn extract_vars(contents: &String) -> Vec<String> {
        lazy_static! {
            // Matches only single word tokens: no variables inside variables
            static ref RE: Regex = Regex::new("<\\S+[^<>]>").unwrap();
        }

        Self::find_with_re(contents, &RE).collect()
    }

    /// Finds unique imports inside contents
    fn extract_imports(contents: &String) -> Vec<String> {
        lazy_static! {
            // Matches only single word tokens: no variables inside variables
            static ref RE: Regex = Regex::new("<import .*>").unwrap();
        }

        Self::find_with_re(contents, &RE)
            .map(|x| x.replace("<", "").replace(">", ""))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::BlockGenerator;
    use crate::config::{Config, FileConfig, ThemeVars};
    use std::fs;

    fn load_config(file: &'static str) -> (ThemeVars, FileConfig) {
        let conf: Config = serde_yaml::from_str(
            &fs::read_to_string(format!("./test-configs/config.yml")).unwrap(),
        )
        .unwrap();

        (
            conf.themes.get("theme").unwrap().to_owned(),
            conf.files.get(file).unwrap().to_owned(),
        )
    }

    #[test]
    fn valid_themer_block() {
        let (theme, conf) = load_config("basic");
        let gen = BlockGenerator::new(&"theme".to_string(), &theme, &conf);

        assert_eq!(
            gen.generate(),
            "set my_background as \"#000000\"\nset my_foreground as \"#ffffff\""
        )
    }

    #[test]
    fn valid_custom_block() {
        let (theme, conf) = load_config("custom");

        let res = BlockGenerator::new(&"theme".to_string(), &theme, &conf).generate();

        let expected = format!(
            r#"# This is just a comment
# This is colors for my theme theme:
set background as #000000
set foreground as #ffffff
set foreground as {}"#,
            theme.get("foreground").unwrap()
        );

        assert_eq!(res, expected);
    }

    #[test]
    fn valid_wrapper() {
        let (theme, conf) = load_config("custom");

        let gen = BlockGenerator::new(&"theme".to_string(), &theme, &conf);
        let s = String::from("some string \n on newline");
        let res = gen.wrap(&s);

        assert_eq!(res, format!("# THEMER\n{s}\n# THEMER_END"));
    }

    #[test]
    fn imports() {
        let (theme, conf) = load_config("imports");

        let res = BlockGenerator::new(&"theme".to_string(), &theme, &conf).generate();

        assert_eq!(
            res,
            "# This is imported file for theme theme\nbackground = #000000\nforeground = #ffffff"
        )
    }

    #[test]
    fn ignore() {
        let (theme, conf) = load_config("ignore");
        let res = BlockGenerator::new(&"theme".to_string(), &theme, &conf).generate();

        assert_eq!(res, "background = #000000");
    }

    #[test]
    fn only() {
        let (theme, conf) = load_config("only");
        let res = BlockGenerator::new(&"theme".to_string(), &theme, &conf).generate();

        assert_eq!(res, "foreground = #ffffff");
    }

    #[test]
    fn aliases() {
        let (theme, conf) = load_config("aliases");
        let res = BlockGenerator::new(&"theme".to_string(), &theme, &conf).generate();

        assert_eq!(res, "bg = #000000\nfg = #ffffff");
    }
}
