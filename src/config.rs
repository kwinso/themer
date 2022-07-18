use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub path: String,
    #[serde(default = "default_comment")]
    pub comment: String,
    #[serde(default = "default_format")]
    pub format: String,
    pub custom: Option<String>,
}

fn default_comment() -> String {
    "#".to_owned()
}
fn default_format() -> String {
    "<key> = <value>".to_owned()
}

pub type ThemeVars = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub themes: HashMap<String, ThemeVars>,
    pub files: HashMap<String, FileConfig>,
}
